//! An open project, without any interface: the screens call these methods and only draw.
//! Every change goes edit → operation file → journal → state, the same path the other
//! people's changes take when they arrive through the sync.

use std::path::{Path, PathBuf};

use serde_json::Value;
use utsim_lss_core::domain::{Approach, Column, fields};
use utsim_lss_core::op::FIELD_DELETED;
use utsim_lss_core::presence::{self, HEARTBEAT_MS, Presence};
use utsim_lss_core::tools::{self, Deliverable, Tool, ap::CsvError, fmea, plans};
use utsim_lss_core::{Conflict, Date, Edit, EntityRef, Id, State, kanban, perfis};
use utsim_lss_store::project::ProjectFolder;
use utsim_lss_store::{Clock, Identity, Refresh, lssz};

/// How often the folder is looked at for other people's changes, milliseconds.
pub const REFRESH_MS: i64 = 2_000;

pub struct Session {
    folder: ProjectFolder,
    clock: Box<dyn Clock>,
    last_refresh_ms: i64,
    last_beat_ms: i64,
    beat: Option<(String, Option<EntityRef>)>,
    others: Vec<Presence>,
    /// Instant of the last change picked up from someone else.
    pub last_incoming_ms: Option<i64>,
    /// Who may write: nobody checked (a project) or the program's profiles.
    guard: Guard,
    /// The state built only from authorized operations (program and coaching logs).
    authorized: Option<perfis::Authorized>,
}

/// The permission rules of a folder (`docs/plano_lean_seis_sigma.md`, 8.8).
#[derive(Debug, Clone, PartialEq)]
pub enum Guard {
    /// Project and portfolio folders: the rules of their own screens.
    Free,
    /// The program folder: the profiles of its role list; the creator is an MBB.
    Program { founder: Id },
    /// A belt's coaching log: only the belt and the program's MBBs.
    Coaching { belt: Id, mbbs: Vec<Id> },
    /// A restricted training folder (question bank, grades, class): the staff, and in a
    /// class folder each trainee's own answers.
    Training(perfis::TrainingWriters),
}

impl Session {
    pub fn open(
        path: &Path,
        me: &Identity,
        cache: Option<&Path>,
        clock: Box<dyn Clock>,
    ) -> utsim_lss_store::Result<Self> {
        let mut folder = ProjectFolder::open(path, me, cache)?;
        match folder.main_kind() {
            Some(utsim_lss_core::Kind::Program) => return Self::open_program(path, me, clock),
            Some(utsim_lss_core::Kind::CoachingLog | utsim_lss_core::Kind::TrainingFolder) => {
                return Err(utsim_lss_store::Error::Invalid(
                    crate::text::l6::l6().open_log_from_program.to_string(),
                ));
            }
            _ => {}
        }
        folder.register_me(clock.as_ref())?;
        Ok(Self::with(folder, clock))
    }

    /// Opens a program folder: every operation in memory, each checked against its
    /// author's profile at its instant.
    pub fn open_program(
        path: &Path,
        me: &Identity,
        clock: Box<dyn Clock>,
    ) -> utsim_lss_store::Result<Self> {
        let folder = ProjectFolder::open_full(path, me)?;
        let founder = folder.manifest().created_by;
        let mut s = Self::with(folder, clock);
        s.guard = Guard::Program { founder };
        s.reauthorize();
        // Registering oneself (name and e-mail) is free for everyone.
        let edits =
            utsim_lss_core::domain::Person::register(s.state(), me.person, &me.name, &me.email);
        s.apply(edits)?;
        Ok(s)
    }

    pub fn create_program(
        path: &Path,
        name: &str,
        me: &Identity,
        clock: Box<dyn Clock>,
    ) -> utsim_lss_store::Result<Self> {
        let folder = ProjectFolder::create_program(path, name, me, clock.as_ref())?;
        let founder = folder.manifest().created_by;
        let mut s = Self::with(folder, clock);
        s.guard = Guard::Program { founder };
        s.reauthorize();
        Ok(s)
    }

