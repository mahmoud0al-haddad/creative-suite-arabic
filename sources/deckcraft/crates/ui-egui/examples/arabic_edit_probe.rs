//! Diagnostic of real TextEdit keyboard/pointer editing, without a desktop window.
use egui::text::{CCursor, CCursorRange};
use egui::{Context, Event, FontFamily, Id, Key, Modifiers, PointerButton};

fn frame(ctx: &Context, text: &mut String, events: Vec<Event>) -> Result<egui::text_edit::TextEditOutput, Box<dyn std::error::Error>> {
    let mut edit = None;
    let mut output = ctx.run_ui(
        egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 200.0))), events, ..Default::default() },
        |ui| {
            edit = Some(
                egui::TextEdit::singleline(text)
                    .id(Id::new("mixed-input"))
                    .font(egui::FontId::new(22.0, FontFamily::Proportional))
                    .desired_width(700.0)
                    .show(ui),
            );
        },
    );
    output.textures_delta.clear();
    edit.ok_or_else(|| "missing TextEdit output".into())
}

fn cursor(ctx: &Context, index: usize) {
    let id = Id::new("mixed-input");
    let mut state = egui::text_edit::TextEditState::load(ctx, id).unwrap_or_default();
    state.cursor.set_char_range(Some(CCursorRange::one(CCursor::new(index))));
    state.store(ctx, id);
    ctx.memory_mut(|m| m.request_focus(id));
}

