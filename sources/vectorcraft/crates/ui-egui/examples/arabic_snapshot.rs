//! Offscreen verification of both interface languages in one live session.
use vectorcraft_engine::Session;
use vectorcraft_ui_egui::{Services, VectorcraftApp};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::var("ARABIC_SNAPSHOT_DIR")?;
    let app = VectorcraftApp::new(Session::new(), Services::default());
    let mut h =
        egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).with_max_steps(64).wgpu().build_ui_state(|ui, app| app.ui(ui), app);
    vectorcraft_ui_egui::theme::install_fonts(&h.ctx);
    for code in ["ar", "en"] {
        h.state_mut().session.prefs.interface_language = code.into();
        h.run_steps(5);
        if h.state().ui_language().code() != code {
            return Err("interface language did not switch".into());
        }
        h.render().map_err(std::io::Error::other)?.save(format!("{out}/vectorcraft-{code}.png"))?;
    }
    Ok(())
}
