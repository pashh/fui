use cursive::view::AnyView;
use cursive::views;
use serde_json::value::Value;

use fields;
use fields::WidgetManager;

pub struct Text;

impl Text {
    pub fn new<IS: Into<String>>(label: IS) -> fields::Field<TextManager, String> {
        fields::Field::new(label, TextManager, "".to_string())
    }
}

#[derive(Clone)]
pub struct TextManager;

impl WidgetManager for TextManager {
    fn full_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView> {
        let view = self.widget_factory(initial);
        fields::label_with_help_layout(view, label, help)
    }
    fn get_value(&self, view: &AnyView) -> String {
        // fuck yea!
        let boxed_widget: &Box<AnyView> = (*view.as_any()).downcast_ref::<Box<AnyView>>().unwrap();
        let widget: &views::LinearLayout = (**boxed_widget)
            .as_any()
            .downcast_ref::<views::LinearLayout>()
            .unwrap();
        let boxed_widget: &Box<AnyView> = widget
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<Box<AnyView>>()
            .unwrap();
        let edit: &views::EditView = (**boxed_widget)
            .as_any()
            .downcast_ref::<views::EditView>()
            .unwrap();
        let value: String = (&*edit.get_content()).clone();
        value
    }
    fn set_error(&self, view: &mut AnyView, error: &str) {
        let view: &mut Box<AnyView> = (*view.as_any_mut()).downcast_mut::<Box<AnyView>>().unwrap();
        let layout: &mut views::LinearLayout = (**view)
            .as_any_mut()
            .downcast_mut::<views::LinearLayout>()
            .unwrap();
        let text: &mut views::TextView = layout
            .get_child_mut(2)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<views::TextView>()
            .unwrap();
        text.set_content(error);
    }
    fn widget_factory(&self, value: &str) -> Box<AnyView> {
        Box::new(views::EditView::new().content(value))
    }
}

impl fields::FormField for fields::Field<TextManager, String> {
    fn get_widget(&self) -> Box<AnyView> {
        self.widget_manager
            .full_widget(&self.label, &self.help, &self.initial)
    }

    fn get_widget_value(&self, view: &AnyView) -> String {
        self.widget_manager.get_value(view)
    }

    fn validate(&self, data: &str) -> Result<Value, String> {
        for v in &self.validators {
            if let Some(e) = v.validate(data) {
                return Err(e);
            }
        }
        Ok(Value::String(data.to_owned()))
    }

    /// Gets label of the field
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Sets error field's error
    fn set_widget_error(&self, view: &mut AnyView, error: &str) {
        self.widget_manager.set_error(view, error)
    }
}

impl<W: WidgetManager> fields::Field<W, String> {
    pub fn initial<IS: Into<String>>(mut self, initial: IS) -> Self {
        self.initial = initial.into();
        self
    }
}
