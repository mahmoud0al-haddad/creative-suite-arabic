//! Offscreen verification of Arabic/English UI; optional open font for published screenshots.
use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services, i18n};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::var("ARABIC_SNAPSHOT_DIR")?;
    let mut app = CadApp::new(Session::new(), Services::default());
    app.ui.in_window_menu = true;
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).with_max_steps(64).wgpu().build_ui_state(
        |ui, app| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    cadcraft_ui_egui::theme::install_fonts(&h.ctx, "ar");
    // Let the app finish its one-time styling before overriding runtime system fonts.
    // Published screenshots may use an explicitly supplied openly licensed face only.
    h.run_steps(1);
    if let Ok(path) = std::env::var("PUBLIC_SCREENSHOT_FONT") {
        let bytes = std::fs::read(path)?;
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("public-open-font".into(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
        for stack in fonts.families.values_mut() {
            stack.insert(0, "public-open-font".into());
        }
        h.ctx.set_fonts(fonts);
    }
    for (code, preference) in [("ar", i18n::Preference::Arabic), ("en", i18n::Preference::English)] {
        h.state_mut().ui.interface_language = preference;
        h.run_steps(5);
        h.render().map_err(std::io::Error::other)?.save(format!("{out}/cadcraft-{code}.png"))?;
    }
    Ok(())
}
