//! Interface chrome only: documents, command IDs and paths are never translated.
use serde::{Deserialize, Serialize};
use std::cell::Cell;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Ar,
}
impl Language {
    pub const ALL: [Self; 2] = [Self::En, Self::Ar];
    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ar => "العربية",
        }
    }
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ar => "ar",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "en" => Some(Self::En),
            "ar" => Some(Self::Ar),
            _ => None,
        }
    }
    pub fn tr(self, s: &str) -> &str {
        if self == Self::Ar { arabic::translate(s).unwrap_or(s) } else { s }
    }
}
mod arabic;
thread_local! { static CURRENT: Cell<Language> = const { Cell::new(Language::En) }; }
pub fn set_current(language: Language) {
    CURRENT.set(language);
}
pub fn current() -> Language {
    CURRENT.get()
}
pub fn t(s: &str) -> &str {
    CURRENT.get().tr(s)
}
/// Translate only the UI description of a time step, retaining its engine value.
pub(crate) fn grid_label(grid: soundcraft_time::GridValue) -> String {
    use soundcraft_time::GridValue;
    if current() == Language::En {
        return grid.label();
    }
    match grid {
        GridValue::Note { value, dotted, triplet } => {
            let mut label = t(value.label()).to_string();
            for (active, word) in [(dotted, "dotted"), (triplet, "triplet")] {
                if active {
                    label.push(' ');
                    label.push_str(t(word));
                }
            }
            label
        }
        GridValue::Seconds(value) => format!("{value:.3} ث"),
        GridValue::Frames(value) => format!("{value} إطار"),
        GridValue::Samples(value) => format!("{value} عينة"),
    }
}
pub fn switcher(ui: &mut egui::Ui, language: &mut Language) {
    egui::ComboBox::from_id_salt("interface-language").width(85.0).selected_text(language.name()).show_ui(ui, |ui| {
        for value in Language::ALL {
            ui.selectable_value(language, value, value.name());
        }
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn app_language_action_is_reviewable_and_persists() {
        let mut app = crate::SoundApp::new(soundcraft_engine::Engine::default(), None, Default::default());
        app.run("app.language.set", serde_json::json!({"language": "ar"})).unwrap();
        assert_eq!(app.ui.language, Language::Ar);
        let restored: crate::UiState = serde_json::from_str(&serde_json::to_string(&app.ui).unwrap()).unwrap();
        assert_eq!(restored.language, Language::Ar);
        app.run("app.language.set", serde_json::json!({"language": "en"})).unwrap();
        assert_eq!(app.ui.language, Language::En);
    }
    #[test]
    fn language_persists_and_switches_without_touching_documents() {
        for language in Language::ALL {
            let value = serde_json::to_string(&language).unwrap();
            assert_eq!(serde_json::from_str::<Language>(&value).unwrap(), language);
        }
        assert_eq!(Language::Ar.tr("Save"), "حفظ");
        assert_eq!(Language::En.tr("Save"), "Save");
        assert_eq!(Language::Ar.tr("my-document.pptx"), "my-document.pptx");
    }
    #[test]
    fn arabic_edit_screen_paints_localized_chrome() {
        let _palette = crate::theme::lock_test_palette();
        let ctx = egui::Context::default();
        crate::fonts::install(&ctx);
        let mut app = crate::SoundApp::new(soundcraft_engine::Engine::default(), None, Default::default());
        app.ui.language = Language::Ar;
        let mut output = egui::FullOutput::default();
        for _ in 0..3 {
            output = ctx.run_ui(
                egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1600.0, 1000.0))), ..Default::default() },
                |ui| {
                    app.logic(&ui.ctx().clone());
                    app.ui(ui);
                },
            );
            // This text-only test intentionally has no texture renderer.
            output.textures_delta.clear();
        }
        let texts: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                _ => None,
            })
            .collect();
        for label in [
            "SHUFFLE",
            "SLIP",
            "TRACKS",
            "CLIPS",
            "GROUPS",
            "Tracks and Groups",
            "Start",
            "End",
            "Length",
            "MIDI EDITOR",
            "MEMORY LOCATIONS",
            "UNDO HISTORY",
            "Bars|Beats",
        ] {
            assert!(texts.contains(&Language::Ar.tr(label)), "Arabic screen did not paint {label}: {texts:?}");
            assert!(!texts.contains(&label), "English chrome still painted: {label}");
        }
    }
}

/// Format interface text after translating its literal template; values remain verbatim.
pub fn format(template: &str, values: &[(&str, String)]) -> String {
    let translated = t(template);
    let mut out = String::new();
    let mut rest = translated;
    let mut positional = 0usize;
    while let Some((before, after)) = rest.split_once('{') {
        out.push_str(before);
        let Some((key, tail)) = after.split_once('}') else {
            out.push('{');
            out.push_str(after);
            return out;
        };
        let value = if key.starts_with(':') || key.is_empty() {
            let v = values.iter().filter(|(key, _)| key.is_empty() || key.starts_with(':')).nth(positional).map(|(_, v)| v.as_str());
            positional = positional.saturating_add(1);
            v
        } else {
            values.iter().find(|(k, _)| *k == key).map(|(_, v)| v.as_str())
        };
        match value {
            Some(value) => out.push_str(value),
            None => {
                out.push('{');
                out.push_str(key);
                out.push('}');
            }
        }
        rest = tail;
    }
    out.push_str(rest);
    out
}

