//! The suite's shell in the application bar: the Settings gear, the theme button and the command palette's search
//! icon, drawn with the suite's `craft-icons`. What each does is the app's own command (Edit › Settings…, Window ›
//! UI Brightness, Help › Search Commands…), so the buttons, the menus and the control channel agree.
//!
//! The theme keeps the four brightness levels of the preference (`uiBrightness`: Dark, Medium Dark, Medium Light,
//! Light) over the suite's three modes: the button shows Automatic, Light or Dark and moves between them; Light and
//! Dark pick the level the system would (Light, Dark), and the four levels stay selectable in the menu and in
//! Settings.

use craft_icons::{Icon, ThemeMode};
use egui::{Response, Role, Ui, WidgetInfo};
use serde_json::json;

use crate::LocalvectorApp;
use crate::theme::Brightness;

/// The suite's mode a `uiBrightness` value stands for: "system" is Automatic, the two dark levels are Dark, the
/// two light levels are Light. An id this version does not know follows `current`, the brightness on screen.
pub fn theme_mode(ui_brightness: &str, current: Brightness) -> ThemeMode {
    if ui_brightness.eq_ignore_ascii_case(Brightness::SYSTEM_ID) {
        return ThemeMode::System;
    }
    match Brightness::parse(ui_brightness).unwrap_or(current) {
        Brightness::Dark | Brightness::MediumDark => ThemeMode::Dark,
        Brightness::MediumLight | Brightness::Light => ThemeMode::Light,
    }
}

/// The `uiBrightness` value a suite mode chooses: Automatic follows the system, Light and Dark are the levels the
/// system would pick.
pub fn brightness_id(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::System => Brightness::SYSTEM_ID,
        ThemeMode::Light => Brightness::Light.id(),
        ThemeMode::Dark => Brightness::Dark.id(),
    }
}

/// The mode the app shows now.
pub fn current_mode(app: &LocalvectorApp) -> ThemeMode {
    theme_mode(&app.session.prefs.ui_brightness, app.ui.brightness)
}

/// "Theme: Automatic": the theme button's tooltip, translated.
pub fn theme_tip(mode: ThemeMode) -> String {
    format!("{}: {}", tl!("Theme"), tl!(mode.label()))
}

/// "Settings (Ctrl+K)": the gear's tooltip, with the shortcut the user has for it.
pub fn settings_tip() -> String {
    with_shortcut(tl!("Settings"), "edit.preferences")
}

/// "Search commands (Ctrl+Shift+/)": the search icon's tooltip.
pub fn search_tip() -> String {
    with_shortcut(tl!("Search commands"), "help.commandPalette")
}

/// A label with the shortcut the user has for `command`: "Document Setup (Ctrl+Alt+P)".
pub(crate) fn with_shortcut(label: &str, command: &str) -> String {
    match crate::menus::shortcut_of(command) {
        Some(shortcut) => format!("{label} ({})", crate::menus::shown_shortcut(shortcut)),
        None => label.to_string(),
    }
}

/// One suite icon button: its tooltip is also the control's accessible name.
fn button(ui: &mut Ui, icon: Icon, tip: &str) -> Response {
    let response = craft_icons::ui::button(ui, icon, tip);
    response.widget_info(|| WidgetInfo::labeled(Role::Button, true, tip));
    response
}

/// The gear: opens Settings (Edit › Settings…) on User Interface, where the theme levels and the language are.
pub fn settings_button(app: &mut LocalvectorApp, ui: &mut Ui) {
    if button(ui, Icon::Settings, &settings_tip()).clicked() {
        app.run("edit.preferences", json!({"category": "User Interface"})).ok();
    }
}

/// The theme button: shows the mode and moves to the next (Automatic, Light, Dark). The levels are in Window › UI
/// Brightness and in Settings.
pub fn theme_button(app: &mut LocalvectorApp, ui: &mut Ui) {
    let mode = current_mode(app);
    if button(ui, mode.icon(), &theme_tip(mode)).clicked() {
        set_mode(app, mode.next());
    }
}

/// Chooses the level a suite mode stands for (the same command as Window › UI Brightness).
pub fn set_mode(app: &mut LocalvectorApp, mode: ThemeMode) {
    if let Err(e) = app.run("window.brightness", json!({"brightness": brightness_id(mode)})) {
        app.status(e);
    }
}

/// The search icon: opens the command palette (Help › Search Commands…).
pub fn search_button(app: &mut LocalvectorApp, ui: &mut Ui) {
    if button(ui, Icon::Search, &search_tip()).clicked() {
        app.run("help.commandPalette", json!({})).ok();
    }
}
