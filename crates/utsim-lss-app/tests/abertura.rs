//! The opening screens drawn without a window: who you are (first run) and the start screen
//! with its actions. With `UTSIM_CAPTURAS=<pasta>` the PNGs are saved there for review.

use eframe::egui::{Context, vec2};
use utsim_lss_app::App;
use utsim_lss_app::app::Screen;
use utsim_lss_store::SystemClock;
use utsim_lss_store::user::AppDirs;
use utsim_ui_tema::captura::{Captura, Imagem};

fn conferir(img: &Imagem, nome: &str) {
    let mut cores = std::collections::BTreeSet::new();
    for y in (0..img.height).step_by(7) {
        for x in (0..img.width).step_by(7) {
            cores.insert(img.pixel(x, y));
        }
    }
    assert!(cores.len() > 12, "{nome}: a tela quase não pintou nada");
    if let Some(d) = std::env::var_os("UTSIM_CAPTURAS") {
        let d = std::path::PathBuf::from(d);
        std::fs::create_dir_all(&d).unwrap();
        img.salvar_png(&d.join(format!("{nome}.png"))).unwrap();
    }
}

#[test]
fn abertura_picture() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = App::with_clock(
        AppDirs::under(dir.path()),
        Box::new(|| Box::new(SystemClock)),
    );
    let mut cap = Captura::new(Context::default(), vec2(1440.0, 900.0));
    assert_eq!(app.screen, Screen::Identity);
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "seis_sigma_identidade");
    assert!(app.submit_identity("Ana Lima", "ana@empresa.com.br"));
    assert_eq!(app.screen, Screen::Start);
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "seis_sigma_abertura");
}
