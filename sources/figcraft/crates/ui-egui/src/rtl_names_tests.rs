//! Right-to-left and mixed-direction names: elided display and the rename field's visual caret,
//! driven through egui headless.

use egui::text::{CCursor, CCursorRange, CharIndex};
use egui::{Event, Key, Modifiers, Pos2, RawInput, Rect, vec2};

use super::*;
use crate::theme::regular;

fn frame(ctx: &egui::Context, events: Vec<Event>, mut f: impl FnMut(&mut Ui)) {
    let input = RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0))), events, ..Default::default() };
    ctx.run_ui(input, |ui| f(ui)).textures_delta.clear();
}

fn key(k: Key, shift: bool) -> Event {
    Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: if shift { Modifiers::SHIFT } else { Modifiers::NONE } }
}

fn caret(ctx: &egui::Context, id: Id) -> (usize, usize) {
    let r = egui::TextEdit::load_state(ctx, id).and_then(|s| s.cursor.char_range()).unwrap_or_default();
    (r.primary.index.0, r.secondary.index.0)
}

fn set_caret(ctx: &egui::Context, id: Id, i: usize) {
    let mut s = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    s.cursor.set_char_range(Some(CCursorRange::one(CCursor::new(i))));
    s.store(ctx, id);
}

#[test]
fn names_elide_at_their_end() {
    let ctx = egui::Context::default();
    frame(&ctx, vec![], |ui| {
        let p = ui.painter();
        let g = name_galley(p, "Rectangle with a rather long name", regular(11.0), Color32::WHITE, 60.0);
        assert!(g.elided);
        assert!(g.size().x <= 61.0);
        let g = name_galley(p, "Frame", regular(11.0), Color32::WHITE, 200.0);
        assert_eq!(g.text(), "Frame");
        // Right to left: shown in visual order; cut names keep their start, at the right.
        let g = name_galley(p, "שלום", regular(11.0), Color32::WHITE, 500.0);
        assert_eq!(g.text(), "שלום");
        let g = name_galley(p, "שלום עולם ומלואו", regular(11.0), Color32::WHITE, 40.0);
        assert!(g.elided);
        assert!(g.rows[0].row.glyphs.first().is_some_and(|g| g.chr == '…'));
        assert!(g.size().x <= 41.0);
        // Arabic letters join.
        let g = name_galley(p, "سلام", regular(11.0), Color32::WHITE, 500.0);
        assert_eq!(g.text(), "سلام", "Document names remain logical Unicode");
    });
}

#[test]
fn rename_caret_moves_visually() {
    let ctx = egui::Context::default();
    let id = Id::new("rename_test");
    let mut text = "abc אבג".to_string();
    let run = |events: Vec<Event>, text: &mut String| {
        frame(&ctx, events, |ui| {
            ui.memory_mut(|m| m.request_focus(id));
            single_line(ui, id, text, regular(11.0), 200.0);
        });
    };
    run(vec![], &mut text);
    run(vec![], &mut text);
    set_caret(&ctx, id, 3);
    // Right from the space: into the Hebrew word at its visual left, which is its logical end.
    run(vec![key(Key::ArrowRight, false)], &mut text);
    assert_eq!(caret(&ctx, id).0, 7);
    run(vec![key(Key::ArrowRight, false)], &mut text);
    assert_eq!(caret(&ctx, id).0, 6);
    // Shift extends from where the caret was.
    run(vec![key(Key::ArrowRight, true)], &mut text);
    assert_eq!(caret(&ctx, id), (5, 6));
    // Without Shift a selection collapses to its side.
    run(vec![key(Key::ArrowLeft, false)], &mut text);
    assert_eq!(caret(&ctx, id), (6, 6));
    run(vec![key(Key::ArrowLeft, false), key(Key::ArrowLeft, false)], &mut text);
    assert_eq!(caret(&ctx, id).0, 3);
    // Typing still goes in at the caret.
    run(vec![Event::Text("ד".into())], &mut text);
    assert_eq!(text, "abcד אבג");
    // Plain left-to-right text keeps egui's own caret.
    let mut latin = "Frame".to_string();
    let lid = Id::new("latin");
    let mut run = |events: Vec<Event>| {
        frame(&ctx, events, |ui| {
            ui.memory_mut(|m| m.request_focus(lid));
            single_line(ui, lid, &mut latin, regular(11.0), 200.0);
        });
    };
    run(vec![]);
    run(vec![]);
    set_caret(&ctx, lid, 1);
    run(vec![key(Key::ArrowRight, false)]);
    assert_eq!(caret(&ctx, lid).0, 2);
}

