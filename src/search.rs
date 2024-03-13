use gtk::prelude::*;
use gtk4 as gtk;

pub fn build_search_result() -> gtk::FlowBox {
    let container = gtk::FlowBox::builder().build();
    container.append(&gtk::Label::new(Some("Can't search yet")));

    container
}

pub fn handle_search(stack: gtk::Stack, container: gtk::FlowBox) -> impl Fn(&gtk::Entry) {
    return move |entry: &gtk::Entry| {
        let value = entry.text();
        if value == "" {
            stack.set_visible_child_name("notebook");
        } else {
            stack.set_visible_child_name("search");
        }
    };
}
