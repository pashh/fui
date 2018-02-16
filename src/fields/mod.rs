use cursive::view::AnyView;
use cursive::views;
use serde_json::value::Value;
use std::rc::Rc;
use validators::Validator;

mod autocomplete;
mod checkbox;
mod multiselect;
mod text;

pub use self::autocomplete::Autocomplete;
pub use self::checkbox::Checkbox;
pub use self::multiselect::Multiselect;
pub use self::text::Text;

/// Covers communication between from `Field` to `Widget`
pub trait WidgetManager {
    fn full_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView>;
    fn get_value(&self, view: &AnyView) -> String;
    fn set_error(&self, _view: &mut AnyView, error: &str);
    fn widget_factory(&self, value: &str) -> Box<AnyView>;
}

/// Building block for `Form`s which stores data & `Widget`
#[derive(Clone)]
pub struct Field<W: WidgetManager, T> {
    label: String,
    help: String,
    initial: T,
    validators: Vec<Rc<Validator>>,
    widget_manager: W,
}

impl<W: WidgetManager, T> Field<W, T> {
    pub fn new<IS: Into<String>>(label: IS, widget_manager: W, initial: T) -> Self {
        Field {
            label: label.into(),
            help: "".into(),
            initial: initial,
            validators: vec![],
            widget_manager: widget_manager,
        }
    }
    pub fn help<IS: Into<String>>(mut self, msg: IS) -> Self {
        self.help = msg.into();
        self
    }
    pub fn validator<V: Validator + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Rc::new(validator));
        self
    }
}

/// Trait will be completely changed when all fields are migrated to Field and FormField will be
/// removed
pub trait FormField {
    //TODO:: get_widget_manager()?
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