fn key(key: Key, modifiers: Modifiers) -> Event {
    Event::Key { key, physical_key: Some(key), pressed: true, repeat: false, modifiers }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::default();
    deckcraft_ui_egui::theme::install_fonts(&ctx);
    let original = "مرحبا abc 123";
    let mut text = original.to_string();
    let _ = frame(&ctx, &mut text, vec![])?;
    cursor(&ctx, 0);
    let initial = frame(&ctx, &mut text, vec![])?;
    let carets: Vec<_> = (0..=text.chars().count())
        .map(|i| {
            serde_json::json!({
                "logical_index": i, "x": initial.galley.pos_from_cursor(CCursor::new(i)).center().x
            })
        })
        .collect();
    let row = initial.galley.rows.first().ok_or("no rendered row")?;
    let glyphs: Vec<_> = row.glyphs.iter().map(|g| serde_json::json!({"char":g.chr.to_string(),"x":g.pos.x,"width":g.advance_width})).collect();
    let a = row.glyphs.iter().find(|g| g.chr == 'a').ok_or("Latin a missing")?;
    let local = egui::vec2(row.pos.x + a.pos.x + a.advance_width * 0.2, row.pos.y + row.size.y * 0.5);
    let hit = initial.galley.cursor_from_pos(local);
    let pos = initial.galley_pos + local;
    let _ = frame(
        &ctx,
        &mut text,
        vec![Event::PointerMoved(pos), Event::PointerButton { pos, button: PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE }],
    )?;
    let clicked =
        frame(&ctx, &mut text, vec![Event::PointerButton { pos, button: PointerButton::Primary, pressed: false, modifiers: Modifiers::NONE }])?;
    let after_click_cursor = clicked.cursor_range.map(|r| usize::from(r.primary.index));
    let _ = frame(&ctx, &mut text, vec![Event::Text("X".into())])?;
    let after_pointer_typing = text.clone();
    text = original.to_string();
    cursor(&ctx, 3);
    let _ = frame(&ctx, &mut text, vec![key(Key::Backspace, Modifiers::NONE)])?;
    let after_delete = text.clone();
    text = original.to_string();
    cursor(&ctx, 6);
    let shifted = frame(&ctx, &mut text, vec![key(Key::ArrowRight, Modifiers::SHIFT)])?;
    let selection = shifted.cursor_range.map(|r| (usize::from(r.secondary.index), usize::from(r.primary.index)));
    let _ = frame(&ctx, &mut text, vec![Event::Text("Z".into())])?;
    let after_selection_replacement = text.clone();
    text = original.to_string();
    cursor(&ctx, 3);
    let before_arrow = frame(&ctx, &mut text, vec![])?;
    let before_arrow_x = before_arrow.galley.pos_from_cursor(CCursor::new(3)).center().x;
    let after_arrow = frame(&ctx, &mut text, vec![key(Key::ArrowRight, Modifiers::NONE)])?;
    let after_arrow_cursor = after_arrow.cursor_range.map(|r| usize::from(r.primary.index));
    let after_arrow_x = after_arrow.cursor_range.map(|r| after_arrow.galley.pos_from_cursor(r.primary).center().x);
    text = "abc بجد".into();
    let _ = frame(&ctx, &mut text, vec![])?;
    let select_arabic = || {
        let id = Id::new("mixed-input");
        let mut state = egui::text_edit::TextEditState::load(&ctx, id).unwrap_or_default();
        state.cursor.set_char_range(Some(CCursorRange::two(CCursor::new(5), CCursor::new(6))));
        state.store(&ctx, id);
        ctx.memory_mut(|m| m.request_focus(id));
    };
    select_arabic();
    let collapse_right = frame(&ctx, &mut text, vec![key(Key::ArrowRight, Modifiers::NONE)])?;
    let collapse_right = collapse_right.cursor_range.map(|c| (usize::from(c.primary.index), usize::from(c.secondary.index)));
    assert_eq!(collapse_right, Some((5, 5)), "RTL selection Right collapses to visual right");
    select_arabic();
    let collapse_left = frame(&ctx, &mut text, vec![key(Key::ArrowLeft, Modifiers::NONE)])?;
    let collapse_left = collapse_left.cursor_range.map(|c| (usize::from(c.primary.index), usize::from(c.secondary.index)));
    assert_eq!(collapse_left, Some((6, 6)), "RTL selection Left collapses to visual left");
    let _ = frame(&ctx, &mut text, vec![Event::Text("س".into())])?;
    assert_eq!(text, "abc بجسد", "typing after collapse uses logical source order");
    let after_rtl_collapse_typing = text.clone();
    let report = serde_json::json!({
        "rtl_selection_right":collapse_right, "rtl_selection_left":collapse_left, "after_rtl_collapse_typing":after_rtl_collapse_typing,
        "original_logical_text":original, "logical_char_count":original.chars().count(),
        "rendered_glyphs":glyphs,"logical_carets":carets,
        "latin_a_expected_logical_index":6,"latin_a_hit_test_index":usize::from(hit.index),
        "clicked_cursor":after_click_cursor,"after_pointer_typing":after_pointer_typing,
        "expected_pointer_typing":"مرحبا Xabc 123","after_backspace_at_logical_3":after_delete,
        "shift_right_from_logical_6_selection":selection,"after_selection_replacement":after_selection_replacement,
        "arrow_right_from_arabic_logical_3_expected":2, "arrow_right_from_arabic_logical_3_actual":after_arrow_cursor,
        "arrow_right_x_before":before_arrow_x, "arrow_right_x_after":after_arrow_x
    });
    let json = serde_json::to_string_pretty(&report)?;
    println!("{json}");
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, json)?;
    }
    assert_eq!(usize::from(hit.index), 6, "hit testing must map visual Latin position to logical text");
    assert_eq!(after_click_cursor, Some(6));
    assert_eq!(after_pointer_typing, "مرحبا Xabc 123");
    assert_eq!(after_delete, "مربا abc 123");
    assert_eq!(selection, Some((6, 7)));
    assert_eq!(after_selection_replacement, "مرحبا Zbc 123");
    assert_eq!(after_arrow_cursor, Some(2));
    assert!(after_arrow_x.is_some_and(|x| x > before_arrow_x), "Right must move visually right inside Arabic");
    Ok(())
}
