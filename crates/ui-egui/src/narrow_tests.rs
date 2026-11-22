//! The kittest size guard: with a document and art on screen, every dialog and window of the app
//! fits the main window's minimum size (`window::MIN_SIZE`), and so do the application bar, the
//! command palette and the update card.

use egui::accesskit::Role;
use egui_kittest::Harness;
use egui_kittest::kittest::NodeT;
use serde_json::json;

use crate::LocalvectorApp;

/// The main window's minimum width and height (`apps/localvector/src/window.rs`).
const MIN_WIDTH: f32 = 800.0;
const MIN_HEIGHT: f32 = 500.0;

/// An app with a document and a shape selected: real content on the canvas.
fn app_with_art() -> LocalvectorApp {
    let mut app = LocalvectorApp::new(localvector_engine::Session::new(), crate::Services::default());
    app.run("prefs.set", json!({"key": "interfaceLanguage", "value": "en"})).unwrap();
    app.run("file.new", json!({"width": 200, "height": 200})).unwrap();
    app.run("shape.rectangle", json!({"x": 10, "y": 10, "width": 80, "height": 60})).unwrap();
    app
}

/// A harness at `width` running the app's real frame (bar, panels, canvas, dialogs), with `open`
/// applied before the first step so the dialog is there from the start.
fn harness_at(width: f32, open: impl FnOnce(&mut LocalvectorApp)) -> Harness<'static, LocalvectorApp> {
    let mut app = app_with_art();
    open(&mut app);
    let mut fonts = false;
    let mut h = Harness::builder()
        .with_size(egui::vec2(width, MIN_HEIGHT))
        .with_max_steps(64)
        .with_accessibility_check(cfg!(not(debug_assertions)) || std::env::var_os("NARROW_DEBUG").is_none())
        .build_ui_state(
            move |ui, app: &mut LocalvectorApp| {
                if !std::mem::replace(&mut fonts, true) {
                    crate::theme::install_fonts(ui.ctx());
                }
                app.begin_host_iteration();
                app.tick(ui.ctx());
                app.ui(ui);
            },
            app,
        );
    h.run_steps(4);
    h
}

/// Where a window's content may go past the bottom edge: the body of a dialog whose controls do not
/// fit scrolls, with its title and its buttons outside the `ScrollArea`. The accesskit tree reports
/// the scrolled content unclipped, so what hangs below is accepted only when it comes before a
/// vertical scroll bar inside the window and in its column; the bar is drawn after the content and
/// the buttons after it.
fn below_is_scrolled(harness: &Harness<'_, LocalvectorApp>) -> impl Fn(usize, egui::accesskit::Rect) -> bool {
    const TOL: f64 = 0.5;
    let nodes: Vec<_> = harness.root().children_recursive().collect();
    let bars: Vec<(usize, egui::accesskit::Rect)> = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| {
            let rect = node.accesskit_node().bounding_box()?;
            let vertical = rect.y1 - rect.y0 > rect.x1 - rect.x0;
            let inside = rect.x0 >= -TOL && rect.x1 <= f64::from(MIN_WIDTH) + TOL && rect.y0 >= -TOL && rect.y1 <= f64::from(MIN_HEIGHT) + TOL;
            (node.accesskit_node().role() == Role::ScrollBar && vertical && inside).then_some((i, rect))
        })
        .collect();
    move |i, rect| bars.iter().any(|&(bar_at, bar)| bar_at > i && rect.x1 <= bar.x1 + TOL && bar.y0 <= rect.y0 + TOL)
}

