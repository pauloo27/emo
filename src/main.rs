mod emoji_btn;
mod emojis;
mod groups;
mod navigation;
mod search;

use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::process;
use std::rc::Rc;

const APP_ID: &str = "cafe.ndo.Emo";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    load_global_css();

    let search_entry = gtk::Entry::builder()
        .placeholder_text("Search here")
        .primary_icon_name("system-search-symbolic")
        .build();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let emoji_list = must_load_emojis();

    let search_result_container = search::build_search_result();
    let scrolled_search_result = gtk::ScrolledWindow::builder()
        .child(&search_result_container)
        .hexpand(true)
        .vexpand(true)
        .build();

    let notebook = build_notebook(emoji_list.clone());

    let stack = gtk::Stack::builder().build();
    stack.add_named(&notebook, Some("notebook"));
    stack.add_named(&scrolled_search_result, Some("search"));

    container.append(&search_entry);
    container.append(&stack);

    search_entry.connect_changed(search::handle_search(
        stack,
        search_result_container,
        emoji_list,
    ));

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_height(200)
        .default_width(450)
        .title("Emo")
        .child(&container)
        .build();

    navigation::handle_navigation(app, &window, &notebook);

    window.connect_close_request(|window| {
        window.set_visible(false);
        glib::Propagation::Stop
    });

    window.present();
}

fn must_load_emojis() -> Rc<Vec<Rc<emojis::Emoji>>> {
    let emoji_list = match emojis::load_emojis() {
        Ok(emojis) => emojis,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    Rc::new(emoji_list.into_iter().map(Rc::new).collect())
}

fn build_notebook(emojis: Rc<Vec<Rc<emojis::Emoji>>>) -> gtk::Notebook {
    let container = gtk::Notebook::builder().hexpand(true).vexpand(true).build();
    container.set_tab_pos(gtk::PositionType::Left);

    groups::load_groups(container.clone(), emojis);

    container
}

fn load_global_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
