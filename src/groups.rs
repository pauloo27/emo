use crate::emojis;
use gtk::gdk;
use gtk::prelude::*;
use gtk4 as gtk;
use std::collections::HashMap;
use std::process;

pub fn load_groups(container: gtk::Notebook) {
    let mut groups_containers = HashMap::<String, gtk::FlowBox>::new();

    let mut create_group = |symbol: &str, names: &[&str]| {
        let group_box = gtk::FlowBox::new();
        let scrolled = gtk::ScrolledWindow::builder().child(&group_box).build();

        container.append_page(
            &scrolled,
            Some(
                &gtk::Label::builder()
                    .margin_start(10)
                    .margin_end(10)
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

    let emojis = match emojis::load_emojis() {
        Ok(emojis) => emojis,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    for emoji in emojis {
        if let Some(container) = groups_containers.get(&emoji.group) {
            let btn = gtk::Button::builder()
                .tooltip_text(emoji.annotation)
                .label(&emoji.emoji)
                .build();

            let emoji_char = emoji.emoji;

            btn.connect_clicked(move |_| {
                let clipboard = gdk::Display::default()
                    .expect("Failed to get display")
                    .clipboard();

                clipboard.set_text(&emoji_char);
            });

            container.append(&btn);
        };
    }
}
