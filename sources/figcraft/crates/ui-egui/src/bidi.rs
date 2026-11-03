//! Logical Unicode names, elision and editing through the shared Arabic renderer.
//! Document text is never replaced by Arabic presentation forms or reversed words.
use egui::text::{LayoutJob, TextWrapping};
use egui::{Color32, FontId, Galley, Id, Painter, Pos2, Rect, Response, Ui, pos2, vec2};
use std::sync::Arc;

pub fn name_galley(painter: &Painter, name: &str, font: FontId, color: Color32, max_w: f32) -> Arc<Galley> {
    let mut job = LayoutJob::simple_singleline(name.to_owned(), font, color);
    job.wrap = TextWrapping::truncate_at_width(max_w.max(1.0));
    painter.layout_job(job)
}

pub fn paint_name(painter: &Painter, left_center: Pos2, name: &str, font: FontId, color: Color32, max_w: f32) -> Rect {
    let galley = name_galley(painter, name, font, color, max_w);
    let rect = Rect::from_min_size(pos2(left_center.x, left_center.y - galley.size().y / 2.0), galley.size());
    painter.galley(rect.min, galley, color);
    rect
}

pub fn single_line(ui: &mut Ui, id: Id, text: &mut String, font: FontId, width: f32) -> Response {
    let height = ui.available_height().max(0.0);
    egui::TextEdit::singleline(text)
        .id(id)
        .frame(egui::Frame::NONE)
        .font(font)
        .desired_width(width)
        .vertical_align(egui::Align::Center)
        .min_size(vec2(0.0, height))
        .show(ui)
        .response
        .response
}

/// Faces the UI adds for scripts its own fonts lack (desktop: from the system), as egui font data.
pub fn script_fallbacks(have: &dyn Fn(char) -> bool) -> Vec<(String, egui::FontData)> {
    // Hebrew, Arabic letters, Arabic presentation forms (what joined names are drawn with).
    const SAMPLES: [char; 3] = ['\u{05D0}', '\u{0628}', '\u{FE91}'];
    const FAMILIES: [&str; 11] = [
        "Segoe UI",
        "Arial",
        "Tahoma",
        "Noto Sans Hebrew",
        "Noto Sans Arabic",
        "Arial Hebrew",
        "Geeza Pro",
        "Lucida Grande",
        "DejaVu Sans",
        "FreeSans",
        "Noto Sans",
    ];
    let mut out: Vec<(String, egui::FontData)> = Vec::new();
    let mut taken: Vec<Arc<figcraft_text::Face>> = Vec::new();
    for c in SAMPLES {
        if have(c) || taken.iter().any(|f| f.has_char(c)) {
            continue;
        }
        let Some(face) = figcraft_text::face_with_char(c, &FAMILIES) else { continue };
        out.push((format!("system-{}-{}", face.family, face.style), crate::theme::font_data(&face)));
        taken.push(face);
    }
    out
}

#[cfg(test)]
#[path = "rtl_names_tests.rs"]
mod tests;
