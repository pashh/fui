// Partially reflected `ln` command with these actions:
// * create a link to TARGET with the name LINK_NAME
// * create a link to TARGET in the current directory
// * create links to each TARGET in DIRECTORY

extern crate fui;

use fui::feeders::DirItems;
use fui::fields::{Autocomplete, Checkbox, Text};
use fui::form::FormView;
use fui::validators::{DirExists, Required};
use fui::{Fui, Value};

fn hdlr(v: Value) {
    println!("user input (from hdlr) {:?}", v);
}

fn main() {
    let target = Autocomplete::new("TARGET", DirItems::new())
        .help("Target of link")
        .validator(Required);
    let make_symbolic =
        Checkbox::new("make_symbolic").help("make symbolic links instead of hard links");
    Fui::new()
        .action(
            "BASIC LINK: create a link to TARGET with the name LINK_NAME",
            FormView::new()
                .field(target.clone())
                .field(
                    // TODO: Autocomplete(DirItems::new())
                    Text::new("LINK_NAME")
                        .help("Destiny of link")
                        .validator(Required),
                )
                .field(make_symbolic.clone().initial(true)),
            hdlr,
        )
        .action(
            "MANY FILES, SINGLE DIR: create links to each TARGET in DIRECTORY",
            FormView::new()
                .field(target)
                .field(
                    Autocomplete::new("DIRECTORY", DirItems::new())
                        .help("Directory where all links should be stored")
                        .validator(Required)
                        .validator(DirExists),
                )
                .field(make_symbolic.clone()),
            hdlr,
        )
        .run();
}
