// Partially reflected `ln` command with these actions:
// * create a link to TARGET with the name LINK_NAME
// * create a link to TARGET in the current directory
// * create links to each TARGET in DIRECTORY

extern crate fui;

use fui::feeders::DirItems;
use fui::fields::{Autocomplete, Checkbox, Multiselect};
use fui::form::FormView;
use fui::utils;
use fui::validators::{DirExists, Required};
use fui::{Fui, Value};

fn hdlr(v: Value) {
    println!("user input (from hdlr) {:?}", v);
}

fn main() {
    let make_symbolic =
        Checkbox::new("make_symbolic").help("make symbolic links instead of hard links");
    Fui::new()
        .action(
            "BASIC LINK: create a link to TARGET with the name LINK_NAME",
            FormView::new()
                .field(
                    Autocomplete::new("TARGET", DirItems::new())
                        .help("Target of link")
                        .validator(Required),
                )
                .field(
                    Autocomplete::new("LINK_NAME", DirItems::new())
                        .help("Destiny of link")
                        .validator(Required),
                )
                .field(make_symbolic.clone().initial(true)),
            hdlr,
        )
        .action(
            "MANY FILES, SINGLE DIR: create links to each TARGET in DIRECTORY",
            FormView::new()
                .field(
                    Multiselect::new("TARGET", DirItems::new())
                        .help("Target of link")
                        .validator(Required),
                )
                .field(
                    Autocomplete::new("DIRECTORY", DirItems::dirs())
                        .help("Directory where all links should be stored")
                        .initial(utils::cwd())
                        .validator(Required)
                        .validator(DirExists),
                )
                .field(make_symbolic.clone()),
            hdlr,
        )
        .run();
}