/// Every node, in accesskit coordinates, inside the window of `width`×[`MIN_HEIGHT`] points. A text
/// run may report the galley's full width before the clip egui paints; the field's node is the
/// guard.
fn assert_inside(harness: &Harness<'_, LocalvectorApp>, what: &str, width: f32) {
    let scrolled = below_is_scrolled(harness);
    let (w, h) = (f64::from(width), f64::from(MIN_HEIGHT));
    for (i, node) in harness.root().children_recursive().enumerate() {
        if node.accesskit_node().role() == Role::TextRun {
            continue;
        }
        let Some(rect) = node.accesskit_node().bounding_box() else { continue };
        if rect.width() <= 0.0 {
            continue;
        }
        let role = node.accesskit_node().role();
        assert!(rect.x0 >= -0.5 && rect.x1 <= w + 0.5, "{what}: {role:?} outside the {width}-point window: {rect:?}");
        assert!(rect.y0 >= -0.5, "{what}: {role:?} above the window: {rect:?}");
        if rect.y1 > h + 0.5 && !scrolled(i, rect) {
            panic!("{what}: {role:?} below the {MIN_HEIGHT}-point window and outside a scrolling body: {rect:?}");
        }
    }
}

/// The menu commands that open a parameter dialog of their own (`menus::invoke`'s `menu_dialog`).
const MENU_DIALOGS: &[&str] = &[
    "object.move",
    "object.rotate",
    "object.scale",
    "object.reflect",
    "object.shear",
    "object.transformEach",
    "path.average",
    "path.averageJoin",
    "object.path.offsetPath",
    "object.path.simplify",
    "object.path.smooth",
    "object.path.splitIntoGrid",
    "object.vectorHalftone",
    "artboard.rearrange",
];

/// The dialog kinds drawn as their own window by the registry (their modules build the fields).
const WINDOW_KINDS: &[&str] = &["preferences", "shortcuts", "newWorkspace", "manageWorkspaces", "findFont"];