/// Translate error presentation only; API errors and parameter/command IDs remain unchanged.
pub fn error_text(message: &str) -> String {
    if CURRENT.get() != Language::Ar {
        return message.to_string();
    }
    let dynamic = crate::diagnostic_display::display_message(message);
    if dynamic != message {
        return dynamic;
    }
    let translated = t(message);
    if translated != message {
        return translated.to_string();
    }
    for (prefix, template) in [
        ("bad parameters: ", "Bad parameters: {message}"),
        ("not available: ", "Not available: {message}"),
        ("unsupported: ", "Unsupported: {message}"),
    ] {
        if let Some(detail) = message.strip_prefix(prefix) {
            return error_format(template, &[("message", crate::diagnostic_display::display_message(detail))]);
        }
    }
    if let Some(command) = message.strip_prefix("unknown command `").and_then(|value| value.strip_suffix('`')) {
        return error_format("Unknown command `{command}`", &[("command", command.to_string())]);
    }
    for prefix in ["invalid parameters for `", "bad parameters for `"] {
        if let Some((command, detail)) = message.strip_prefix(prefix).and_then(|value| value.split_once("`: ")) {
            return error_format(
                "Invalid parameters for `{command}`: {message}",
                &[("command", command.to_string()), ("message", crate::diagnostic_display::display_message(detail))],
            );
        }
    }
    for prefix in ["command `", "`"] {
        if let Some(value) = message.strip_prefix(prefix) {
            for marker in ["` is not available right now: ", "` is not available: "] {
                if let Some((command, detail)) = value.split_once(marker) {
                    return error_format(
                        "Command `{command}` is not available: {message}",
                        &[("command", command.to_string()), ("message", crate::diagnostic_display::display_message(detail))],
                    );
                }
            }
        }
    }
    message.to_string()
}

fn error_format(template: &str, values: &[(&str, String)]) -> String {
    format(template, values)
}

#[cfg(test)]
mod error_display_tests {
    use super::*;

    #[test]
    fn translated_error_display_preserves_ids_and_original_english() {
        set_current(Language::Ar);
        assert_eq!(error_text("unknown command `probe.{name}`"), "أمر غير معروف `probe.{name}`");
        assert_eq!(error_text("bad parameters: literal-{path}"), "معلمات غير صالحة: literal-{path}");
        set_current(Language::En);
        assert_eq!(error_text("bad parameters: literal-{path}"), "bad parameters: literal-{path}");
    }
}

#[cfg(test)]
mod dsp_registry_coverage_tests {
    use super::*;
    #[test]
    fn arabic_covers_builtin_audio_processor_metadata() {
        for plugin in soundcraft_dsp::plugins() {
            assert_ne!(Language::Ar.tr(plugin.name), plugin.name, "plugin: {}", plugin.name);
            assert_ne!(Language::Ar.tr(plugin.short_name), plugin.short_name, "short plugin name: {}", plugin.short_name);
            for parameter in plugin.params {
                assert_ne!(Language::Ar.tr(parameter.name), parameter.name, "parameter: {}", parameter.name);
                for choice in parameter.choices {
                    if !choice.chars().all(|c| c.is_ascii_digit() || c == '.') {
                        assert_ne!(Language::Ar.tr(choice), *choice, "choice: {choice}");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod mixed_format_tests {
    #[test]
    fn named_and_positional_values_keep_their_own_indices_and_literal_braces() {
        super::set_current(super::Language::En);
        assert_eq!(super::format("{name}: {} / {}", &[("name", "اسم {x}".into()), ("", "abc".into()), ("", "123".into())]), "اسم {x}: abc / 123");
    }
}

/// Arabic elapsed time, with singular/dual/plural forms; numbers stay logical ASCII values.
pub(crate) fn arabic_ago(n: u64, unit: &str) -> String {
    let (single, dual, plural, accusative) = match unit {
        "minute" => ("دقيقة واحدة", "دقيقتين", "دقائق", "دقيقة"),
        "hour" => ("ساعة واحدة", "ساعتين", "ساعات", "ساعة"),
        "day" => ("يوم واحد", "يومين", "أيام", "يومًا"),
        "week" => ("أسبوع واحد", "أسبوعين", "أسابيع", "أسبوعًا"),
        "month" => ("شهر واحد", "شهرين", "أشهر", "شهرًا"),
        "year" => ("سنة واحدة", "سنتين", "سنوات", "سنة"),
        _ => return std::format!("قبل {n} {unit}"),
    };
    match n {
        1 => std::format!("قبل {single}"),
        2 => std::format!("قبل {dual}"),
        _ if (3..=10).contains(&(n % 100)) => std::format!("قبل {n} {plural}"),
        _ => std::format!("قبل {n} {accusative}"),
    }
}
