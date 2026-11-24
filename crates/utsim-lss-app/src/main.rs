//! `utsim-seis-sigma [pasta.lss]`: opens the window; with a folder, opens that project.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::process::ExitCode;

use utsim_lss_app::App;
use utsim_lss_app::text::texts;
use utsim_lss_store::user::AppDirs;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(c) = utsim_ui_tema::janela::versao_ou_ajuda(
        args.first().map(String::as_str),
        "utsim-seis-sigma",
        env!("CARGO_PKG_VERSION"),
        "utsim-seis-sigma [pasta.lss]\n  sem argumentos: abre a janela\n  pasta.lss: abre esse projeto",
    ) {
        return c;
    }
    let Some(dirs) = AppDirs::standard() else {
        eprintln!("erro: não encontrei a pasta do usuário");
        return ExitCode::FAILURE;
    };
    let open = args.first().map(PathBuf::from);
    let name = texts().app_name;
    let options = eframe::NativeOptions {
        viewport: utsim_ui_tema::janela::viewport(name, [1180.0, 760.0], [760.0, 480.0], None),
        ..Default::default()
    };
    let result = eframe::run_native(
        name,
        options,
        Box::new(move |cc| {
            cc.egui_ctx
                .set_pixels_per_point(cc.egui_ctx.pixels_per_point().max(1.1));
            // The family's fonts and style (egui's fonts stay as the fallback, for the
            // symbols ● △ ◆ of the drawn tools) and the app's green.
            utsim_ui_tema::install(&cc.egui_ctx);
            utsim_lss_app::theme::apply(&cc.egui_ctx);
            let mut app = App::new(dirs);
            if let Some(p) = open {
                app.open_project(&p);
            }
            Ok(Box::new(app))
        }),
    );
    utsim_ui_tema::janela::saida(result)
}
