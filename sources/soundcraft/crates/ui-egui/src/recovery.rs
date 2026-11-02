//! The offer to recover unsaved work an earlier run left in the autosave folder.

use crate::SoundApp;
use crate::theme::{Tokens, bold};
use egui::{Align2, vec2};

/// Offer the newest leftover autosave, one at a time.
pub fn show(app: &mut SoundApp, ctx: &egui::Context) {
    let Some(left) = app.leftovers.first().cloned() else { return };
    let mut open = true;
    let mut recover = None;
    egui::Window::new(egui::RichText::new(crate::i18n::t("Recover Unsaved Work")).font(bold(13.0)))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, vec2(0.0, -80.0))
        .show(ctx, |ui| {
            let age = left.saved.elapsed().map_or_else(|_| crate::i18n::t("recently").into(), |d| ago(d.as_secs()));
            let what = crate::i18n::t(if left.recording { "unsaved changes and an unfinished recording" } else { "unsaved changes" });
            ui.label(crate::i18n::format(
                "“{}” has {what} from an earlier run, autosaved {age}.",
                &[("", left.name.to_string()), ("what", what.to_string()), ("age", age.to_string())],
            ));
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button(crate::i18n::t("Discard")).clicked() {
                    recover = Some(false);
                }
                let recover_button = egui::Button::new(egui::RichText::new(crate::i18n::t("Recover")).strong()).fill(Tokens::current().accent);
                if ui
                    .add_enabled(!app.engine.transport.recording, recover_button)
                    .on_disabled_hover_text(crate::i18n::t("Stop recording first"))
                    .clicked()
                {
                    recover = Some(true);
                }
            });
        });
    if !open || recover.is_some() {
        app.leftovers.retain(|l| *l != left);
    }
    match recover {
        Some(true) => app.recover(&left),
        Some(false) => soundcraft_engine::autosave::discard(&left),
        None => {}
    }
}

fn ago(secs: u64) -> String {
    let (n, unit) = match secs {
        0..60 => return crate::i18n::t("less than a minute ago").into(),
        60..3_600 => (secs / 60, "minute"),
        3_600..86_400 => (secs / 3_600, "hour"),
        _ => (secs / 86_400, "day"),
    };
    if crate::i18n::current() == crate::i18n::Language::Ar {
        return crate::i18n::arabic_ago(n, unit);
    }
    format!("{n} {unit}{} ago", if n == 1 { "" } else { "s" })
}

#[cfg(test)]
mod arabic_recovery_tests {
    #[test]
    fn recovery_age_and_session_name_are_natural_and_verbatim() {
        use crate::i18n::{self, Language};
        let prior = i18n::current();
        i18n::set_current(Language::Ar);
        assert_eq!(super::ago(30), "قبل أقل من دقيقة");
        assert_eq!(super::ago(60), "قبل دقيقة واحدة");
        assert_eq!(super::ago(120), "قبل دقيقتين");
        assert_eq!(super::ago(3 * 3600), "قبل 3 ساعات");
        let name = "Open {literal} جلسة File123";
        let rendered = i18n::format(
            "“{}” has {what} from an earlier run, autosaved {age}.",
            &[("", name.into()), ("what", i18n::t("unsaved changes").into()), ("age", super::ago(120))],
        );
        assert!(rendered.contains(name));
        assert!(!rendered.contains("unsaved changes"));
        i18n::set_current(Language::En);
        assert_eq!(super::ago(120), "2 minutes ago");
        i18n::set_current(prior);
    }
}