    /// Opens (or, with `create`, creates) the coaching log of `belt` under the program.
    pub fn open_coaching(
        path: &Path,
        me: &Identity,
        clock: Box<dyn Clock>,
        belt: Id,
        program: Id,
        mbbs: Vec<Id>,
        create: bool,
    ) -> utsim_lss_store::Result<Self> {
        // MB8: the log is seen only by the MBB and the belt.
        if !utsim_lss_core::mentoria::can_see(me.person, belt, mbbs.contains(&me.person)) {
            return Err(utsim_lss_store::Error::Invalid(
                crate::text::l6::l6().log_not_yours.to_string(),
            ));
        }
        let guard = Guard::Coaching { belt, mbbs };
        let folder = if create && !path.join(utsim_lss_store::project::MANIFEST).is_file() {
            ProjectFolder::create_coaching(path, belt, program, me, clock.as_ref())?
        } else {
            ProjectFolder::open_full(path, me)?
        };
        let mut s = Self::with(folder, clock);
        s.guard = guard;
        s.reauthorize();
        Ok(s)
    }

    /// Opens (or, with `create`, creates) a restricted training folder of the program.
    pub fn open_training(
        path: &Path,
        me: &Identity,
        clock: Box<dyn Clock>,
        folder: utsim_lss_store::program::TrainingFolder,
        program: Id,
        writers: perfis::TrainingWriters,
        create: bool,
    ) -> utsim_lss_store::Result<Self> {
        let f = if create && !path.join(utsim_lss_store::project::MANIFEST).is_file() {
            if !writers.staff.contains(&me.person) {
                return Err(utsim_lss_store::Error::Invalid(
                    crate::text::l7::l7().not_staff.to_string(),
                ));
            }
            ProjectFolder::create_training(path, folder, program, me, clock.as_ref())?
        } else {
            ProjectFolder::open_full(path, me)?
        };
        let mut s = Self::with(f, clock);
        s.guard = Guard::Training(writers);
        s.reauthorize();
        Ok(s)
    }

    /// The program's creator (`None` outside a program folder).
    pub fn founder(&self) -> Option<Id> {
        match self.guard {
            Guard::Program { founder } => Some(founder),
            _ => None,
        }
    }

    pub fn is_program(&self) -> bool {
        matches!(self.guard, Guard::Program { .. })
    }

    pub fn guard(&self) -> &Guard {
        &self.guard
    }

    /// Operations without effect because their author had no profile for them.
    pub fn unauthorized(&self) -> &[perfis::Unauthorized] {
        self.authorized
            .as_ref()
            .map_or(&[], |a| a.unauthorized.as_slice())
    }

    /// Rebuilds the authorized state from every operation of the folder.
    fn reauthorize(&mut self) {
        self.authorized = match &self.guard {
            Guard::Free => None,
            Guard::Program { founder } => Some(perfis::replay(self.folder.all_ops(), *founder)),
            Guard::Coaching { belt, mbbs } => {
                Some(perfis::replay_coaching(self.folder.all_ops(), *belt, mbbs))
            }
            Guard::Training(w) => Some(perfis::replay_training(self.folder.all_ops(), w)),
        };
    }

    /// Checks the profile for an action of the program (screens call it before acting,
    /// so the refusal has a clear message even for actions that write nothing here).
    pub fn may(&self, action: perfis::Action, unit: Option<Id>) -> Result<(), perfis::Denied> {
        match self.guard {
            Guard::Program { founder } => {
                perfis::check(self.state(), founder, self.me(), action, unit)
            }
            _ => Ok(()),
        }
    }

    /// Reopens the folder from the files (after the history was rewritten).
    pub fn reload(&mut self) -> utsim_lss_store::Result<()> {
        let me = self.folder.me().clone();
        self.folder = ProjectFolder::open_full(self.folder.root(), &me)?;
        self.reauthorize();
        Ok(())
    }

    pub fn create(
        path: &Path,
        title: &str,
        approach: Approach,
        me: &Identity,
        cache: Option<&Path>,
        clock: Box<dyn Clock>,
    ) -> utsim_lss_store::Result<Self> {
        let folder = ProjectFolder::create(path, title, approach, me, clock.as_ref(), cache)?;
        Ok(Self::with(folder, clock))
    }

