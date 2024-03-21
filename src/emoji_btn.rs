use std::rc::Rc;

use crate::emojis;
use glib::clone;
use gtk::prelude::*;
use gtk::{gdk, glib};
use gtk4 as gtk;

pub fn build_emoji_btn(emoji: Rc<emojis::Emoji>) -> gtk::Button {
    let btn = gtk::Button::builder()
        .tooltip_text(&emoji.annotation)
        .label(&emoji.emoji)
        .focusable(false)
        .build();

    btn.connect_clicked(clone!(@strong emoji, @weak btn => move |_| {
        println!("{}: {}", emoji.annotation, emoji.emoji);
        let clipboard = gdk::Display::default()
            .expect("Failed to get display")
            .clipboard();

        clipboard.set_text(&emoji.emoji);
        btn.parent().unwrap().grab_focus();
    }));

    btn
}
