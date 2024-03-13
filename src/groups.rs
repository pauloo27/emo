use crate::emojis;
use glib::clone;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::collections::HashMap;
use std::rc::Rc;

pub fn load_groups(container: gtk::Notebook, emojis: Rc<Vec<Rc<emojis::Emoji>>>) {
    let mut groups_containers = HashMap::<String, gtk::FlowBox>::new();

    let mut create_group = |symbol: &str, names: &[&str]| {
        let group_box = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .build();
        let scrolled = gtk::ScrolledWindow::builder().child(&group_box).build();

        container.append_page(
            &scrolled,
            Some(
                &gtk::Label::builder()
                    .label(symbol)
                    .tooltip_text(names.join(", "))
                    .build(),
            ),
        );
        for name in names {
            groups_containers.insert(name.to_string(), group_box.clone());
        }
    };

    create_group("😀", &["smileys-emotion", "people-body"]);
    create_group("🐶", &["animals-nature"]);
    create_group("🍎", &["food-fruit", "food-drink"]);
    create_group("⚽️", &["activities"]);
    create_group("🏚️", &["travel-places"]);
    create_group("💡", &["objects"]);
    create_group("❗️", &["symbols"]);
    create_group("🇧🇷", &["flags"]);

    for emoji in emojis.as_ref() {
        if let Some(container) = groups_containers.get(&emoji.group) {
            let btn = gtk::Button::builder()
                .tooltip_text(&emoji.annotation)
                .label(&emoji.emoji)
                .build();

            btn.connect_clicked(clone!(@strong emoji => move |_| {
                println!("{}: {}", emoji.annotation, emoji.emoji);
                let clipboard = gdk::Display::default()
                    .expect("Failed to get display")
                    .clipboard();

                clipboard.set_text(&emoji.emoji);
            }));

            container.append(&btn);
        };
    }
}
