//! Ordinary Arabic input/display regressions; stored document text remains logical Unicode.
use egui::text::{CCursor, LayoutJob};
use egui::{Color32, FontData, FontId, Galley, Vec2};
use std::sync::Arc;

fn contexts() -> Vec<egui::Context> {
    let mut files: Vec<Vec<u8>> = photocraft_text::CRAFT_FONTS
        .iter()
        .filter(|font| font.family == "Noto Sans Arabic" && font.scripts.contains(&"Arab"))
        .map(|font| font.bytes.to_vec())
        .collect();
    #[cfg(target_os = "windows")]
    if let Some(windows) = std::env::var_os("WINDIR")
        && let Ok(bytes) = std::fs::read(std::path::PathBuf::from(windows).join("Fonts/tahoma.ttf"))
    {
        files.push(bytes);
    }
    if files.is_empty() {
        eprintln!("Arabic input font checks skipped: no optional ordinary Arabic font input");
    }
    files
        .into_iter()
        .map(|bytes| {
            let ctx = egui::Context::default();
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert("ordinary Arabic".into(), Arc::new(FontData::from_owned(bytes)));
            for stack in fonts.families.values_mut() {
                stack.insert(0, "ordinary Arabic".into());
            }
            ctx.set_fonts(fonts);
            ctx.run_ui(egui::RawInput::default(), |_| {}).textures_delta.clear();
            ctx
        })
        .collect()
}

fn layout(ctx: &egui::Context, text: &str, width: f32) -> Arc<Galley> {
    ctx.fonts_mut(|fonts| fonts.layout(text.to_owned(), FontId::proportional(14.0), Color32::WHITE, width))
}

#[test]
fn mixed_latin_click_and_arrows_address_the_original_string() {
    for ctx in contexts() {
        let text = "أبج File123";
        let galley = layout(&ctx, text, f32::INFINITY);
        let row = &galley.rows[0];
        let f = row.glyphs.iter().find(|glyph| glyph.chr == 'F').unwrap();
        let hit = galley.cursor_from_pos(Vec2::new(row.pos.x + f.pos.x + 0.1, row.rect().center().y));
        assert_eq!(hit.index.0, 4);
        assert_eq!(galley.cursor_right_one_character(&hit).index.0, 5);
        assert_eq!(galley.cursor_left_one_character(&CCursor::new(5)).index.0, 4);
        assert_eq!(galley.text(), text);
        let arabic = layout(&ctx, "أبج", f32::INFINITY);
        assert!(arabic.pos_from_cursor(CCursor::new(0)).center().x > arabic.pos_from_cursor(CCursor::new(3)).center().x);
        assert_eq!(arabic.cursor_left_one_character(&CCursor::new(0)).index.0, 1);
        assert_eq!(arabic.cursor_right_one_character(&CCursor::new(3)).index.0, 2);
    }
}

#[test]
fn selection_rectangles_follow_logical_ranges_in_both_directions() {
    for ctx in contexts() {
        for (text, start, end) in [("أبج File123", 4, 5), ("أبج", 0, 1), ("مرحبا abc 123", 6, 7), ("لا abc", 0, 1)] {
            let mut galley = layout(&ctx, text, f32::INFINITY);
            let before = galley.rows[0].visuals.mesh.vertices.len();
            let a = galley.pos_from_cursor(CCursor::new(start)).center().x;
            let b = galley.pos_from_cursor(CCursor::new(end)).center().x;
            egui::text_selection::visuals::paint_text_selection(
                &mut galley,
                &egui::Visuals::dark(),
                &egui::text_selection::CCursorRange::two(CCursor::new(start), CCursor::new(end)),
                None,
            );
            let selected = &galley.rows[0].visuals.mesh.vertices[before..];
            assert!(!selected.is_empty(), "{text}: {start}..{end}");
            let left = selected.iter().map(|vertex| vertex.pos.x).min_by(f32::total_cmp).unwrap();
            let right = selected.iter().map(|vertex| vertex.pos.x).max_by(f32::total_cmp).unwrap();
            assert!(right > left, "selection width is positive");
            assert!((left - a.min(b)).abs() < 1.5, "{text}: left {left}, caret {a}/{b}");
            assert!((right - a.max(b)).abs() < 1.5, "{text}: right {right}, caret {a}/{b}");
        }
    }
}

