use cursive::view::AnyView;
use cursive::views;
use serde_json::value::Value;

use validators::Validator;
use fields;

pub struct Text {
    label: String,
    help: String,
    initial: String,
    validators: Vec<Box<Validator>>,
}
impl Text {
    pub fn new<IS: Into<String>>(label: IS) -> Self {
        Text {
            label: label.into(),
            help: "".into(),
            initial: "".into(),
            validators: vec![],
        }
    }
    pub fn help<IS: Into<String>>(mut self, msg: IS) -> Self {
        self.help = msg.into();
        self
    }
    pub fn initial<IS: Into<String>>(mut self, value: IS) -> Self {
        self.initial = value.into();
        self
    }

    pub fn validator<V: Validator + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Box::new(validator));
        self
    }
}

impl fields::FormField for Text {
    fn get_widget(&self) -> Box<AnyView> {
        let view = views::EditView::new().content(self.initial.clone());
        fields::label_with_help_layout(Box::new(view), &self.label, &self.help)
    }

    fn get_widget_value(&self, view: &AnyView) -> String {
        // fuck yea!
        let view: &Box<AnyView> = (*view.as_any()).downcast_ref::<Box<AnyView>>().unwrap();
        let wrapped_widget: &views::LinearLayout = (**view)
            .as_any()
            .downcast_ref::<views::LinearLayout>()
            .unwrap();
        let wrapped_edit: &Box<AnyView> = wrapped_widget
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<Box<AnyView>>()
            .unwrap();
        let edit: &views::EditView = (**wrapped_edit)
            .as_any()
            .downcast_ref::<views::EditView>()
            .unwrap();
        let value: String = (&*edit.get_content()).clone();
        value
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
}
