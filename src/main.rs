mod emojis;
mod groups;
mod search;

use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

const APP_ID: &str = "cafe.ndo.Emo";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
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

    let search_result_container = search::build_search_result();

    let stack = gtk::Stack::builder().build();
    stack.add_named(&build_notebook(), Some("notebook"));
    stack.add_named(&search_result_container, Some("search"));

    container.append(&search_entry);
    container.append(&stack);

    search_entry.connect_changed(search::handle_search(stack, search_result_container));

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_height(200)
        .default_width(450)
        .title("Emo")
        .child(&container)
        .build();

    window.present();
}

fn build_notebook() -> gtk::Notebook {
    let container = gtk::Notebook::builder().hexpand(true).vexpand(true).build();
    container.set_tab_pos(gtk::PositionType::Left);

    groups::load_groups(container.clone());

    container
}
