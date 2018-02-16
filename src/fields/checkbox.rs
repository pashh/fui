use std::str::FromStr;

use cursive::view::AnyView;
use cursive::views;
use serde_json::value::Value;

use fields;
use fields::WidgetManager;

#[derive(Clone)]
pub struct CheckboxManager;

impl fields::WidgetManager for CheckboxManager {
    fn full_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView> {
        let checkbox = self.widget_factory(&initial);
        fields::label_with_help_layout(checkbox, &label, &help)
    }
    fn get_value(&self, view: &AnyView) -> String {
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
    fn set_error(&self, _view: &mut AnyView, _error: &str) {
        // no operation, checkbox is always valid
    }
    fn widget_factory(&self, value: &str) -> Box<AnyView> {
        let value = FromStr::from_str(value).unwrap();
        let mut checkbox = views::Checkbox::new();
        checkbox.set_checked(value);
        Box::new(checkbox)
    }
}

pub struct Checkbox;

impl Checkbox {
    pub fn new<IS: Into<String>>(label: IS) -> fields::Field<CheckboxManager, bool> {
        fields::Field::new(label, CheckboxManager, false)
    }
}

impl<W: WidgetManager> fields::Field<W, bool> {
    pub fn initial(mut self, value: bool) -> Self {
        self.initial = value;
        self
    }
}

impl fields::FormField for fields::Field<CheckboxManager, bool> {
    fn get_widget(&self) -> Box<AnyView> {
        let initial = format!("{}", self.initial);
        self.widget_manager
            .full_widget(&self.label, &self.help, &initial)
    }
    fn get_label(&self) -> &str {
        &self.label
    }
    fn get_widget_value(&self, view: &AnyView) -> String {
        self.widget_manager.get_value(view)
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
