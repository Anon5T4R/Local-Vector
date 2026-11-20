//! The command palette (the search icon of the application bar, Help › Search Commands…): every command and tool,
//! in the suite's `craft-palette` box. It lists what the menus list (and the commands that have no menu), with the
//! menu path as the group, the shortcut and the enabled state the menu shows, and runs the pick through the same
//! code as the menu ([`menus::invoke`]).

use std::sync::Arc;

use craft_palette::{Command, Labels};
use serde_json::json;

use crate::{LocalvectorApp, menus};

/// Everything the palette lists: (label with its menu path, command id or `tool:<id>`, shortcut).
pub fn items() -> Vec<(String, String, String)> {
    let mut items = vec![];
    // Aliases kept for older scripts run a command that is listed already.
    for c in localvector_engine::command_specs().iter().filter(|c| !localvector_engine::cmd::is_alias(c.id)) {
        let path = c.menu.join(" › ");
        let label = if path.is_empty() { c.label.to_string() } else { format!("{path} › {}", c.label) };
        items.push((label, c.id.to_string(), menus::shortcut_of(c.id).unwrap_or("").to_string()));
    }
    // UI Brightness takes the level as a parameter, which a pick cannot give: each level is listed on its own
    // ([`BRIGHTNESS`]).
    for c in menus::UI_COMMANDS.iter().filter(|c| c.0 != "window.brightness") {
        items.push((c.1.to_string(), c.0.to_string(), menus::shortcut_of(c.0).unwrap_or("").to_string()));
    }
    items.extend(brightness_levels().map(|(label, id)| (label, id, String::new())));
    for tool in localvector_tools::catalog::all_tools() {
        items.push((tool.label.to_string(), format!("tool:{}", tool.id), crate::shortcut_editor::tool_shortcut(tool.id).unwrap_or("").to_string()));
    }
    items
}

/// Where the menus keep each command that takes no parameters: id → menu path.
fn menu_paths() -> std::collections::HashMap<&'static str, String> {
    fn walk(path: &[&'static str], items: &[menus::Item], out: &mut std::collections::HashMap<&'static str, String>) {
        for item in items {
            match item {
                menus::Item::Cmd(_, id, params) if params.is_null() || params.as_object().is_some_and(serde_json::Map::is_empty) => {
                    out.entry(*id).or_insert_with(|| path.join(" › "));
                }
                menus::Item::Sub(name, children) => {
                    let mut deeper = path.to_vec();
                    deeper.push(*name);
                    walk(&deeper, children, out);
                }
                _ => {}
            }
        }
    }
    let mut out = std::collections::HashMap::new();
    for (title, items) in menus::menu_tree() {
        walk(&[title], &items, &mut out);
    }
    out
}

/// The id prefix of a palette entry that sets the UI brightness (`window.brightness:<level>`).
const BRIGHTNESS: &str = "window.brightness:";

/// Window › UI Brightness: Automatic, then the four levels, as (label with its path, palette id).
fn brightness_levels() -> impl Iterator<Item = (String, String)> {
    let path = "Window › UI Brightness › ";
    std::iter::once((format!("{path}Automatic"), format!("{BRIGHTNESS}{}", crate::theme::Brightness::SYSTEM_ID)))
        .chain(crate::theme::Brightness::ALL.into_iter().map(move |b| (format!("{path}{}", b.label()), format!("{BRIGHTNESS}{}", b.id()))))
}

/// A palette label in `lang`: each `›`-separated menu segment and the command label translate on
/// their own.
fn shown_label(lang: crate::i18n::Lang, label: &str) -> String {
    label.split(" › ").map(|s| crate::i18n::tr(lang, s)).collect::<Vec<_>>().join(" › ")
}

/// What the entries depend on: the language, the installed plug-ins and the shortcuts.
fn cache_key(lang: crate::i18n::Lang) -> (&'static str, u64, u64) {
    (lang.code(), menus::plugin_revision(), crate::shortcut_editor::GENERATION.load(std::sync::atomic::Ordering::Relaxed))
}

