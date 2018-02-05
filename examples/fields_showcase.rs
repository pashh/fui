// Demonstrates available form fields
extern crate cursive;
extern crate fui;
extern crate serde_json;

use cursive::Cursive;
use cursive::traits::Boxable;
use cursive::views::Dialog;
use serde_json::value::Value;

use fui::form::FormView;
use fui::fields::{Autocomplete, Checkbox, Text};

fn show_data(c: &mut Cursive, data: Value) {
    let text = format!("Got data: {:?}", data);
    c.add_layer(Dialog::info(text));
}

fn main() {
    let mut siv = Cursive::new();

    let widget = FormView::new()
        .field(Checkbox::new("verbose").help("this is help for checkbox"))
        .field(Text::new("text-field").help("this is help for text"))
        .field(
            Autocomplete::new("autocomplete-field", vec!["op1", "op2", "op3"])
                .help("this is help for autocomplete"),
        )
        .on_submit(show_data);

    siv.add_layer(Dialog::around(widget).full_screen());

    siv.run();
}
