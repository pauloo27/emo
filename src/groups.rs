use crate::emoji_btn::build_emoji_btn;
use crate::emojis;
use gtk::prelude::*;
use gtk4 as gtk;
use std::collections::HashMap;
use std::rc::Rc;

pub fn load_groups(container: gtk::Notebook, emojis: Rc<Vec<Rc<emojis::Emoji>>>) {
    let mut groups_containers = HashMap::<String, gtk::FlowBox>::new();

    let mut create_group = |symbol: &str, names: &[&str]| {
        let group_box = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .activate_on_single_click(true)
            .build();
        let scrolled = gtk::ScrolledWindow::builder().child(&group_box).build();

        group_box.connect_child_activated(|_, box_child| {
            // for some reason it has ~300ms of delay :(
            box_child.child().unwrap().activate();
        });

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
        // ignore skin toned emojis
        if emoji.is_skintone() {
            continue;
        }

        if let Some(container) = groups_containers.get(&emoji.group) {
            let btn = build_emoji_btn(emoji.clone());
            container.append(&btn);
        };
    }
}
