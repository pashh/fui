use std::rc::Rc;

use cursive::view::AnyView;
use cursive::views::{LinearLayout, TextView};
use serde_json::value::Value;

use feeders::Feeder;
use fields::{label_with_help_layout, FormField};
use validators::Validator;
use views;

pub struct Autocomplete {
    label: String,
    help: String,
    initial: String,
    validators: Vec<Box<Validator>>,
    feeder: Rc<Feeder>,
}
impl Autocomplete {
    pub fn new<IS: Into<String>, DP: Feeder>(label: IS, feeder: DP) -> Self {
        Autocomplete {
            label: label.into(),
            help: "".into(),
            initial: "".into(),
            validators: vec![],
            feeder: Rc::new(feeder),
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

impl FormField for Autocomplete {
    fn get_widget(&self) -> Box<AnyView> {
        let dp = Rc::clone(&self.feeder);
        let view = views::Autocomplete::new(dp).value(self.initial.as_ref());
        label_with_help_layout(Box::new(view), &self.label, &self.help)
    }

    fn get_widget_value(&self, view: &AnyView) -> String {
        let boxed_widget = (*view).as_any().downcast_ref::<Box<AnyView>>().unwrap();
        let widget = (**boxed_widget)
            .as_any()
            .downcast_ref::<LinearLayout>()
            .unwrap();
        let boxed_field = (*widget)
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<Box<AnyView>>()
            .unwrap();
        let ac = (**boxed_field)
            .as_any()
            .downcast_ref::<views::Autocomplete>()
            .unwrap();
        let value = (*ac).get_value();

        (&*value).clone()
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

    /// Sets field's error
    fn set_widget_error(&self, view: &mut AnyView, error: &str) {
        let boxed_widget = (*view).as_any_mut().downcast_mut::<Box<AnyView>>().unwrap();
        let widget = (**boxed_widget)
            .as_any_mut()
            .downcast_mut::<LinearLayout>()
            .unwrap();
        let error_field = (*widget)
            .get_child_mut(2)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<TextView>()
            .unwrap();
        error_field.set_content(error);
    }
}
