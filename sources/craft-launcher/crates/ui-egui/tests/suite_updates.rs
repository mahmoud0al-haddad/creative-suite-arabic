//! The independent edition prompts, notifies once, and never queues upstream component installs.
use artcraft_launcher_core::{
    SUITE,
    protocol::{Backend, Capabilities, Command, Event, Source},
    suite::{self, Component, Installer, Metadata, SuiteRelease, SuiteStatus},
};
use artcraft_launcher_ui_egui::settings::CheckEvery;
use artcraft_launcher_ui_egui::{Host, LauncherUi, Settings};
use egui_kittest::{Harness, kittest::Queryable};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Mutex,
};

// Language selection is process-wide; rendering tests must not race each other.
static I18N_LOCK: Mutex<()> = Mutex::new(());

struct SuiteBackend {
    events: Rc<RefCell<Vec<Event>>>,
    sent: Rc<RefCell<Vec<Command>>>,
    now: Rc<Cell<i64>>,
}
impl Backend for SuiteBackend {
    fn integrated_suite(&self) -> bool {
        true
    }
    fn send(&mut self, command: Command) {
        self.sent.borrow_mut().push(command);
    }
    fn poll(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events.borrow_mut())
    }
    fn capabilities(&self) -> Capabilities {
        Capabilities { manage_apps: true, self_update: false, reveal_label: "Show in Explorer" }
    }
    fn platform(&self) -> Option<artcraft_launcher_core::Platform> {
        Some(artcraft_launcher_core::Platform { os: artcraft_launcher_core::Os::Windows, arch: artcraft_launcher_core::Arch::X64 })
    }
    fn now(&self) -> i64 {
        self.now.get()
    }
}
struct Notices(Rc<RefCell<Vec<(String, String)>>>);
impl Host for Notices {
    fn notify(&mut self, title: &str, body: &str) {
        self.0.borrow_mut().push((title.into(), body.into()));
    }
}
fn update() -> SuiteStatus {
    let tag = "v2026.10-ar.2";
    SuiteStatus::Available(Box::new(SuiteRelease {
        metadata: Metadata {
            schema: 1,
            edition: suite::EDITION.into(),
            platform: suite::PLATFORM.into(),
            install_id: suite::INSTALL_ID.into(),
            revision: 2,
            tag: tag.into(),
            launcher: Component { version: "0.4.1".into(), sha256: "c".repeat(64) },
            applications: SUITE.iter().map(|app| (app.id.into(), Component { version: "0.6.1".into(), sha256: "a".repeat(64) })).collect(),
        },
        installer: Installer { name: suite::SETUP_NAME.into(), url: suite::asset_url(tag, suite::SETUP_NAME), size: 1234, sha256: "b".repeat(64) },
    }))
}
#[test]
fn arabic_notification_prompts_and_user_action_updates_only_complete_suite() {
    let _language_guard = I18N_LOCK.lock().unwrap();
    let events = Rc::new(RefCell::new(vec![
        Event::SuiteUpdate(update()),
        Event::CheckFinished { at: 100000, source: Source::GitHubApi, rate_limited_until: None },
    ]));
    let sent = Rc::new(RefCell::new(Vec::new()));
    let notices = Rc::new(RefCell::new(Vec::new()));
    let mut host = Notices(notices.clone());
    let mut backend = Some(SuiteBackend { events: events.clone(), sent: sent.clone(), now: Rc::new(Cell::new(100000)) });
    let mut harness = Harness::builder().with_size(egui::vec2(1180., 900.)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if let Some(backend) = backend.take() {
                *state = Some(LauncherUi::new(
                    ui.ctx(),
                    Box::new(backend),
                    Settings { language: "ar".into(), notify: true, auto_update: true, ..Settings::default() },
                ));
            }
            if let Some(app) = state {
                app.logic(&ui.ctx().clone(), &mut host);
                app.ui(ui, &mut host);
            }
        },
        None,
    );
    harness.run_steps(8);
    assert_eq!(notices.borrow().len(), 1);
    assert_eq!(notices.borrow()[0].0, "تحديث الحزمة العربية");
    assert!(notices.borrow()[0].1.contains("v2026.10-ar.2"));
    assert!(!sent.borrow().iter().any(|command| matches!(command, Command::Install { .. } | Command::UpdateLauncher | Command::UpdateSuite)));
    harness.get_by_label("تنزيل برنامج التثبيت وتشغيله").click();
    harness.run_steps(3);
    assert_eq!(sent.borrow().iter().filter(|command| **command == Command::UpdateSuite).count(), 1);
    events.borrow_mut().push(Event::SuiteUpdate(update()));
    harness.run_steps(3);
    assert_eq!(notices.borrow().len(), 1, "one notification per revision");
}
#[test]
fn manual_checks_preference_and_daily_schedule_are_preserved() {
    let _language_guard = I18N_LOCK.lock().unwrap();
    let ctx = egui::Context::default();
    let sent = Rc::new(RefCell::new(Vec::new()));
    let clock = Rc::new(Cell::new(100000));
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut host = Notices(Rc::default());
    let backend = SuiteBackend { events: events.clone(), sent: sent.clone(), now: clock.clone() };
    let mut ui = LauncherUi::new(&ctx, Box::new(backend), Settings { check_every: CheckEvery::Never, ..Settings::default() });
    ui.logic(&ctx, &mut host);
    assert!(!sent.borrow().iter().any(|c| matches!(c, Command::CheckForUpdates { .. })), "manual means no startup request");
    ui.settings.check_every = CheckEvery::Day;
    ui.settings.last_check = Some(100000);
    clock.set(100000 + 300);
    ui.logic(&ctx, &mut host);
    assert!(!sent.borrow().iter().any(|c| matches!(c, Command::CheckForUpdates { .. })), "no five-minute bundle polls");
    clock.set(100000 + suite::CHECK_INTERVAL);
    ui.logic(&ctx, &mut host);
    assert!(sent.borrow().contains(&Command::CheckForUpdates { force: false }));
}

#[test]
fn integrated_settings_offer_bundle_updates_without_unused_component_install_choices() {
    let _language_guard = I18N_LOCK.lock().unwrap();
    let mut backend = Some(SuiteBackend { events: Rc::default(), sent: Rc::default(), now: Rc::new(Cell::new(100000)) });
    let mut host = Notices(Rc::default());
    let mut harness = Harness::builder().with_size(egui::vec2(1180., 900.)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if let Some(backend) = backend.take() {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(backend), Settings { language: "en".into(), ..Settings::default() }));
            }
            if let Some(app) = state {
                app.logic(&ui.ctx().clone(), &mut host);
                app.ui(ui, &mut host);
            }
        },
        None,
    );
    harness.run_steps(6);
    harness.get_by_label("Settings").click();
    harness.run_steps(4);
    harness.get_by_label("Automatic checks run once a day. Setup opens only when you choose to update.");
    for label in ["Default install folder", "Installing on Windows", "Install new apps as", "Automatically download and install updates"] {
        assert!(harness.query_by_label(label).is_none(), "unused component option remains visible: {label}");
    }
}
