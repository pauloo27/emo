use crate::{emoji_btn::build_emoji_btn, emojis};
use gtk::prelude::*;
use gtk4 as gtk;
use std::rc::Rc;

pub fn build_search_result() -> gtk::FlowBox {
    let container = gtk::FlowBox::builder().build();
    container.append(&gtk::Label::new(Some("Can't search yet")));

    container.connect_child_activated(|_, box_child| {
        // for some reason it has ~300ms of delay :(
        box_child.child().unwrap().activate();
    });

    container
}

pub fn handle_search(
    stack: gtk::Stack,
    container: gtk::FlowBox,
    emojis: Rc<Vec<Rc<emojis::Emoji>>>,
) -> impl Fn(&gtk::Entry) {
    return move |entry: &gtk::Entry| {
        let value = entry.text();
        if value == "" {
            stack.set_visible_child_name("notebook");
        } else {
            stack.set_visible_child_name("search");
            // clear the container...
            let mut next_child = container.first_child();
            while let Some(child) = next_child {
                next_child = child.next_sibling();
                container.remove(&child);
            }
            // TODO: debounce
            // show the thingy
            show_filtered_emojis(&value, emojis.clone(), container.clone());
        }
    };
}

fn show_filtered_emojis(search: &str, emojis: Rc<Vec<Rc<emojis::Emoji>>>, container: gtk::FlowBox) {
    println!("Searching for: {}", search);
    // for now, to avoid trashing the app by spamming 98123712397 results
    if search.chars().count() < 2 {
        return;
    }

    // TODO: limit result count
    for emoji in emojis.iter() {
        // moving this to inside the if will be slower? maybe
        if emoji.is_skintone() {
            continue;
        }

        if emoji.annotation.contains(search) || emoji.tags.contains(&search.to_string()) {
            let btn = build_emoji_btn(emoji.clone());
            container.append(&btn);
        }
    }
}
