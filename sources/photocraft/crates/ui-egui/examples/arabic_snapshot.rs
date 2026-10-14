//! Offscreen Arabic/English shell verification without touching a user's settings or files.
use photocraft_ui_egui::{PhotocraftApp, Services};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::var("ARABIC_SNAPSHOT_DIR")?;
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).with_max_steps(64).wgpu().build_eframe(|cc| {
        PhotocraftApp::setup_context(&cc.egui_ctx, Default::default());
        PhotocraftApp::new(photocraft_engine::Session::new(), Services::default())
    });
    for code in ["ar", "en"] {
        h.state_mut().run("prefs.set", json!({"path": "interface.language", "value": code}))?;
        h.run_steps(5);
        h.render().map_err(std::io::Error::other)?.save(format!("{out}/photocraft-{code}.png"))?;
    }
    Ok(())
}
