//! What the stage L7 screens (training and certification, `docs/plano_lean_seis_sigma.md`,
//! 8.5) share: the training data read from the program journal, the restricted folders
//! opened on demand (question bank, grades, class folders) and the state kept between
//! frames.

use std::collections::BTreeMap;

use utsim_lss_core::certificacao::{self, Certification};
use utsim_lss_core::formacao::{
    self, Class, ClassSession, Criterion, Instructor, Nomination, Wave, curriculum, exams,
};
use utsim_lss_core::perfis::TrainingWriters;
use utsim_lss_core::programa::BeltLevel;
use utsim_lss_core::{Edit, Id, State};
use utsim_lss_store::program::{self, TrainingFolder};

use super::Data;
use crate::app::App;
use crate::session::Session;

/// The training data of the program journal.
#[derive(Debug, Clone, Default)]
pub struct Training {
    pub waves: Vec<Wave>,
    pub classes: Vec<Class>,
    pub sessions: Vec<ClassSession>,
    pub noms: Vec<Nomination>,
    pub criteria: Vec<Criterion>,
    pub modules: Vec<curriculum::Module>,
    pub instructors: Vec<Instructor>,
    pub exams: Vec<exams::Exam>,
    pub certs: Vec<Certification>,
}

impl Training {
    pub fn read(st: &State, program: Id) -> Training {
        Training {
            waves: formacao::waves(st, program),
            classes: formacao::classes(st, program),
            sessions: formacao::all_sessions(st),
            noms: formacao::nominations(st, program),
            criteria: formacao::criteria(st, program),
            modules: curriculum::modules(st, program),
            instructors: formacao::instructors(st, program),
            exams: exams::exams(st),
            certs: certificacao::certifications(st, program),
        }
    }

    pub fn class(&self, id: Option<Id>) -> Option<&Class> {
        id.and_then(|c| self.classes.iter().find(|k| k.id == c))
    }

    pub fn wave_name(&self, id: Option<Id>) -> String {
        id.and_then(|w| self.waves.iter().find(|x| x.id == w))
            .map_or_else(String::new, |w| w.name.clone())
    }

    pub fn module_name(&self, id: Id) -> String {
        self.modules
            .iter()
            .find(|m| m.id == id)
            .map_or_else(|| id.short(), |m| m.name.clone())
    }

    /// The instructors' people.
    pub fn instructor_people(&self) -> Vec<Id> {
        let mut v: Vec<Id> = self.instructors.iter().filter_map(|i| i.person).collect();
        v.sort();
        v.dedup();
        v
    }
}

/// The training screens' state between frames.
#[derive(Default)]
pub struct L7Ui {
    pub(crate) class: Option<Id>,
    pub(crate) wave: Option<Id>,
    pub(crate) level: BeltLevel,
    pub(crate) person: Option<Id>,
    pub(crate) manager: Option<Id>,
    pub(crate) opportunity: Option<Id>,
    pub(crate) criterion_kind: formacao::CriterionKind,
    pub(crate) criterion_elim: bool,
    pub(crate) suggestion: Option<formacao::teams::Suggestion>,
    pub(crate) session_modules: Vec<Id>,
    pub(crate) instructor_modules: Vec<Id>,
    pub(crate) question: exams::QuestionInput,
    pub(crate) recipe: usize,
    pub(crate) exam_modules: Vec<Id>,
    pub(crate) exam_counts: [u32; 3],
    pub(crate) exam_mode: exams::Mode,
    pub(crate) exam_retake: Option<Id>,
    pub(crate) cert_level: BeltLevel,
    pub(crate) cert_person: Option<Id>,
    /// The trainee's answers being typed, by question.
    pub(crate) answers: BTreeMap<Id, String>,
    pub(crate) exam_mode_on: Option<Id>,
    /// Restricted folders opened (bank, grades, classes).
    pub(crate) folders: BTreeMap<TrainingFolder, Session>,
    /// The reaction evaluation being filled, by class: scores (0 = blank) and comment.
    pub(crate) reaction: BTreeMap<Id, ([u8; 5], String)>,
}

impl L7Ui {
    pub fn close(&mut self) {
        for (_, mut s) in std::mem::take(&mut self.folders) {
            s.close();
        }
        *self = L7Ui::default();
    }
}

impl App {
    /// The writers of a training folder: the MBBs and the instructors; in a class folder,
    /// also its trainees.
    pub(in crate::app) fn training_writers(d: &Data, folder: TrainingFolder) -> TrainingWriters {
        let mut staff = d.mbbs.clone();
        staff.extend(d.tr.instructor_people());
        let students = match folder {
            TrainingFolder::Class(c) => {
                if let Some(k) = d.tr.class(Some(c)) {
                    staff.extend(k.instructors.iter().copied());
                }
                formacao::students(&d.tr.noms, c)
            }
            _ => Vec::new(),
        };
        staff.sort();
        staff.dedup();
        TrainingWriters { staff, students }
    }

    /// Opens a training folder if it exists (or creates it, for the staff, with `create`).
    fn training_folder(
        &mut self,
        s: &Session,
        d: &Data,
        folder: TrainingFolder,
        create: bool,
    ) -> Option<Session> {
        let path = program::training_path(s.folder().root(), folder);
        if !create && !path.join(utsim_lss_store::project::MANIFEST).is_file() {
            return None;
        }
        let me = s.folder().me().clone();
        let w = Self::training_writers(d, folder);
        let r = Session::open_training(&path, &me, (self.clock)(), folder, d.program, w, create);
        self.report(r)
    }

    /// The state of a training folder (opened once and refreshed), or `None` when it does
    /// not exist or cannot be read.
    pub(in crate::app) fn training_state(
        &mut self,
        s: &Session,
        d: &Data,
        folder: TrainingFolder,
    ) -> Option<State> {
        if !self.l6.l7.folders.contains_key(&folder) {
            let opened = self.training_folder(s, d, folder, false)?;
            self.l6.l7.folders.insert(folder, opened);
        }
        let f = self.l6.l7.folders.get_mut(&folder)?;
        let r = f.tick("formacao", None);
        let st = f.state().clone();
        self.report(r);
        Some(st)
    }

    /// Writes into a training folder (created on the first write by the staff).
    pub(in crate::app) fn write_training(
        &mut self,
        s: &Session,
        d: &Data,
        folder: TrainingFolder,
        edits: Vec<Edit>,
    ) -> bool {
        if edits.is_empty() {
            return true;
        }
        if !self.l6.l7.folders.contains_key(&folder) {
            match self.training_folder(s, d, folder, true) {
                Some(f) => {
                    self.l6.l7.folders.insert(folder, f);
                }
                None => return false,
            }
        }
        let Some(f) = self.l6.l7.folders.get_mut(&folder) else {
            return false;
        };
        let r = f.apply(edits);
        self.report(r).is_some()
    }
}
