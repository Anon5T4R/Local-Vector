//! The suite's shell in the application bar: the Settings gear, the theme button and the command palette's search
//! icon, Window › UI Brightness with Automatic over the four levels, and Edit › Settings….

use craft_icons::ThemeMode;
use egui::{Key, Modifiers, vec2};
use egui_kittest::{Harness, kittest::Queryable};
use serde_json::json;

use crate::LocalvectorApp;
use crate::theme::Brightness;

fn harness() -> Harness<'static, LocalvectorApp> {
    sized(vec2(1280.0, 800.0))
}

fn sized(size: egui::Vec2) -> Harness<'static, LocalvectorApp> {
    let mut app = LocalvectorApp::new(localvector_engine::Session::new(), crate::Services::default());
    app.run("prefs.set", json!({"key": "interfaceLanguage", "value": "en"})).unwrap();
    app.run("prefs.set", json!({"key": "uiBrightness", "value": "system"})).unwrap();
    app.run("file.new", json!({"width": 200, "height": 200})).unwrap();
    let mut h = Harness::builder().with_size(size).with_max_steps(64).with_accessibility_check(false).build_ui_state(
        |ui, app: &mut LocalvectorApp| {
            app.begin_host_iteration();
            app.tick(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

fn level(h: &Harness<'_, LocalvectorApp>) -> String {
    h.state().session.prefs.ui_brightness.clone()
}

fn settings_open(h: &Harness<'_, LocalvectorApp>) -> bool {
    h.state().ui.dialog.as_ref().is_some_and(|d| d.kind == "preferences")
}

/// From the right edge: the gear, the theme button, the search icon, each 6 apart.
#[test]
fn the_bar_has_the_gear_the_theme_and_the_search_in_the_suites_order() {
    let h = harness();
    let gear = h.get_by_label_contains("Settings (").rect();
    let theme = h.get_by_label("Theme: Automatic").rect();
    let search = h.get_by_label_contains("Search commands (").rect();
    assert!(gear.left() > theme.right() && theme.left() > search.right(), "gear {gear:?}, theme {theme:?}, search {search:?}");
    assert!(1280.0 - gear.right() >= 8.0, "the gear is off the window's edge: {gear:?}");
    let gap = |a: egui::Rect, b: egui::Rect| a.left() - b.right();
    assert!((gap(gear, theme) - 6.0).abs() < 3.0 && (gap(theme, search) - 6.0).abs() < 3.0, "{gear:?} {theme:?} {search:?}");
    // The three icons share one row with the application bar's other controls.
    let home = h.get_by_label("Home").rect();
    assert!((gear.center().y - home.center().y).abs() < 4.0 && (theme.center().y - home.center().y).abs() < 4.0);
}

/// A bar too narrow for all three drops the theme first (Window › UI Brightness has it), then the search icon, then the
/// gear; the switcher narrows before any of that, and nothing is ever drawn without the one before it.
#[test]
fn a_narrow_bar_drops_the_theme_then_the_search_and_keeps_the_gear_longest() {
    let shown: Vec<(u32, [bool; 3])> = [1280, 800, 740, 720, 700, 680, 660, 640, 620, 600, 580, 560, 540, 500]
        .into_iter()
        .map(|hundreds| {
            let width = hundreds as f32;
            let h = sized(vec2(width, 600.0));
            let has = |label: &str| h.query_by_label_contains(label).is_some();
            (hundreds, [has("Settings ("), has("Search commands ("), has("Theme: ")])
        })
        .collect();
    for (width, [gear, search, theme]) in &shown {
        assert!(!theme || *search, "{width}: the theme button shows only with the search icon: {shown:?}");
        assert!(!search || *gear, "{width}: the search icon shows only with the gear: {shown:?}");
    }
    assert_eq!(shown[0].1, [true, true, true], "all three in a wide window: {shown:?}");
    assert!(shown.iter().any(|(_, s)| *s == [true, true, false]), "the theme goes first: {shown:?}");
}

/// The button goes Automatic, Light, Dark, Automatic; Light and Dark pick the levels the system would.
#[test]
fn the_theme_button_cycles_automatic_light_dark() {
    let mut h = harness();
    assert_eq!(level(&h), "system");
    for (label, next) in [("Theme: Automatic", "light"), ("Theme: Light", "dark"), ("Theme: Dark", "system")] {
        h.get_by_label(label).click();
        h.run_steps(3);
        assert_eq!(level(&h), next, "after {label}");
    }
    h.get_by_label("Theme: Automatic").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.brightness, Brightness::Light, "Light shows the light level");
    h.get_by_label("Theme: Light").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.brightness, Brightness::Dark, "Dark shows the dark level");
}

/// The four levels sit over the three modes: a medium level reads as its side, and the button leaves it for the
/// next mode.
#[test]
fn a_medium_level_shows_as_its_mode_and_the_button_moves_on() {
    let mut h = harness();
    let ctx = h.ctx.clone();
    for (id, label, next) in [("mediumLight", "Theme: Light", "dark"), ("mediumDark", "Theme: Dark", "system")] {
        h.state_mut().run("window.brightness", json!({"brightness": id})).unwrap();
        h.run_steps(3);
        assert_eq!(level(&h), id);
        h.get_by_label(label).click();
        h.run_steps(3);
        assert_eq!(level(&h), next, "{id}");
    }
    let _ = ctx;
}

/// An old or unknown `uiBrightness` never breaks the mode: it follows the level on screen.
#[test]
fn an_unknown_level_follows_the_brightness_on_screen() {
    assert_eq!(crate::shell::theme_mode("system", Brightness::Dark), ThemeMode::System);
    assert_eq!(crate::shell::theme_mode("SYSTEM", Brightness::Dark), ThemeMode::System);
    for (id, mode) in [("dark", ThemeMode::Dark), ("mediumDark", ThemeMode::Dark), ("mediumLight", ThemeMode::Light), ("light", ThemeMode::Light)] {
        assert_eq!(crate::shell::theme_mode(id, Brightness::MediumDark), mode, "{id}");
    }
    assert_eq!(crate::shell::theme_mode("sepia", Brightness::Light), ThemeMode::Light);
    assert_eq!(crate::shell::theme_mode("", Brightness::MediumDark), ThemeMode::Dark);
    for mode in ThemeMode::EVERY {
        assert_eq!(crate::shell::theme_mode(crate::shell::brightness_id(mode), Brightness::MediumDark), mode);
    }
}

/// Window › UI Brightness lists Automatic and the four levels, the current one checked, and runs each.
#[test]
fn window_ui_brightness_offers_automatic_and_the_four_levels() {
    let mut h = harness();
    let entries = crate::menus::menu_entries(h.state());
    let rows: Vec<(String, String)> = entries
        .iter()
        .filter(|e| e.path == ["Window", "UI Brightness"])
        .map(|e| (e.label.clone(), e.params["brightness"].as_str().unwrap_or_default().to_string()))
        .collect();
    assert_eq!(
        rows,
        [("Automatic", "system"), ("Dark", "dark"), ("Medium Dark", "mediumDark"), ("Medium Light", "mediumLight"), ("Light", "light")]
            .map(|(a, b)| (a.to_string(), b.to_string()))
    );
    for (_, id) in &rows {
        crate::menus::invoke(h.state_mut(), "window.brightness", json!({"brightness": id}));
        h.run_steps(2);
        assert_eq!(&level(&h), id);
        let checked: Vec<Option<bool>> =
            rows.iter().map(|(_, other)| crate::menus::checked(h.state(), "window.brightness", &json!({"brightness": other}))).collect();
        assert_eq!(checked.iter().filter(|c| **c == Some(true)).count(), 1, "{id}: {checked:?}");
    }
}

/// The gear opens Settings, where the brightness levels and the language are.
#[test]
fn the_gear_opens_settings_on_the_first_page() {
    let mut h = harness();
    assert!(!settings_open(&h));
    h.get_by_label_contains("Settings (").click();
    h.run_steps(4);
    assert!(settings_open(&h));
    assert_eq!(
        h.state().ui.dialog.as_ref().map(|d| d.str("__category")),
        Some("User Interface".to_string()),
        "the page with the theme and the language"
    );
    assert!(h.query_all_by_label("Settings").count() >= 2, "the screen is called Settings (its heading and the bar's button)");
    assert!(h.query_by_label_contains("Preferences").is_none(), "no Preferences left on screen");
}

/// Edit › Settings… (Ctrl+K) opens the same screen; no menu is still called Preferences.
#[test]
fn edit_settings_opens_settings_and_ctrl_k_stays_the_settings_shortcut() {
    let mut h = harness();
    let entries = crate::menus::menu_entries(h.state());
    let settings = entries.iter().find(|e| e.command.as_deref() == Some("edit.preferences")).unwrap();
    assert_eq!((settings.label.as_str(), settings.path.as_slice()), ("Settings…", ["Edit".to_string()].as_slice()));
    assert_eq!(settings.shortcut, "Cmd+K");
    assert!(entries.iter().all(|e| !e.label.starts_with("Preferences") && !e.path.iter().any(|p| p == "Preferences")));
    h.key_press_modifiers(Modifiers::COMMAND, Key::K);
    h.run_steps(4);
    assert!(settings_open(&h), "Ctrl+K opens Settings");
    assert!(!h.state().ui.palette_open, "the palette opens from the search icon (or Help › Search Commands…)");
}

/// The search icon opens the palette with the menus' commands; typing narrows it and Enter runs the command through
/// the menu's own code.
#[test]
fn the_search_icon_opens_the_palette_and_runs_a_command() {
    let mut h = harness();
    assert!(!h.state().ui.palette_open);
    h.get_by_label_contains("Search commands (").click();
    h.run_steps(3);
    assert!(h.state().ui.palette_open && h.state().palette_box.is_open());
    h.event(egui::Event::Text("about local".into()));
    h.run_steps(3);
    h.key_press(Key::Enter);
    h.run_steps(4);
    assert!(!h.state().ui.palette_open, "running a command closes the palette");
    assert!(h.state().ui.about, "About ran as the Help menu item does");
}

/// Ctrl+Shift+/ (Help › Search Commands…) keeps opening it, and Escape closes it.
#[test]
fn the_palette_keeps_its_own_shortcut_and_escape_closes_it() {
    let mut h = harness();
    h.key_press_modifiers(Modifiers::COMMAND | Modifiers::SHIFT, Key::Slash);
    h.run_steps(4);
    assert!(h.state().ui.palette_open, "Cmd+Shift+/ opens the palette");
    h.key_press(Key::Escape);
    h.run_steps(3);
    assert!(!h.state().ui.palette_open, "Escape closes it");
}

/// A command the menu disables is listed (so it is learned) and does not run, as in the menu.
#[test]
fn a_command_the_menu_disables_does_not_run_from_the_palette() {
    let h = harness();
    let list = crate::palette::commands(h.state(), &h.ctx, crate::i18n::Lang::EN);
    let disabled = list.iter().find(|c| !c.enabled && !c.id.starts_with("tool:")).expect("a command the menu disables");
    assert!(!crate::menus::enabled(h.state(), &disabled.id));
}

/// Screenshots of the window for review: `cargo test -p localvector-ui-egui casca_shots -- --ignored`. They land in
/// `target/qa-casca/`.
#[test]
#[ignore = "QA: writes PNGs of the window to target/qa-casca"]
fn casca_shots() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/qa-casca");
    std::fs::create_dir_all(&directory).unwrap();
    let shot = |h: &mut Harness<'_, LocalvectorApp>, name: &str| {
        h.run_steps(4);
        let image = h.render().unwrap();
        image.save(directory.join(format!("{name}.png"))).unwrap();
    };
    let mut h = harness();
    let set = |h: &mut Harness<'_, LocalvectorApp>, id: &str| {
        h.state_mut().run("window.brightness", json!({"brightness": id})).unwrap();
        h.run_steps(3);
    };
    set(&mut h, "light");
    shot(&mut h, "barra-claro");
    set(&mut h, "dark");
    shot(&mut h, "barra-escuro");
    crate::menus::invoke(h.state_mut(), "edit.preferences", json!({"category": "User Interface"}));
    shot(&mut h, "configuracoes-escuro");
    h.state_mut().ui.dialog = None;
    set(&mut h, "mediumLight");
    h.state_mut().ui.palette_open = true;
    h.run_steps(3);
    h.event(egui::Event::Text("sett".into()));
    shot(&mut h, "paleta-claro");
    h.state_mut().ui.palette_open = false;
    set(&mut h, "mediumDark");
    h.state_mut().ui.palette_open = true;
    h.run_steps(3);
    shot(&mut h, "paleta-escuro");
}
