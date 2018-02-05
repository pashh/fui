//use std::str::FromStr;
use cursive::view::AnyView;
use cursive::views;
use serde_json::value::Value;

mod autocomplete;
mod checkbox;
mod text;

pub use self::autocomplete::Autocomplete;
pub use self::checkbox::Checkbox;
pub use self::text::Text;

pub trait FormField {
    /// Gets widget representing this field
    fn get_widget(&self) -> Box<AnyView>;
    /// Gets value from widget
    fn get_widget_value(&self, view: &AnyView) -> String;
    /// Validates data
    //TODO: make data String
    fn validate(&self, data: &str) -> Result<Value, String>;
    /// Gets label of the field
    fn get_label(&self) -> &str;
    /// Sets error field's error
    fn set_widget_error(&self, _view: &mut AnyView, error: &str);
    // TODO: this should include these, but Self return fails
    // pub fn initial(mut self, value: bool) -> Self;
    // pub fn help<IS: Into<String>>(mut self, text: IS) -> Self;
}

fn format_annotation(label: &str, help: &str) -> String {
    if help.len() > 0 {
        format!("{:20}: {}", label, help)
    } else {
        format!("{:20}", label)
    }
}

/// Widget layout where label & help are in the same line.
///
/// This layout works with container views, like:
///
/// * [`LinearLayout`](../../cursive/views/struct.LinearLayout.html)
///
/// or views using container views, like:
///
/// * [`Autocomplete`](../views/struct.Autocomplete.html)
pub fn label_with_help_layout(view: Box<AnyView>, label: &str, help: &str) -> Box<AnyView> {
    let text = format_annotation(label, help);
    let widget = views::LinearLayout::vertical()
        .child(views::TextView::new(text))
        .child(view)
        .child(views::TextView::new(""))
        .child(views::DummyView);

    Box::new(widget)
}
