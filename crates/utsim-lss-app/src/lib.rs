//! UTSim LSS, the Lean Six Sigma project manager of the UTSim family
//! (`docs/plano_lean_seis_sigma.md`): a native window (eframe/egui, the same egui as the
//! launcher), with no server but the optional read-only gemba page ([`rede`]).
//!
//! - [`session`]: an open project without interface (edits, refresh, presence);
//! - [`app`]: the screens (first run, start, project navigation, overview, charter, action
//!   board, conflicts);
//! - [`text`]: every text of the interface, in one catalogue;
//! - [`theme`]: dark and light look.

pub mod app;
/// The gemba page: the open actions as a read-only web page (P7).
pub mod rede;
/// The documents of the reports (stage L5).
pub mod reports;
pub mod session;
pub mod text;
pub mod theme;
/// Pedidos de outros apps da família (`--utsim-pedido`): a oportunidade do UTSim Ambiental.
pub mod troca;

pub use app::App;
