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

    let notebook = build_notebook();

    container.append(&search_entry);
    container.append(&notebook);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_height(200)
        .title("Emo")
        .child(&container)
        .build();

    window.present();
}

fn build_notebook() -> gtk::Notebook {
    let container = gtk::Notebook::builder().hexpand(true).vexpand(true).build();

    container.append_page(
        &gtk::Label::new(Some("some content...")),
        Some(&gtk::Label::new(Some("🕒️"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("😀"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("🐶"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("🍎"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("⚽️"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("🏚️"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("💡"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("❗️"))),
    );

    container.append_page(
        &gtk::Label::new(Some("more content...")),
        Some(&gtk::Label::new(Some("🇧🇷"))),
    );

    container
}
