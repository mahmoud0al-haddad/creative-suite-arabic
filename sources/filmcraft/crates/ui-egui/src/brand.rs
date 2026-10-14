//! Plain application name for this personal modified build; official ArtCraft artwork is omitted.
/// Draw an ordinary text title and return its rectangle.
pub fn paint_wordmark(ui: &egui::Ui, left_center: egui::Pos2, height: f32, dark: bool) -> Option<egui::Rect> {
    let color = if dark { egui::Color32::WHITE } else { egui::Color32::BLACK };
    Some(ui.painter().text(left_center, egui::Align2::LEFT_CENTER, "FilmCraft", egui::FontId::proportional(height), color))
}
