//! Keep logical Unicode for widgets, accessibility and dynamic placeholders.
//! The local epaint renderer wraps logical text and applies bidi separately to each line.
use std::borrow::Cow;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

pub fn visual(text: &str) -> Cow<'_, str> {
    Cow::Borrowed(text)
}

pub fn display_rtl(text: &str) -> String {
    text.to_owned()
}

/// A wrapped logical line. Shaping and UAX9 happen exactly once in epaint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub rtl: bool,
}
pub fn has_rtl(text: &str) -> bool {
    text.chars().any(is_rtl_script)
}
pub fn visual_auto(text: &str) -> Cow<'_, str> {
    Cow::Borrowed(text)
}
pub fn wrap(text: &str, mut fits: impl FnMut(&str) -> bool) -> Vec<Line> {
    let info = unicode_bidi::BidiInfo::new(text, None);
    let mut lines = Vec::new();
    for para in &info.paragraphs {
        let whole = text.get(para.range.clone()).unwrap_or_default();
        if whole == "\n" && text.get(..para.range.start).is_some_and(|before| before.ends_with('\r')) {
            continue;
        }
        let end = para.range.start.saturating_add(whole.trim_end_matches(is_paragraph_separator).len());
        for range in break_lines(text, para.range.start..end, &mut fits) {
            lines.push(Line { text: text.get(range).unwrap_or_default().to_owned(), rtl: para.level.is_rtl() });
        }
    }
    lines
}
/// The byte ranges of the lines `range` of `text` breaks into: as many words as `fits` accepts
/// per line, a word too wide for any line split between characters. Always at least one line.
fn break_lines(text: &str, range: Range<usize>, fits: &mut impl FnMut(&str) -> bool) -> Vec<Range<usize>> {
    let mut lines = Vec::new();
    let mut current: Option<Range<usize>> = None;
    for word in words(text, range.clone()) {
        if let Some(line) = current.as_mut() {
            if fits(text.get(line.start..word.end).unwrap_or_default()) {
                line.end = word.end;
                continue;
            }
            lines.push(line.clone());
        }
        // The word starts a line.
        let mut start = word.start;
        loop {
            let end = fitting_prefix(text, start..word.end, fits);
            if end >= word.end {
                current = Some(start..word.end);
                break;
            }
            lines.push(start..end);
            start = end;
        }
    }
    lines.push(current.unwrap_or(range.start..range.start));
    lines
}

/// The end of the longest start of `range` that `fits`, at least one character.
fn fitting_prefix(text: &str, range: Range<usize>, fits: &mut impl FnMut(&str) -> bool) -> usize {
    let piece = text.get(range.clone()).unwrap_or_default();
    let mut end = range.end;
    for (i, (at, grapheme)) in piece.grapheme_indices(true).enumerate() {
        let next = range.start.saturating_add(at).saturating_add(grapheme.len());
        if i > 0 && !fits(text.get(range.start..next).unwrap_or_default()) {
            end = range.start.saturating_add(at);
            break;
        }
    }
    end
}

/// The byte ranges of the whitespace-separated words of `range` in `text`.
fn words(text: &str, range: Range<usize>) -> Vec<Range<usize>> {
    let mut words = Vec::new();
    let mut start = None;
    for (at, c) in text.get(range.clone()).unwrap_or_default().char_indices() {
        let at = range.start.saturating_add(at);
        match (c.is_whitespace(), start) {
            (true, Some(s)) => {
                words.push(s..at);
                start = None;
            }
            (false, None) => start = Some(at),
            _ => {}
        }
    }
    if let Some(s) = start {
        words.push(s..range.end);
    }
    words
}

/// Characters that end a paragraph (bidi class B).
fn is_paragraph_separator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{1C}'..='\u{1E}' | '\u{85}' | '\u{2029}')
}

fn is_rtl_script(c: char) -> bool {
    matches!(u32::from(c), 0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFC | 0x1_0800..=0x1_0FFF | 0x1_E800..=0x1_EFFF)
}

#[cfg(test)]
mod tests {
    use unicode_segmentation::UnicodeSegmentation;
    #[test]
    fn wrapped_comments_keep_logical_words_graphemes_and_paragraph_direction() {
        let text = "مَرْحَبًا بالعالم PDF 2026\nA Latin paragraph";
        let lines = super::wrap(text, |line| line.graphemes(true).count() <= 9);
        assert!(lines.len() > 2);
        assert_eq!(lines[0].text, "مَرْحَبًا");
        assert!(lines[0].rtl);
        assert!(lines.iter().any(|line| line.text.contains("2026")));
        assert!(!lines.last().unwrap().rtl);
        let split = super::wrap("بِسْمِ", |_| false);
        assert_eq!(split.iter().map(|line| line.text.as_str()).collect::<Vec<_>>(), ["بِ", "سْ", "مِ"]);
    }
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
