//! Software updates, background check at startup, and startup prompt modal.

use serde_json::Value;

use crate::LocalvectorApp;

use craft_update::updater::{Action, Updater};
use craft_update::{CheckResultDialog, CheckResultResponse};

pub(crate) struct Runtime {
    updater: Updater,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    startup_check: bool,
    check_requested: bool,
    /// The context of the last frame, to wake the UI when a check is queued from a menu callback.
    repaint: Option<egui::Context>,
    /// Whether the result of Help ▸ Check for Updates is in view.
    pub(crate) manual_window_open: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            updater: Updater::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")),
            startup_check: false,
            check_requested: false,
            repaint: None,
            manual_window_open: false,
        }
    }
}

impl LocalvectorApp {
    /// Whether the app asks for a newer release when it starts (off until the desktop app turns it on).
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

pub(crate) fn check(app: &mut LocalvectorApp) -> Result<Value, String> {
    {
        app.updates_rt.check_requested = true;
        app.updates_rt.manual_window_open = true;
        if let Some(ctx) = &app.updates_rt.repaint {
            ctx.request_repaint();
        }
        Ok(Value::Null)
    }
}

pub(crate) fn tick(app: &mut LocalvectorApp, ctx: &egui::Context) {
    {
        let rt = &mut app.updates_rt;
        rt.repaint = Some(ctx.clone());
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        rt.updater.tick(ctx);
    }
}

/// Test-only: offer a release the way a completed check would, and report whether the prompt is
/// up (the update card the width guard checks).
#[cfg(test)]
pub(crate) fn offer_for_test(app: &mut LocalvectorApp, release: craft_update::Release) -> bool {
    app.updates_rt.updater.offer(release);
    app.updates_rt.updater.prompt_open()
}

pub(crate) fn show(app: &mut LocalvectorApp, ctx: &egui::Context) {
    {
        // The restart helper is scheduled before the app closes, so no update starts over unsaved work.
        let clean = !crate::unsaved::any_dirty(app);
        match app.updates_rt.updater.show_if(ctx, clean) {
            Action::OpenUrl(url) => app.open_url(&url),
            Action::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            Action::None => {}
        }
        // An available update goes to the prompt above, which replaces the result dialog.
        let rt = &mut app.updates_rt;
        if rt.manual_window_open && rt.updater.prompt_open() {
            rt.manual_window_open = false;
        }
        if rt.manual_window_open {
            match CheckResultDialog::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")).show(ctx, rt.updater.status()) {
                CheckResultResponse::None => {}
                CheckResultResponse::Close => rt.manual_window_open = false,
                CheckResultResponse::CheckAgain => rt.updater.check_now(ctx),
            }
        }
        // Closing the dialog dismisses the outcome it showed.
        if !rt.manual_window_open {
            rt.updater.dismiss_status();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use craft_update::updater::Status;
    use localvector_engine::Session;

    #[test]
    fn default_runtime_is_idle() {
        let rt = Runtime::default();
        assert!(!rt.manual_window_open);
        {
            assert!(!rt.startup_check);
            assert!(!rt.updater.prompt_open());
            assert_eq!(rt.updater.status(), &Status::Idle);
        }
    }

    #[test]
    fn startup_prompt_shows_and_closes() {
        let mut app = LocalvectorApp::new(Session::new(), Default::default());
        let ctx = egui::Context::default();
        let release = craft_update::Release {
            tag: "v99.0.0".into(),
            version: craft_update::parse_version("99.0.0"),
            name: "Local Vector 99.0.0".into(),
            html_url: "https://github.com/Styx-Studios/Local-Vector/releases/tag/v99.0.0".into(),
            ..Default::default()
        };
        app.updates_rt.updater.offer(release);

        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            show(&mut app, ui.ctx());
        });
        output.textures_delta.clear();
        assert!(app.updates_rt.updater.prompt_open());
    }
}
