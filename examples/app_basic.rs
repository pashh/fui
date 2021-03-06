// Example showing imagined CLI app. with two actions

extern crate fui;

use fui::{Fui, Value};
use fui::form::FormView;
use fui::fields::Text;

fn hdlr(v: Value) {
    println!("user input (from hdlr) {:?}", v);
}

fn main() {
    Fui::new()
        .action(
            "ACTION1: description",
            FormView::new().field(Text::new("action1 data").help("help for action1 data")),
            |v| {
                println!("user input (from callback) {:?}", v);
            },
        )
        .action(
            "ACTION2: description",
            FormView::new().field(Text::new("action2 data").help("help for action2 data")),
            hdlr,
        )
        .run();
}
