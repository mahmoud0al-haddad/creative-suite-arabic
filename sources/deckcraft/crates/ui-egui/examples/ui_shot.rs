//! Offscreen language visual check; never opens or takes focus from a desktop window.
use deckcraft_ui_egui::{Services, SlideApp};
use std::sync::atomic::{AtomicBool, Ordering};
static READY: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).map(String::as_str).unwrap_or("deckcraft.png");
    let code = args.get(2).map(String::as_str).unwrap_or("ar");
    let mut session = deckcraft_engine::Session::new();
    deckcraft_engine::sample::open_sample(&mut session)?;
    let mut app = SlideApp::new(session, Services::default());
    app.run("app.language.set", serde_json::json!({"language": code}))?;
    if let Some(dialog) = args.get(3) {
        app.dialog = Some(deckcraft_ui_egui::dialogs::Dialog::new(dialog));
    }
    if let Some(tab) = args.get(4) {
        app.ui.tab = tab.clone();
    }
    let mut harness = egui_kittest::Harness::builder()
        .with_size(egui::vec2(1600.0, 1000.0))
        .with_pixels_per_point(2.0)
        .with_max_steps(1_000_000)
        .wgpu()
        .build_ui_state(
            |ui, app: &mut SlideApp| {
                if READY.load(Ordering::Relaxed) {
                    app.logic(ui.ctx());
                    app.ui(ui);
                }
            },
            app,
        );
    harness.input_mut().max_texture_side = Some(8192);
    READY.store(true, Ordering::Relaxed);
    for _ in 0..8 {
        harness.step();
    }
    harness.render()?.save(out)?;
    println!("wrote {out}");
    Ok(())
}
