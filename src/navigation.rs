use glib::clone;
use gtk::glib;
use gtk::{gio::ActionEntry, prelude::*};
use gtk4 as gtk;

pub fn handle_navigation(
    app: &gtk::Application,
    window: &gtk::ApplicationWindow,
    notebook: &gtk::Notebook,
) {
    let action_close = ActionEntry::builder("esc_close")
        .activate(|window: &gtk::ApplicationWindow, _, _| {
            window.close();
        })
        .build();

    let action_prev_tab = ActionEntry::builder("prev_tab")
        .activate(
            clone!(@strong notebook => move |_: &gtk::ApplicationWindow, _, _| {
                let current = notebook.current_page().unwrap_or(0);
                if current != 0 {
                    notebook.set_current_page(Some(current-1));
                }
            }),
        )
        .build();

    let action_next_tab = ActionEntry::builder("next_tab")
        .activate(
            clone!(@strong notebook => move |_: &gtk::ApplicationWindow, _, _| {
                let current = notebook.current_page().unwrap_or(0);
                let pages = notebook.n_pages();
                if current != pages {
                    notebook.set_current_page(Some(current+1));
                }
            }),
        )
        .build();

    window.add_action_entries([action_close, action_prev_tab, action_next_tab]);

    // close on escape
    app.set_accels_for_action("win.esc_close", &["Escape"]);

    // toggle tabs on tab/shift+tab
    app.set_accels_for_action("win.next_tab", &["Tab"]);
    app.set_accels_for_action("win.prev_tab", &["<Shift>Tab"]);

    // TODO: arrow keys
}