/// Every dialog the app can open with a document and a selection on screen fits the minimum
/// width: the UI commands that open one (run each; the open dialog is the witness), the menu
/// parameter dialogs, the registry's own windows, the palette and the update card.
#[test]
fn every_dialog_fits_the_narrow_window() {
    let mut checked = 0usize;
    // One dialog's failure is reported with the rest instead of hiding them.
    let mut problems: Vec<String> = Vec::new();
    let failing = |problems: &mut Vec<String>, what: &str, outcome: std::thread::Result<()>| {
        if let Err(panic) = outcome {
            let message = panic.downcast_ref::<String>().map(String::as_str).or(panic.downcast_ref::<&str>().copied()).unwrap_or("?");
            problems.push(format!("{what}: {message}"));
        }
    };
    // UI commands: the ones that open a dialog leave it in `ui.dialog`.
    for command in crate::menus::UI_COMMANDS.iter() {
        let mut seen_dialog = false;
        #[cfg(debug_assertions)]
        if std::env::var_os("NARROW_DEBUG").is_some() {
            eprintln!("dialog: {}", command.0);
        }
        let h = harness_at(MIN_WIDTH, |app| {
            let _ = app.run(command.0, json!({}));
            seen_dialog = app.ui.dialog.is_some();
        });
        if seen_dialog {
            #[cfg(debug_assertions)]
            if std::env::var_os("NARROW_DEBUG").is_some() {
                let mut outside = 0;
                for node in h.root().children_recursive() {
                    if let Some(rect) = node.accesskit_node().bounding_box()
                        && rect.x1 > f64::from(MIN_WIDTH) + 0.5
                    {
                        outside += 1;
                        eprintln!("{command:?}: WIDTH {:?} {rect:?}", node.accesskit_node().role());
                    }
                }
                let unnamed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| h.check_accessibility()));
                if let Err(e) = unnamed
                    && let Some(msg) = e.downcast_ref::<String>()
                {
                    eprintln!("{command:?}: UNNAMED {msg}");
                }
                if outside == 0 {
                    eprintln!("{command:?}: ok");
                }
                continue;
            }
            failing(&mut problems, command.0, std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| assert_inside(&h, command.0, MIN_WIDTH))));
            checked += 1;
        }
    }
    // Menu commands whose parameter dialog `menus::invoke` opens.
    for id in MENU_DIALOGS {
        let mut seen_dialog = false;
        let h = harness_at(MIN_WIDTH, |app| {
            crate::menus::invoke(app, id, json!({}));
            seen_dialog = app.ui.dialog.is_some();
        });
        assert!(seen_dialog, "{id} opens its dialog");
        failing(&mut problems, id, std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| assert_inside(&h, id, MIN_WIDTH))));
        checked += 1;
    }
    // The registry's window dialogs, opened as the app's own code opens them.
    for kind in WINDOW_KINDS {
        let h = harness_at(MIN_WIDTH, |app| {
            app.ui.dialog = Some(crate::state::Dialog { kind: (*kind).into(), fields: serde_json::Map::new() });
        });
        assert!(h.state().ui.dialog.is_some(), "{kind} stays open");
        failing(&mut problems, kind, std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| assert_inside(&h, kind, MIN_WIDTH))));
        checked += 1;
    }
    #[cfg(debug_assertions)]
    if std::env::var_os("NARROW_DEBUG").is_some() {
        for command in crate::menus::UI_COMMANDS.iter() {
            let mut seen = false;
            let h = harness_at(MIN_WIDTH, |app| {
                let _ = app.run(command.0, json!({}));
                seen = app.ui.dialog.is_some();
            });
            if !seen {
                continue;
            }
            for node in h.root().children_recursive() {
                if let Some(rect) = node.accesskit_node().bounding_box()
                    && rect.x1 > f64::from(MIN_WIDTH) + 0.5
                {
                    eprintln!("{command:?}: {:?} {rect:?}", node.accesskit_node().role());
                }
            }
        }
    }
    // The command palette (craft-palette) and the update card.
    let h = harness_at(MIN_WIDTH, |app| {
        app.ui.palette_open = true;
    });
    assert!(h.state().ui.palette_open, "the palette is open");
    failing(&mut problems, "palette", std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| assert_inside(&h, "palette", MIN_WIDTH))));
    checked += 1;

    let release = craft_update::Release {
        tag: "v99.0.0".into(),
        version: craft_update::parse_version("99.0.0"),
        name: "Local Vector 99.0.0".into(),
        html_url: "https://github.com/Anon5T4R/Local-Vector/releases/tag/v99.0.0".into(),
        ..Default::default()
    };
    let mut h = harness_at(MIN_WIDTH, |app| {
        assert!(crate::updates_ui::offer_for_test(app, release), "the update card is up");
    });
    h.run_steps(4);
    failing(&mut problems, "update-card", std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| assert_inside(&h, "update-card", MIN_WIDTH))));
    checked += 1;
    assert!(problems.is_empty(), "{} diálogo(s) fora do padrão:\n{}", problems.len(), problems.join("\n"));
    assert!(checked > 40, "{checked} dialogs checked: the sweep found the app's dialogs");
}

/// The application bar itself (and the screen under it) fits the minimum width.
#[test]
fn the_application_bar_fits_the_narrow_window() {
    let h = harness_at(MIN_WIDTH, |_| {});
    assert_inside(&h, "app-bar", MIN_WIDTH);
}

/// Debug helper: dump the top-band widgets with the Control bar on.
#[test]
#[ignore = "debug dump"]
fn debug_control_bar_top() {
    let mut app = app_with_art();
    app.run("window.control", json!({})).unwrap();
    let mut fonts = false;
    let mut h =
        egui_kittest::Harness::builder().with_size(egui::vec2(MIN_WIDTH, 700.0)).with_max_steps(64).with_accessibility_check(false).build_ui_state(
            move |ui, app: &mut crate::LocalvectorApp| {
                if !std::mem::replace(&mut fonts, true) {
                    crate::theme::install_fonts(ui.ctx());
                }
                app.begin_host_iteration();
                app.tick(ui.ctx());
                app.ui(ui);
            },
            app,
        );
    h.run_steps(4);
    for node in h.root().children_recursive() {
        if let Some(rect) = node.accesskit_node().bounding_box()
            && rect.y0 < 130.0
            && rect.width() > 0.0
            && node.accesskit_node().role() != egui::accesskit::Role::TextRun
        {
            eprintln!("{:?} {:?} label={:?}", node.accesskit_node().role(), rect, node.accesskit_node().label());
        }
    }
}