#[test]
fn wrapped_rows_blank_paragraphs_and_clusters_keep_logical_indices() {
    for ctx in contexts() {
        for source in ["أبج\n\nFile123", "مرحبا بالعالم لا 2026 abc\nالسطر الثاني", "لَا ملف 🇸🇦 abc 123"] {
            let galley = layout(&ctx, source, 100.0);
            assert_eq!(galley.end().index.0, source.chars().count());
            assert_eq!(galley.text(), source);
            for row in &galley.rows {
                assert!(row.size.y > 0.0);
            }
            for index in 0..=source.chars().count() {
                let rect = galley.pos_from_cursor(CCursor::new(index));
                assert!(rect.min.x.is_finite() && rect.min.y.is_finite() && rect.max.x.is_finite() && rect.max.y.is_finite(), "{source}: {index}");
                let hit = galley.cursor_from_pos(rect.center().to_vec2());
                assert!(hit.index.0 <= source.chars().count());
                let back = galley.pos_from_cursor(hit);
                assert!((back.center().x - rect.center().x).abs() < 1.5);
            }
        }
        let galley =
            ctx.fonts_mut(|fonts| fonts.layout_job(LayoutJob::simple("أبج\nFile123".into(), FontId::proportional(14.0), Color32::WHITE, f32::INFINITY)));
        assert_eq!(galley.end().index.0, 11);
        ctx.fonts_mut(|fonts| {
            assert!(fonts.has_glyphs(&FontId::proportional(14.0), "أبج File123"));
            assert!(!fonts.has_glyph(&FontId::proportional(14.0), '\u{10ffff}'));
        });
    }
}

#[test]
fn long_words_elision_and_line_breaks_keep_reading_order() {
    for ctx in contexts() {
        let mut job = LayoutJob::simple_singleline("مستندجديدللطباعة ABC123".into(), FontId::proportional(14.0), Color32::WHITE);
        job.wrap = egui::text::TextWrapping::wrap_at_width(45.0);
        let wrapped = ctx.fonts_mut(|fonts| fonts.layout_job(job.clone()));
        assert!(wrapped.rows.len() > 2);
        assert_eq!(wrapped.end().index.0, job.text.chars().count());
        assert!(wrapped.rows.iter().all(|row| row.size.x <= 45.5));
        job.wrap = egui::text::TextWrapping::truncate_at_width(45.0);
        let truncated = ctx.fonts_mut(|fonts| fonts.layout_job(job));
        assert!(truncated.elided);
        assert_eq!(truncated.rows.len(), 1);
        assert!(truncated.size().x <= 45.5);
        assert_eq!(truncated.rows[0].glyphs.first().unwrap().chr, '…');
        let mut job = LayoutJob::simple_singleline("ملف\nجديد".into(), FontId::proportional(14.0), Color32::WHITE);
        let single = ctx.fonts_mut(|fonts| fonts.layout_job(job.clone()));
        assert_eq!(single.rows.len(), 1);
        assert_eq!(single.text(), "ملف\nجديد");
        job.break_on_newline = true;
        assert_eq!(ctx.fonts_mut(|fonts| fonts.layout_job(job)).rows.len(), 2);
    }
}

#[test]
fn elision_keeps_the_original_rich_text_section_styles() {
    for ctx in contexts() {
        let mut job = LayoutJob::default();
        job.append("أبج\n", 0.0, egui::TextFormat { font_id: FontId::proportional(14.0), color: Color32::GREEN, ..Default::default() });
        job.append("File123 طويللللللللل", 0.0, egui::TextFormat { font_id: FontId::proportional(14.0), color: Color32::RED, ..Default::default() });
        job.wrap.max_width = 45.0;
        job.wrap.max_rows = 2;
        let galley = ctx.fonts_mut(|fonts| fonts.layout_job(job));
        assert!(galley.elided);
        let row = &galley.rows[1];
        let glyph = row.glyphs.iter().find(|glyph| glyph.chr == 'F').unwrap();
        assert_eq!(row.visuals.mesh.vertices[glyph.first_vertex as usize].color, Color32::RED);
    }
}