#[test]
fn clicks_place_the_caret_by_what_is_shown() {
    let ctx = egui::Context::default();
    let id = Id::new("click_test");
    let mut text = "שלום".to_string();
    let mut rect = Rect::NOTHING;
    let run = |events: Vec<Event>, text: &mut String, rect: &mut Rect| {
        frame(&ctx, events, |ui| {
            ui.memory_mut(|m| m.request_focus(id));
            *rect = single_line(ui, id, text, regular(11.0), 200.0).rect;
        });
    };
    run(vec![], &mut text, &mut rect);
    run(vec![], &mut text, &mut rect);
    // The text's right end is its start.
    let galley = ctx.fonts_mut(|f| f.layout_no_wrap("שלום".into(), regular(11.0), Color32::WHITE));
    let right = Pos2::new(rect.min.x + galley.size().x - 1.0, rect.center().y);
    let press = |p: Pos2, pressed: bool| Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed, modifiers: Modifiers::NONE };
    run(vec![Event::PointerMoved(right), press(right, true)], &mut text, &mut rect);
    run(vec![press(right, false)], &mut text, &mut rect);
    assert_eq!(caret(&ctx, id).0, 0);
    let left = Pos2::new(rect.min.x + 1.0, rect.center().y);
    run(vec![Event::PointerMoved(left), press(left, true)], &mut text, &mut rect);
    run(vec![press(left, false)], &mut text, &mut rect);
    assert_eq!(caret(&ctx, id).0, 4);
}

#[test]
fn arabic_names_keep_latin_and_numbers_in_visual_order() {
    let ctx = egui::Context::default();
    crate::theme::install_fonts(&ctx, true, 0);
    frame(&ctx, vec![], |ui| {
        let source = "كتاب جديد PhotoCraft 12";
        let g = name_galley(ui.painter(), source, regular(14.0), Color32::WHITE, 500.0);
        assert_eq!(g.text(), source);
        let glyphs: String = g.rows[0].row.glyphs.iter().map(|g| g.chr).collect();
        assert!(glyphs.contains("PhotoCraft 12"), "{glyphs}");
        // A shaped Arabic cluster may emit multiple glyphs. Its source/cursor map,
        // rather than concatenated glyph labels, defines the displayed characters.
        let row = &g.rows[0].row;
        let bidi = row.bidi.as_ref().expect("mixed Arabic uses Unicode bidi layout");
        assert_eq!(bidi.cursor_x.len(), source.chars().count() + 1);
        assert!(bidi.cursor_x.iter().all(|x| x.is_finite()));
        assert!(row.x_offset(CharIndex(0)) > row.x_offset(CharIndex(4)), "Arabic reads right to left");
        let latin = source.split("PhotoCraft").next().unwrap().chars().count();
        let positions: Vec<_> = (latin..source.chars().count()).map(|i| row.x_offset(CharIndex(i))).collect();
        assert!(positions.windows(2).all(|w| w[0] < w[1]), "Latin names and digits read left to right");
    });
}

#[test]
fn arabic_rename_selection_collapses_visually_and_keeps_input_logical() {
    let ctx = egui::Context::default();
    crate::theme::install_fonts(&ctx, true, 0);
    let id = Id::new("arabic_selection_collapse");
    let mut text = "abc بجد".to_string();
    let run = |events, text: &mut String| {
        frame(&ctx, events, |ui| {
            ui.memory_mut(|m| m.request_focus(id));
            single_line(ui, id, text, regular(14.0), 240.0);
        });
    };
    run(vec![], &mut text);
    run(vec![], &mut text);
    let select = || {
        let mut state = egui::TextEdit::load_state(&ctx, id).unwrap();
        state.cursor.set_char_range(Some(CCursorRange::two(CCursor::new(5), CCursor::new(6))));
        state.store(&ctx, id);
    };
    select();
    run(vec![key(Key::ArrowRight, false)], &mut text);
    assert_eq!(caret(&ctx, id), (5, 5));
    select();
    run(vec![key(Key::ArrowLeft, false)], &mut text);
    assert_eq!(caret(&ctx, id), (6, 6));
    run(vec![Event::Text("س".into())], &mut text);
    assert_eq!(text, "abc بجسد");
}

/// Shaped glyphs occupy visual positions, while the source and cursor count remain logical.
#[test]
fn shown_glyphs_keep_the_visual_order() {
    let ctx = egui::Context::default();
    frame(&ctx, vec![], |ui| {
        for name in
            ["\u{05D0}\u{05D1}", "abc \u{05D0}\u{05D1}\u{05D2} (1)", "\u{0633}\u{0644}\u{0627}\u{0645} \u{0639}\u{0644}\u{064A}\u{0643}\u{0645}"]
        {
            let g = name_galley(ui.painter(), name, regular(11.0), Color32::WHITE, 500.0);
            let glyphs: Vec<(char, f32)> = g.rows.iter().flat_map(|r| r.row.glyphs.iter().map(|gl| (gl.chr, gl.pos.x))).collect();
            assert_eq!(g.text(), name, "Logical source survives display");
            assert_eq!(g.rows[0].row.char_count_excluding_newline().0, name.chars().count());
            assert!(glyphs.windows(2).all(|w| w[0].1 <= w[1].1), "{name}: left to right");
        }
    });
}