    /// Creates a portfolio folder (its project folders go next to it).
    pub fn create_portfolio(
        path: &Path,
        names: (&str, &str, &str),
        me: &Identity,
        cache: Option<&Path>,
        clock: Box<dyn Clock>,
    ) -> utsim_lss_store::Result<Self> {
        let (name, unit, program) = names;
        let folder =
            ProjectFolder::create_portfolio(path, name, unit, program, me, clock.as_ref(), cache)?;
        Ok(Self::with(folder, clock))
    }

    fn with(folder: ProjectFolder, clock: Box<dyn Clock>) -> Self {
        let now = clock.now_ms();
        Self {
            folder,
            clock,
            last_refresh_ms: now,
            last_beat_ms: i64::MIN / 2,
            beat: None,
            others: Vec::new(),
            last_incoming_ms: None,
            guard: Guard::Free,
            authorized: None,
        }
    }

    pub fn folder(&self) -> &ProjectFolder {
        &self.folder
    }

    /// The state: of a program or coaching log, only the authorized operations.
    pub fn state(&self) -> &State {
        match &self.authorized {
            Some(a) => &a.state,
            None => self.folder.state(),
        }
    }

    pub fn project(&self) -> Id {
        self.folder.project_id()
    }

    pub fn me(&self) -> Id {
        self.folder.me().person
    }

    pub fn now_ms(&self) -> i64 {
        self.clock.now_ms()
    }

    /// Today, in the local time of this computer.
    pub fn today(&self) -> Date {
        Date::from_unix_ms(self.clock.now_ms(), self.clock.utc_offset_minutes())
    }