fn edit_frame(ctx: &egui::Context, text: &mut String, events: Vec<egui::Event>) -> (egui::Id, egui::Pos2, egui::Rect) {
    let id = egui::Id::new("Arabic mixed name/search input regression");
    let mut observed = (id, egui::Pos2::ZERO, egui::Rect::NOTHING);
    let input = egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(600.0, 180.0))), events, ..Default::default() };
    ctx.run_ui(input, |ui| {
        ui.memory_mut(|memory| memory.request_focus(id));
        let out = egui::TextEdit::singleline(text).id(id).desired_width(420.0).show(ui);
        observed = (id, out.galley_pos, out.response.rect);
    })
    .textures_delta
    .clear();
    observed
}

fn key(key: egui::Key) -> egui::Event {
    egui::Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::NONE }
}

fn select(ctx: &egui::Context, id: egui::Id, a: usize, b: usize) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap();
    state.cursor.set_char_range(Some(egui::text_selection::CCursorRange::two(CCursor::new(a), CCursor::new(b))));
    state.store(ctx, id);
}

fn caret(ctx: &egui::Context, id: egui::Id) -> (usize, usize) {
    let range = egui::TextEdit::load_state(ctx, id).unwrap().cursor.char_range().unwrap();
    (range.primary.index.0, range.secondary.index.0)
}

#[test]
fn actual_text_edit_click_insert_delete_arrows_and_selection_keep_logical_text() {
    for ctx in contexts() {
        let original = "أبج File123";
        let mut text = original.to_owned();
        edit_frame(&ctx, &mut text, vec![]);
        let (id, origin, rect) = edit_frame(&ctx, &mut text, vec![]);
        let pos = egui::pos2(origin.x + 0.1, rect.center().y);
        let pointer = |pressed| egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed, modifiers: egui::Modifiers::NONE };
        edit_frame(&ctx, &mut text, vec![egui::Event::PointerMoved(pos), pointer(true), pointer(false)]);
        assert_eq!(caret(&ctx, id), (4, 4));
        edit_frame(&ctx, &mut text, vec![egui::Event::Text("X".into())]);
        assert_eq!(text, "أبج XFile123");
        edit_frame(&ctx, &mut text, vec![key(egui::Key::Backspace)]);
        assert_eq!(text, original);
        edit_frame(&ctx, &mut text, vec![key(egui::Key::ArrowRight)]);
        assert_eq!(caret(&ctx, id), (5, 5));
        edit_frame(&ctx, &mut text, vec![egui::Event::Text("Y".into())]);
        assert_eq!(text, "أبج FYile123");
        select(&ctx, id, 0, text.chars().count());
        edit_frame(&ctx, &mut text, vec![egui::Event::Text("بحث abc".into())]);
        assert_eq!(text, "بحث abc");
    }
}

#[test]
fn actual_text_edit_arabic_selection_collapses_to_its_visual_side() {
    for ctx in contexts() {
        let mut text = "abc بجد".to_owned();
        let (id, _, _) = edit_frame(&ctx, &mut text, vec![]);
        edit_frame(&ctx, &mut text, vec![]);
        select(&ctx, id, 5, 6);
        edit_frame(&ctx, &mut text, vec![key(egui::Key::ArrowRight)]);
        assert_eq!(caret(&ctx, id), (5, 5));
        select(&ctx, id, 5, 6);
        edit_frame(&ctx, &mut text, vec![key(egui::Key::ArrowLeft)]);
        assert_eq!(caret(&ctx, id), (6, 6));
        edit_frame(&ctx, &mut text, vec![egui::Event::Text("س".into())]);
        assert_eq!(text, "abc بجسد");
        // Plain left-to-right editing retains its original behavior.
        text = "File123".into();
        edit_frame(&ctx, &mut text, vec![]);
        select(&ctx, id, 2, 5);
        edit_frame(&ctx, &mut text, vec![key(egui::Key::ArrowLeft)]);
        assert_eq!(caret(&ctx, id), (2, 2));
        select(&ctx, id, 2, 5);
        edit_frame(&ctx, &mut text, vec![key(egui::Key::ArrowRight)]);
        assert_eq!(caret(&ctx, id), (5, 5));
    }
}
