// Partially reflected `ln` command with these actions:
// * create a link to TARGET with the name LINK_NAME
// * create a link to TARGET in the current directory
// * create links to each TARGET in DIRECTORY

extern crate fui;

use fui::{Fui, Value};
use fui::feeders::DirItems;
use fui::form::FormView;
use fui::fields::{Autocomplete, Checkbox, Text};
use fui::validators::Required;

fn hdlr(v: Value) {
    println!("user input (from hdlr) {:?}", v);
}

fn clone_target() -> Autocomplete {
    // cloning Autocomplete is not implemented yet, so we're using clone_target for that
    // perhaps instead of cloning, simple reference would be enough but this needs research and
    // implementation, so stay tuned
    Autocomplete::new("TARGET", DirItems::current_dir().files())
        .help("Target of link")
        .validator(Required)
    //TODO: .multi(true)
}

fn main() {
    let make_symbolic =
        Checkbox::new("make_symbolic").help("make symbolic links instead of hard links");
    Fui::new()
        .action(
            "BASIC LINK: create a link to TARGET with the name LINK_NAME",
            FormView::new()
                .field(clone_target())
                .field(
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
                .field(clone_target())
                .field(
                    Autocomplete::new("DIRECTORY", DirItems::current_dir().dirs())
                        .help("Directory where all links should be stored")
                        .validator(Required),
                )
                .field(make_symbolic.clone()),
            hdlr,
        )
        .run();
}
