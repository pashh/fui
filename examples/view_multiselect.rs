// Usage example of view Multiselect
extern crate cursive;
extern crate fui;

use std::rc::Rc;

use cursive::Cursive;
use cursive::views::LinearLayout;

use fui::feeders::DirItems;
use fui::views::Multiselect;

fn handler(_c: &mut Cursive, kind: &str, value: Rc<String>) {
    let text = format!("{:?}: {:?}", kind, value);
    // uncomment it to see data without redirecting errors stream, like:
    // cargo run --example view_multiselect 2>errors.log
    //c.add_layer(Dialog::info(text.clone()));
    eprintln!("{:?}", text);
}

fn main() {
    let mut c = Cursive::new();

    let widget = LinearLayout::vertical().child(
        Multiselect::new(DirItems::current_dir().dirs())
                // allows user to select single item many items
                //TODO: .redundant_selection()
                // allows user to select single out of completition
                //TODO: .select_anything(handler)
                .on_select(|c, text| handler(c, "on_select", text))
                .on_deselect(|c, text| handler(c, "on_deselect", text)),
    );

    c.add_layer(widget);

    c.run();
}
