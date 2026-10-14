//! Keep logical Unicode for widgets, accessibility and dynamic placeholders.
//! The local epaint renderer wraps logical text and applies bidi separately to each line.
use std::borrow::Cow;

pub fn visual(text: &str) -> Cow<'_, str> {
    Cow::Borrowed(text)
}

pub fn display_rtl(text: &str) -> String {
    text.to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn logical_text_and_user_values_survive_until_layout() {
        for value in ["تقرير نهائي 2026.pdf", "الصفحة {p} من {n}", "حفظ باسم…", "PDF (عربي) 12"] {
            assert_eq!(super::visual(value), value);
            assert_eq!(super::display_rtl(value), value);
        }
    }

    #[test]
    fn renderer_wraps_logical_arabic_and_keeps_latin_values() {
        let ctx = egui::Context::default();
        ctx.set_fonts(crate::theme::font_definitions());
        let mut output = ctx.run_ui(egui::RawInput::default(), |_| {});
        output.textures_delta.clear();
        ctx.fonts_mut(|fonts| {
            let source = "تم فتح ملف PDF 2026.pdf بنجاح";
            let galley = fonts.layout_no_wrap(source.to_owned(), egui::FontId::proportional(14.0), egui::Color32::WHITE);
            assert_eq!(galley.job.text, source);
            let visible: String = galley.rows[0].row.glyphs.iter().map(|glyph| glyph.chr).collect();
            assert!(visible.contains("PDF"), "{visible}");
            assert!(visible.contains("2026.pdf"), "{visible}");
            // Noto may emit several glyphs for one cluster. Glyph::chr is a
            // diagnostic label, so compare logical caret geometry, not letters
            // reconstructed from the visual glyph list.
            assert_eq!(galley.end().index.0, source.chars().count());
            let start = galley.pos_from_cursor(egui::text::CCursor::new(0));
            let after_word = galley.pos_from_cursor(egui::text::CCursor::new(2));
            assert!(start.center().x > after_word.center().x);
            let source = "تم فتح المستند ويمكن الآن قراءة جميع صفحاته وتحريرها وحفظها وطباعتها باستخدام الأدوات المتاحة";
            let wrapped = fonts.layout(source.to_owned(), egui::FontId::proportional(14.0), egui::Color32::WHITE, 150.0);
            assert!(wrapped.rows.len() > 1);
            assert_eq!(wrapped.job.text, source);
            let first = &wrapped.rows[0];
            let bidi = first.bidi.as_ref().expect("Arabic row keeps logical positions");
            assert!(bidi.glyph_ranges.iter().any(|range| range.start == 0));
            assert!(bidi.cursor_x[0] > bidi.cursor_x[2]);
            assert!(first.char_count_excluding_newline().0 < source.chars().count());
            assert_eq!(wrapped.end().index.0, source.chars().count());
        });
    }
}
