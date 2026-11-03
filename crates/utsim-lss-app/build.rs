//! On Windows, embeds the icon and the product name in the executable.

fn main() {
    println!("cargo:rerun-if-changed=../../assets/icone/utsim.ico");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/icone/utsim.ico");
        res.set("ProductName", "UTSim Seis Sigma");
        res.set(
            "FileDescription",
            "UTSim Seis Sigma: gerenciador de projetos Lean Seis Sigma da família UTSim",
        );
        res.set("LegalCopyright", "© 2026 Jandrei");
        if let Err(e) = res.compile() {
            println!("cargo:warning=não foi possível embutir o ícone: {e}");
        }
    }
}
