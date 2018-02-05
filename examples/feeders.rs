// Usage example of data providers
extern crate cursive;
extern crate fui;

use std::env::home_dir;
use std::rc::Rc;

use cursive::Cursive;
use cursive::traits::Boxable;
use cursive::views::{Dialog, DummyView, LinearLayout};

use fui::views::Autocomplete;
use fui::feeders::DirItems;

fn handler(c: &mut Cursive, submitted: Rc<String>) {
    let text = format!("submitted {:?}", submitted);
    c.add_layer(Dialog::info(text));
}

fn main() {
    let mut c = Cursive::new();

    let vec_selector = vec!["option1", "option2", "option3", ".."];
    let current_dir_dirs = DirItems::current_dir().dirs();
    let current_dir_files = DirItems::current_dir().files();
    let home_dir_dirs = DirItems::dir(home_dir().unwrap()).dirs();
    let home_dir_files = DirItems::dir(home_dir().unwrap()).files();

    let layout = LinearLayout::vertical()
        .child(Autocomplete::new(vec_selector).on_submit(handler))
        .child(DummyView)
        .child(Autocomplete::new(current_dir_dirs).on_submit(handler))
        .child(DummyView)
        .child(Autocomplete::new(current_dir_files).on_submit(handler))
        .child(DummyView)
        .child(Autocomplete::new(home_dir_dirs).on_submit(handler))
        .child(DummyView)
        .child(Autocomplete::new(home_dir_files).on_submit(handler))
        .child(DummyView);

    c.add_layer(Dialog::around(layout).full_width());

    c.run();
}
