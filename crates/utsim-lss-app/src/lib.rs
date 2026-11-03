//! UTSim Seis Sigma, the Lean Six Sigma project manager of the UTSim family
//! (`docs/plano_lean_seis_sigma.md`): a native window (eframe/egui, the same egui as the
//! launcher), no server and no browser.
//!
//! - [`session`]: an open project without interface (edits, refresh, presence);
//! - [`app`]: the screens (first run, start, project navigation, overview, charter, action
//!   board, conflicts);
//! - [`text`]: every text of the interface, in one catalogue;
//! - [`theme`]: dark and light look.

pub mod app;
pub mod session;
pub mod text;
pub mod theme;

pub use app::App;
