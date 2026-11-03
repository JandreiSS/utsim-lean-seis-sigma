//! `utsim-seis-sigma [pasta.lss]`: opens the window; with a folder, opens that project.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::process::ExitCode;

use eframe::egui;
use utsim_lss_app::App;
use utsim_lss_app::text::texts;
use utsim_lss_store::user::AppDirs;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--versao" | "--version" | "-V") => {
            println!("utsim-seis-sigma {}", env!("CARGO_PKG_VERSION"));
            return ExitCode::SUCCESS;
        }
        Some("--ajuda" | "--help" | "-h") => {
            println!(
                "utsim-seis-sigma [pasta.lss]\n  sem argumentos: abre a janela\n  pasta.lss: abre esse projeto"
            );
            return ExitCode::SUCCESS;
        }
        _ => {}
    }
    let Some(dirs) = AppDirs::standard() else {
        eprintln!("erro: não encontrei a pasta do usuário");
        return ExitCode::FAILURE;
    };
    let open = args.first().map(PathBuf::from);
    let name = texts().app_name;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(name)
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([760.0, 480.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        name,
        options,
        Box::new(move |cc| {
            cc.egui_ctx
                .set_pixels_per_point(cc.egui_ctx.pixels_per_point().max(1.1));
            let mut app = App::new(dirs);
            if let Some(p) = open {
                app.open_project(&p);
            }
            Ok(Box::new(app))
        }),
    );
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erro na janela: {e}");
            ExitCode::FAILURE
        }
    }
}
