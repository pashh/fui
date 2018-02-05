use std::str::FromStr;

use cursive::view::AnyView;
use cursive::views;
use serde_json::value::Value;

use fields;

#[derive(Clone)]
pub struct Checkbox {
    label: String,
    help: String,
    initial: bool,
}

impl Checkbox {
    pub fn new<IS: Into<String>>(label: IS) -> Self {
        Checkbox {
            label: label.into(),
            help: "".into(),
            initial: false,
        }
    }
    pub fn help<IS: Into<String>>(mut self, text: IS) -> Self {
        self.help = text.into();
        self
    }
    pub fn initial(mut self, value: bool) -> Self {
        self.initial = value;
        self
    }
}

impl fields::FormField for Checkbox {
    fn get_widget(&self) -> Box<AnyView> {
        let mut checkbox = views::Checkbox::new();
        checkbox.set_checked(self.initial);
        fields::label_with_help_layout(Box::new(checkbox), &self.label, &self.help)
    }
    fn get_label(&self) -> &str {
        &self.label
    }
    fn get_widget_value(&self, view: &AnyView) -> String {
        let boxed_widget = view.as_any().downcast_ref::<Box<AnyView>>().unwrap();
        let widget = (**boxed_widget)
            .as_any()
            .downcast_ref::<views::LinearLayout>()
            .unwrap();
        let boxed_field = widget
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<Box<AnyView>>()
            .unwrap();
        let checkbox = (**boxed_field)
            .as_any()
            .downcast_ref::<views::Checkbox>()
            .unwrap();
        let value = checkbox.is_checked();
        format!("{}", value)
    }
    fn set_widget_error(&self, _view: &mut AnyView, _error: &str) {
        // no operation, checkbox is always valid
    }
    fn validate(&self, data: &str) -> Result<Value, String> {
        let value = FromStr::from_str(data)
            .map(|v| Value::Bool(v))
            .map_err(|_| "Value can't be converterd to bool".to_string());
        value
    }
}