    /// An id generator for edits that create several entities.
    pub fn ids(&self) -> impl FnMut() -> Id + '_ {
        || self.clock.new_id()
    }

    /// Commits edits; returns how many operations were written. In a program folder (and a
    /// coaching log) the edits outside the person's profiles are refused before anything
    /// is written, with the message of [`perfis::Denied`].
    pub fn apply(&mut self, edits: Vec<Edit>) -> utsim_lss_store::Result<usize> {
        if edits.is_empty() {
            return Ok(0);
        }
        let me = self.me();
        let check = match &self.guard {
            Guard::Free => Ok(()),
            Guard::Program { founder } => perfis::check_edits(self.state(), *founder, me, &edits),
            Guard::Coaching { belt, mbbs } => perfis::check_coaching(me, *belt, mbbs, &edits),
            Guard::Training(w) => perfis::check_training(self.state(), me, w, &edits),
        };
        check.map_err(|d| utsim_lss_store::Error::Invalid(d.to_string()))?;
        let ops = self.folder.commit(edits, self.clock.as_ref())?;
        if self.authorized.is_some() {
            self.reauthorize();
        }
        Ok(ops.len())
    }

    /// Sets a field if the value changes. `seen` is the version shown when the person
    /// started editing (see [`Edit::seen`]).
    pub fn set_field(
        &mut self,
        entity: EntityRef,
        field: &str,
        value: Value,
        seen: Option<Option<Id>>,
    ) -> utsim_lss_store::Result<bool> {
        let current = self
            .state()
            .entity(entity.id)
            .and_then(|e| e.get(field))
            .cloned();
        let unchanged =
            current.as_ref() == Some(&value) || (current.is_none() && value.as_str() == Some(""));
        if unchanged {
            return Ok(false);
        }
        let mut edit = Edit::new(entity, field, value);
        edit.seen = seen;
        self.apply(vec![edit])?;
        Ok(true)
    }

    /// A new card at the end of a column; returns its id.
    pub fn add_card(&mut self, title: &str, column: Column) -> utsim_lss_store::Result<Option<Id>> {
        let title = title.trim();
        if title.is_empty() {
            return Ok(None);
        }
        let id = self.clock.new_id();
        let edits = kanban::new_card(self.state(), self.project(), id, title, column);
        self.apply(edits)?;
        Ok(Some(id))
    }

    /// Moves a card, respecting the column's WIP limit and the effectiveness check before
    /// "feito" (`utsim_lss_core::board::check_move`).
    pub fn move_card(
        &mut self,
        card: Id,
        column: Column,
        index: usize,
    ) -> utsim_lss_store::Result<()> {
        utsim_lss_core::board::check_move(self.state(), self.project(), card, column)
            .map_err(|e| utsim_lss_store::Error::Invalid(e.to_string()))?;
        let edits = kanban::move_card(self.state(), self.project(), card, column, index);
        self.apply(edits).map(|_| ())
    }

    /// Moves a card one column to the left (`-1`) or right (`+1`), to the end.
    pub fn shift_card(&mut self, card: Id, step: i32) -> utsim_lss_store::Result<()> {
        let Some(col) = self.card_column(card) else {
            return Ok(());
        };
        let i = Column::ALL.iter().position(|c| *c == col).unwrap_or(0) as i32 + step;
        if let Some(to) = usize::try_from(i).ok().and_then(|i| Column::ALL.get(i)) {
            self.move_card(card, *to, usize::MAX)?;
        }
        Ok(())
    }

    /// Moves a card one place up (`-1`) or down (`+1`) in its column.
    pub fn nudge_card(&mut self, card: Id, step: i32) -> utsim_lss_store::Result<()> {
        let board = kanban::board(self.state(), self.project());
        let Some((col, pos)) = board
            .iter()
            .find_map(|(c, v)| v.iter().position(|a| a.id == card).map(|p| (*c, p)))
        else {
            return Ok(());
        };
        let target = pos as i64 + i64::from(step);
        if target < 0 {
            return Ok(());
        }
        self.move_card(card, col, target as usize)
    }

    pub fn card_column(&self, card: Id) -> Option<Column> {
        self.state()
            .entity(card)
            .map(|e| e.typed(fields::action::COLUMN).unwrap_or_default())
    }

    /// A new id (UUID v7) from the session's clock.
    pub fn new_id(&self) -> Id {
        self.clock.new_id()
    }

    /// A new deliverable of `tool` in its usual phase; returns its id.
    pub fn add_deliverable(&mut self, tool: Tool, name: &str) -> utsim_lss_store::Result<Id> {
        let id = self.clock.new_id();
        let phase = tools::default_phase(self.state(), self.project(), tool);
        self.apply(Deliverable::create(self.project(), phase, tool, id, name))?;
        Ok(id)
    }

    /// A new row or node of a deliverable at the end of its siblings; returns its id.
    pub fn add_item(
        &mut self,
        deliverable: Id,
        item_kind: &str,
        parent: Option<Id>,
        extra: Vec<(&str, Value)>,
    ) -> utsim_lss_store::Result<Id> {
        let id = self.clock.new_id();
        let edits = tools::new_item(
            self.state(),
            self.project(),
            deliverable,
            item_kind,
            id,
            parent,
            extra,
        );
        self.apply(edits)?;
        Ok(id)
    }

    /// Deletes an item and the items below it.
    pub fn delete_item(&mut self, item: Id) -> utsim_lss_store::Result<()> {
        let edits = tools::delete_subtree(self.state(), item);
        self.apply(edits).map(|_| ())
    }

    /// Moves an item one place up (`-1`) or down (`+1`) among its siblings.
    pub fn nudge_item(&mut self, item: Id, step: i32) -> utsim_lss_store::Result<()> {
        let edits = tools::nudge_item(self.state(), item, step);
        self.apply(edits).map(|_| ())
    }

    /// Turns the recommended action of an FMEA row into a card; `None` when the row
    /// already has one.
    pub fn fmea_card(&mut self, row: Id) -> utsim_lss_store::Result<Option<Id>> {
        let card = self.clock.new_id();
        let edits = fmea::action_card(self.state(), self.project(), row, card);
        if edits.is_empty() {
            return Ok(None);
        }
        self.apply(edits)?;
        Ok(Some(card))
    }

    /// Opens a card for a signal of the response plan.
    pub fn signal_card(&mut self, signal: Id) -> utsim_lss_store::Result<Option<Id>> {
        let card = self.clock.new_id();
        let edits = plans::signal_card(self.state(), self.project(), signal, card);
        if edits.is_empty() {
            return Ok(None);
        }
        self.apply(edits)?;
        Ok(Some(card))
    }

    /// Imports the company's action priority table for the project. The outer error is
    /// the folder's; the inner one lists the problems of the CSV.
    pub fn import_ap_table(
        &mut self,
        csv: &str,
        name: &str,
    ) -> utsim_lss_store::Result<Result<(), Vec<CsvError>>> {
        match fmea::import_table(self.project(), csv, name) {
            Ok(edits) => self.apply(edits).map(|_| Ok(())),
            Err(e) => Ok(Err(e)),
        }
    }

    /// Back to the built-in action priority table.
    pub fn use_builtin_ap_table(&mut self) -> utsim_lss_store::Result<()> {
        self.apply(fmea::use_builtin_table(self.project()))
            .map(|_| ())
    }

    pub fn delete(&mut self, entity: EntityRef) -> utsim_lss_store::Result<()> {
        self.apply(vec![Edit::new(entity, FIELD_DELETED, true)])
            .map(|_| ())
    }

    pub fn keep(&mut self, conflict: &Conflict) -> utsim_lss_store::Result<()> {
        self.apply(vec![conflict.dismiss()]).map(|_| ())
    }

    pub fn restore(&mut self, conflict: &Conflict) -> utsim_lss_store::Result<()> {
        self.apply(conflict.restore()).map(|_| ())
    }

    /// Periodic work, called every frame: picks up other people's changes every
    /// [`REFRESH_MS`] and writes the heartbeat when the screen or the edited entity changes
    /// or every [`HEARTBEAT_MS`]. Returns the refresh report when there was one.
    pub fn tick(
        &mut self,
        screen: &str,
        editing: Option<EntityRef>,
    ) -> utsim_lss_store::Result<Option<Refresh>> {
        let now = self.clock.now_ms();
        let mut report = None;
        if now - self.last_refresh_ms >= REFRESH_MS {
            self.last_refresh_ms = now;
            let r = self.folder.refresh(self.clock.as_ref())?;
            if r.new_ops > 0 {
                self.last_incoming_ms = Some(now);
                if self.authorized.is_some() {
                    self.reauthorize();
                }
            }
            report = Some(r);
        }
        let key = (screen.to_string(), editing);
        if self.beat.as_ref() != Some(&key) || now - self.last_beat_ms >= HEARTBEAT_MS {
            self.last_beat_ms = now;
            self.folder
                .heartbeat(self.clock.as_ref(), screen, editing)?;
            self.beat = Some(key);
            self.others = self.folder.others(now);
        } else if report.is_some() {
            self.others = self.folder.others(now);
        }
        Ok(report)
    }

    /// Picks up other people's changes now.
    pub fn refresh_now(&mut self) -> utsim_lss_store::Result<Refresh> {
        self.last_refresh_ms = self.clock.now_ms();
        let r = self.folder.refresh(self.clock.as_ref())?;
        if r.new_ops > 0 && self.authorized.is_some() {
            self.reauthorize();
        }
        Ok(r)
    }

    /// The clock (to open other folders with the same one).
    pub fn clock(&self) -> &dyn Clock {
        self.clock.as_ref()
    }

    /// Other people with the project open.
    pub fn others(&self) -> &[Presence] {
        &self.others
    }

    /// Others editing this entity now.
    pub fn editing(&self, entity: EntityRef) -> Vec<&Presence> {
        presence::editing(&self.others, entity).collect()
    }

    /// Default destination of the `.lssz` export: next to the folder.
    pub fn default_export_path(&self) -> PathBuf {
        let root = self.folder.root();
        let stem = root
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "projeto".into());
        root.with_file_name(format!("{stem}.{}", lssz::EXTENSION))
    }

    pub fn export(&self, dest: &Path) -> utsim_lss_store::Result<()> {
        lssz::export(self.folder.root(), dest)
    }

    /// Closing: presence removed, cache saved.
    pub fn close(&mut self) {
        self.folder.leave();
        let _ = self.folder.save_cache();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
    use utsim_lss_core::Kind;

    /// A clock the test moves by hand.
    #[derive(Clone)]
    pub(crate) struct ManualClock {
        pub now: Arc<AtomicI64>,
        n: Arc<AtomicU64>,
        salt: u8,
    }

    impl ManualClock {
        pub fn new(salt: u8) -> Self {
            Self {
                now: Arc::new(AtomicI64::new(1_793_000_000_000)),
                n: Arc::new(AtomicU64::new(0)),
                salt,
            }
        }
        pub fn advance(&self, ms: i64) {
            self.now.fetch_add(ms, Ordering::Relaxed);
        }
    }

    impl Clock for ManualClock {
        fn now_ms(&self) -> i64 {
            self.now.load(Ordering::Relaxed)
        }
        fn random(&self) -> [u8; 10] {
            let n = self.n.fetch_add(1, Ordering::Relaxed);
            let mut r = [self.salt; 10];
            r[2..].copy_from_slice(&n.to_be_bytes());
            r
        }
    }

    fn me(n: u8, name: &str) -> Identity {
        Identity {
            person: Id::v7(1_700_000_000_000, [n; 10]),
            name: name.into(),
            email: format!("{}@x.com", name.to_lowercase()),
        }
    }

    #[test]
    fn board_edits_go_through_files_and_back() {
        let dir = tempfile::tempdir().unwrap();
        let ca = ManualClock::new(1);
        let cb = ManualClock::new(2);
        let mut a = Session::create(
            &dir.path().join("p"),
            "P",
            Approach::Dmaic,
            &me(1, "Ana"),
            None,
            Box::new(ca.clone()),
        )
        .unwrap();
        let mut b =
            Session::open(a.folder().root(), &me(2, "Bia"), None, Box::new(cb.clone())).unwrap();
        let card = a
            .add_card("Calibrar o paquímetro", Column::Todo)
            .unwrap()
            .unwrap();
        assert!(a.add_card("   ", Column::Todo).unwrap().is_none());
        a.shift_card(card, 1).unwrap();
        assert_eq!(a.card_column(card), Some(Column::Doing));
        a.shift_card(card, -5).unwrap();
        assert_eq!(
            a.card_column(card),
            Some(Column::Doing),
            "out of range: stays"
        );

        // Bia picks it up on her next tick, sees Ana on the board.
        ca.advance(10);
        a.tick("acoes", Some(EntityRef::new(Kind::Action, card)))
            .unwrap();
        cb.advance(REFRESH_MS);
        let r = b.tick("acoes", None).unwrap().unwrap();
        assert!(r.new_ops >= 6);
        assert_eq!(b.card_column(card), Some(Column::Doing));
        assert_eq!(b.others().len(), 1);
        assert_eq!(b.editing(EntityRef::new(Kind::Action, card)).len(), 1);

        // Bia edits the title from the version she saw while Ana changes it: conflict.
        let r = EntityRef::new(Kind::Action, card);
        let seen = b
            .state()
            .entity(card)
            .unwrap()
            .version(fields::action::TITLE);
        a.set_field(
            r,
            fields::action::TITLE,
            Value::from("Calibrar os paquímetros"),
            None,
        )
        .unwrap();
        b.refresh_now().unwrap();
        b.set_field(
            r,
            fields::action::TITLE,
            Value::from("Calibrar paquímetro e micrômetro"),
            Some(seen),
        )
        .unwrap();
        a.refresh_now().unwrap();
        assert_eq!(a.state(), b.state());
        assert_eq!(a.state().conflicts.len(), 1);
        let c = a.state().conflicts[0].clone();
        a.restore(&c).unwrap();
        b.refresh_now().unwrap();
        assert!(b.state().conflicts.is_empty());
        assert_eq!(
            b.state().entity(card).unwrap().text(fields::action::TITLE),
            "Calibrar os paquímetros"
        );
        // Unchanged value: nothing written.
        let before = b.folder().journal().len();
        assert!(
            !b.set_field(
                r,
                fields::action::TITLE,
                Value::from("Calibrar os paquímetros"),
                None
            )
            .unwrap()
        );
        assert_eq!(b.folder().journal().len(), before);

        b.delete(r).unwrap();
        assert!(
            kanban::board(b.state(), b.project())
                .values()
                .all(|v| v.is_empty())
        );
        b.close();
        a.close();
    }

    #[test]
    fn tools_through_the_folder() {
        use utsim_lss_core::tools::ap::{Ap, ApTable};
        let dir = tempfile::tempdir().unwrap();
        let c = ManualClock::new(1);
        let mut s = Session::create(
            &dir.path().join("F"),
            "P",
            Approach::Dmaic,
            &me(1, "Ana"),
            None,
            Box::new(c),
        )
        .unwrap();
        let d = s.add_deliverable(Tool::Fmea, "FMEA").unwrap();
        let row = s
            .add_item(
                d,
                fmea::ITEM,
                None,
                vec![
                    (fmea::fields::S, Value::from(9)),
                    (fmea::fields::O, Value::from(4)),
                    (fmea::fields::D, Value::from(2)),
                    (fmea::fields::ACTION, Value::from("Poka-yoke no gabarito")),
                ],
            )
            .unwrap();
        let card = s.fmea_card(row).unwrap().unwrap();
        assert!(s.fmea_card(row).unwrap().is_none());
        assert_eq!(s.card_column(card), Some(Column::Todo));
        let (t, _) = fmea::project_table(s.state(), s.project());
        assert_eq!(fmea::rows(s.state(), d)[0].ap(&t), Some(Ap::High));
        assert!(s.import_ap_table("x", "y").unwrap().is_err());
        let low = ApTable::from_compact(&"L".repeat(1000)).unwrap();
        s.import_ap_table(&low.to_csv(), "Empresa")
            .unwrap()
            .unwrap();
        let (t, _) = fmea::project_table(s.state(), s.project());
        assert_eq!(fmea::rows(s.state(), d)[0].ap(&t), Some(Ap::Low));
        s.use_builtin_ap_table().unwrap();
        let rp = s.add_deliverable(Tool::ResponsePlan, "Plano").unwrap();
        let sig = s
            .add_item(
                rp,
                plans::RESPONSE_ITEM,
                None,
                vec![(plans::fields::SIGNAL, Value::from("Alarme"))],
            )
            .unwrap();
        assert!(s.signal_card(sig).unwrap().is_some());
        let child = s
            .add_item(rp, plans::RESPONSE_ITEM, Some(sig), vec![])
            .unwrap();
        s.nudge_item(sig, 1).unwrap();
        s.delete_item(sig).unwrap();
        assert!(s.state().entity(child).unwrap().is_deleted());
        // Everything is in the folder: reopening gives the same state.
        let before = s.state().clone();
        let root = s.folder().root().to_path_buf();
        s.close();
        let again =
            Session::open(&root, &me(1, "Ana"), None, Box::new(ManualClock::new(1))).unwrap();
        assert_eq!(again.state().entities, before.entities);
    }

    #[test]
    fn nudge_and_export() {
        let dir = tempfile::tempdir().unwrap();
        let c = ManualClock::new(1);
        let mut s = Session::create(
            &dir.path().join("Linha"),
            "P",
            Approach::A3,
            &me(1, "Ana"),
            None,
            Box::new(c),
        )
        .unwrap();
        let x = s.add_card("x", Column::Todo).unwrap().unwrap();
        let y = s.add_card("y", Column::Todo).unwrap().unwrap();
        s.nudge_card(y, -1).unwrap();
        let titles: Vec<String> = kanban::board(s.state(), s.project())[&Column::Todo]
            .iter()
            .map(|a| a.title.clone())
            .collect();
        assert_eq!(titles, ["y", "x"]);
        s.nudge_card(y, -1).unwrap();
        s.nudge_card(x, 3).unwrap();
        let dest = s.default_export_path();
        assert!(dest.ends_with("Linha.lssz"));
        s.export(&dest).unwrap();
        assert!(dest.is_file());
    }
}