/// The palette's commands in `lang`, every one enabled. They are built when the language, the installed plug-ins or
/// the keyboard shortcuts (shown beside each entry) change, not on every frame while a query is typed.
fn entries(ctx: &egui::Context, lang: crate::i18n::Lang) -> Arc<Vec<Command>> {
    type Cached = ((&'static str, u64, u64), Arc<Vec<Command>>);
    let key = cache_key(lang);
    let id = egui::Id::unique("palette-entries");
    if let Some((k, v)) = ctx.data(|d| d.get_temp::<Cached>(id))
        && k == key
    {
        return v;
    }
    let in_menus = menu_paths();
    let built: Arc<Vec<Command>> = Arc::new(
        items()
            .into_iter()
            .map(|(label, id, shortcut)| {
                // The last segment is the command, the ones before it the menu it lives in; a command with no path of
                // its own is listed under the menu that has it, when one does.
                let shown = match (label.contains(" › "), in_menus.get(id.as_str())) {
                    (false, Some(path)) if !path.is_empty() => format!("{path} › {label}"),
                    _ => label,
                };
                let shown = shown_label(lang, &shown);
                let (group, name) = shown.rsplit_once(" › ").unwrap_or(("", shown.as_str()));
                let command = Command::new(id, name.trim_end_matches('…').to_string(), group.to_string());
                if shortcut.is_empty() { command } else { command.shortcut(menus::shown_shortcut(&shortcut)) }
            })
            .collect(),
    );
    ctx.data_mut(|d| d.insert_temp::<Cached>(id, (key, built.clone())));
    built
}

/// The commands the palette lists now: [`entries`] with the menu's enabled state (a disabled command is listed, so the
/// user learns it exists, but does not run).
pub(crate) fn commands(app: &LocalvectorApp, ctx: &egui::Context, lang: crate::i18n::Lang) -> Vec<Command> {
    entries(ctx, lang)
        .iter()
        .map(|c| {
            let enabled = c.id.starts_with("tool:") || c.id.starts_with(BRIGHTNESS) || menus::enabled(app, &c.id);
            Command { enabled, ..c.clone() }
        })
        .collect()
}

/// The palette, when open: runs what the user picked.
pub fn show(app: &mut LocalvectorApp, ctx: &egui::Context) {
    // `ui.palette_open` is the flag the menus, the shortcuts and the control channel set; the box follows it.
    if app.ui.palette_open != app.palette_box.is_open() {
        if app.ui.palette_open {
            app.palette_box.open();
        } else {
            app.palette_box.close();
        }
    }
    if !app.ui.palette_open {
        return;
    }
    let list = commands(app, ctx, crate::i18n::current());
    let labels = Labels { placeholder: tl!("Search commands and tools…"), empty: tl!("No matching commands") };
    let ran = app.palette_box.show(ctx, &labels, &list);
    app.ui.palette_open = app.palette_box.is_open();
    let Some(id) = ran else { return };
    if let Some(tool) = id.strip_prefix("tool:") {
        app.select_tool(tool);
    } else if let Some(level) = id.strip_prefix(BRIGHTNESS) {
        menus::invoke(app, "window.brightness", json!({ "brightness": level }));
    } else {
        menus::invoke(app, &id, json!({}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn entries_are_built_once_per_language_and_translate_the_label_and_the_menu() {
        let ctx = egui::Context::default();
        let zh = Lang::from_code("zh-hant").unwrap();
        // Tests running alongside edit shortcuts and install plug-ins, which rebuilds the entries:
        // look twice until nothing changed in between.
        let (en, again) = (0..100)
            .find_map(|_| {
                let key = cache_key(Lang::EN);
                let pair = (entries(&ctx, Lang::EN), entries(&ctx, Lang::EN));
                (cache_key(Lang::EN) == key).then_some(pair)
            })
            .unwrap();
        assert!(Arc::ptr_eq(&en, &again), "kept while the language stays");
        let zh_entries = entries(&ctx, zh);
        assert!(!Arc::ptr_eq(&en, &zh_entries), "rebuilt for another language");
        // Editing a shortcut rebuilds them, so the palette shows the new one.
        crate::shortcut_editor::GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        assert!(!Arc::ptr_eq(&zh_entries, &entries(&ctx, zh)), "rebuilt after a shortcut edit");
        let group = zh_entries.iter().find(|e| e.id == "object.group").unwrap();
        assert_eq!(group.label, crate::i18n::tr(zh, "Group"), "the label is the command's, in the UI language");
        assert_eq!(group.group, crate::i18n::tr(zh, "Object"), "the group is the menu's, in the UI language");
        // A query finds it by the label as shown.
        assert!(craft_palette::filter(crate::i18n::tr(zh, "Group"), &zh_entries).iter().any(|&i| zh_entries[i].id == "object.group"));
    }

    /// Searching for the preference's name, Use Precise Cursors, finds the command that toggles it,
    /// and choosing the first result changes the preference.
    #[test]
    fn use_precise_cursors_palette_result_toggles_the_preference() {
        let mut app = LocalvectorApp::new(localvector_engine::Session::new(), crate::Services::default());
        let ctx = egui::Context::default();
        let list = commands(&app, &ctx, Lang::EN);
        let found = craft_palette::filter("use precise cursors", &list);
        assert!(found.iter().any(|&i| list[i].id == "view.preciseCursors"), "preference toggle appears for its name");
        assert!(list.iter().filter(|e| e.id == "view.capsLock").all(|e| e.label != "Use Precise Cursors"));
        let before = app.session.prefs.use_precise_cursors;
        let first = &list[*found.first().expect("query has a result")];
        menus::invoke(&mut app, &first.id.clone(), json!({}));
        assert_ne!(app.session.prefs.use_precise_cursors, before, "first result toggles the preference");
    }

    /// The palette lists `select.key` once, as Cancel Key Object (#541): a pick runs it with no id,
    /// which cancels the key object. `engine.commands` gives agents the same label.
    #[test]
    fn the_palette_lists_cancel_key_object_once() {
        let items = items();
        let ids: Vec<&str> = items.iter().filter(|(label, ..)| label == "Cancel Key Object").map(|(_, id, _)| id.as_str()).collect();
        assert_eq!(ids, ["select.key"]);
        let mut app = LocalvectorApp::new(localvector_engine::Session::new(), crate::Services::default());
        let commands = crate::control::all_commands(&app);
        let key = commands.as_array().unwrap().iter().find(|c| c["id"] == "select.key").unwrap();
        assert_eq!(key["label"], "Cancel Key Object");
        app.run("file.new", json!({"width": 200, "height": 200})).unwrap();
        let a = app.run("shape.rectangle", json!({"x": 10, "y": 10, "width": 10, "height": 10})).unwrap()["id"].clone();
        let b = app.run("shape.rectangle", json!({"x": 80, "y": 50, "width": 30, "height": 30})).unwrap()["id"].clone();
        app.run("select.set", json!({"ids": [a, b]})).unwrap();
        app.run("select.key", json!({"id": b})).unwrap();
        assert!(app.session.active().unwrap().selection.key.is_some());
        // Run a pick the way `show` runs it.
        menus::invoke(&mut app, ids[0], json!({}));
        let st = app.session.active().unwrap();
        assert_eq!((st.selection.len(), st.selection.key), (2, None), "both objects stay selected, without a key");
    }

    /// #4864: a pick of an Asset Export command, which the palette runs with no params, opens the
    /// Asset Export panel instead of failing on the missing params or exporting nowhere.
    #[test]
    fn a_pick_of_an_asset_export_command_opens_its_panel() {
        let ids = ["assets.export", "assets.remove", "assets.rename", "assets.settings.set"];
        for id in ids {
            assert!(items().iter().any(|(_, item, _)| item == id), "{id} is in the palette");
            let mut app = LocalvectorApp::new(localvector_engine::Session::new(), crate::Services::default());
            app.run("file.new", json!({"width": 200, "height": 200})).unwrap();
            app.ui.status.clear();
            assert!(!crate::panel_docking::showing(&app.ui, crate::panels::asset_export::ID), "{id}");
            // Run a pick the way `show` runs it.
            menus::invoke(&mut app, id, json!({}));
            assert!(crate::panel_docking::showing(&app.ui, crate::panels::asset_export::ID), "{id}: {}", app.ui.status);
            assert!(!app.ui.status.contains("invalid parameters"), "{id}: {}", app.ui.status);
            // A second pick leaves the open panel open, docked or floating.
            menus::invoke(&mut app, id, json!({}));
            assert!(crate::panel_docking::showing(&app.ui, crate::panels::asset_export::ID), "{id}: picked twice");
            app.run("window.panel.float", json!({"panel": crate::panels::asset_export::ID})).unwrap();
            assert!(crate::panel_docking::showing(&app.ui, crate::panels::asset_export::ID), "{id}: floating");
            menus::invoke(&mut app, id, json!({}));
            assert!(crate::panel_docking::showing(&app.ui, crate::panels::asset_export::ID), "{id}: floating, picked again");
        }
    }

    /// The palette's list: the menu's own enabled state and shortcut, the menu path as the group, a tool among them,
    /// and the UI brightness levels one by one (the command itself needs a level).
    #[test]
    fn the_palette_lists_the_menus_the_shortcut_and_the_enabled_state() {
        let app = LocalvectorApp::new(localvector_engine::Session::new(), crate::Services::default());
        let ctx = egui::Context::default();
        let list = commands(&app, &ctx, Lang::EN);
        let find = |id: &str| list.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id} is listed"));
        let settings = find("edit.preferences");
        assert_eq!((settings.label.as_str(), settings.group.as_str()), ("Settings", "Edit"));
        assert!(!settings.shortcut.is_empty(), "the menu's shortcut shows");
        assert!(settings.enabled);
        assert!(!find("file.save").enabled, "Save is disabled with no document, as in the menu");
        assert!(find("help.about").enabled && find("help.checkForUpdates").enabled);
        assert!(list.iter().any(|c| c.id.starts_with("tool:")), "the tools are listed");
        assert!(list.iter().all(|c| c.id != "window.brightness"), "the brightness command alone cannot run without a level");
        let levels: Vec<(&str, &str)> = list.iter().filter(|c| c.id.starts_with(BRIGHTNESS)).map(|c| (c.label.as_str(), c.group.as_str())).collect();
        assert_eq!(
            levels,
            [
                ("Automatic", "Window › UI Brightness"),
                ("Dark", "Window › UI Brightness"),
                ("Medium Dark", "Window › UI Brightness"),
                ("Medium Light", "Window › UI Brightness"),
                ("Light", "Window › UI Brightness"),
            ]
        );
    }

    /// The suite's box on a real frame: the search icon (or Help › Search Commands…) opens it, typing narrows it,
    /// Enter runs the first match through the menu's code, and Escape closes it.
    #[test]
    fn the_palette_opens_filters_runs_a_command_and_closes() {
        let mut app = LocalvectorApp::new(localvector_engine::Session::new(), crate::Services::default());
        let ctx = egui::Context::default();
        egui_extras::install_image_loaders(&ctx);
        crate::theme::install_fonts(&ctx);
        let key = |k| egui::Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::NONE };
        let frame = |app: &mut LocalvectorApp, events: Vec<egui::Event>| {
            ctx.run_ui(egui::RawInput { events, ..Default::default() }, |ui| show(app, ui.ctx())).textures_delta.clear();
        };
        frame(&mut app, vec![]);
        assert!(!app.ui.palette_open && !app.palette_box.is_open());
        app.run("help.commandPalette", json!({})).unwrap();
        assert!(app.ui.palette_open, "Help › Search Commands… opens it");
        frame(&mut app, vec![]);
        assert!(app.palette_box.is_open());
        frame(&mut app, vec![egui::Event::Text("about local".into())]);
        frame(&mut app, vec![key(egui::Key::Enter)]);
        assert!(app.ui.about && !app.ui.palette_open, "Enter ran Help › About and closed the box");
        app.ui.about = false;
        app.ui.palette_open = true;
        frame(&mut app, vec![]);
        frame(&mut app, vec![key(egui::Key::Escape)]);
        assert!(!app.ui.palette_open, "Escape closes it");
        // A level of the brightness is a palette entry of its own.
        app.ui.palette_open = true;
        frame(&mut app, vec![]);
        frame(&mut app, vec![egui::Event::Text("medium light".into())]);
        frame(&mut app, vec![key(egui::Key::Enter)]);
        assert_eq!(app.session.prefs.ui_brightness, "mediumLight");
    }
}
