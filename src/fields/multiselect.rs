use std::ops::Deref;
use std::rc::Rc;

use cursive::view::AnyView;
use cursive::views::{LinearLayout, TextView};
use serde_json::value::Value;

use feeders::Feeder;
use fields::{label_with_help_layout, Field, FormField, WidgetManager};
use views;

impl<W: WidgetManager> Field<W, Vec<String>> {
    pub fn initial<U: Deref<Target = str>>(mut self, initial: Vec<U>) -> Self {
        self.initial = initial
            .iter()
            .map(|x| (*x).to_string())
            .collect::<Vec<String>>();
        self
    }
}

pub struct MultiselectManager {
    feeder: Rc<Feeder>,
}

impl WidgetManager for MultiselectManager {
    fn widget_factory(&self, initial: &str) -> Box<AnyView> {
        let mut widget = views::Multiselect::new(Rc::clone(&self.feeder));
        if initial.trim() != "" {
            let items = initial
                .split(",")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            widget.select_items(items);
        }
        Box::new(widget)
    }
    fn full_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView> {
        let view = self.widget_factory(initial);
        label_with_help_layout(view, label, help)
    }
    fn get_value(&self, view: &AnyView) -> String {
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
        let ms = (**boxed_field)
            .as_any()
            .downcast_ref::<views::Multiselect>()
            .unwrap();

        let result: Vec<String> = ms.get_selected_items()
            .iter()
            .map(|x| (*x).to_owned())
            .collect();
        result.join(",")
    }
    fn set_error(&self, view: &mut AnyView, error: &str) {
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

// TODO:: tmp until all fields are migrated to Field and FormField will be removed
impl FormField for Field<MultiselectManager, Vec<String>> {
    fn validate(&self, data: &str) -> Result<Value, String> {
        // TODO:: should comes from Field impl. when all fields get migrated
        let items = data.split(",").collect::<Vec<&str>>();
        for item in items.iter() {
            for v in &self.validators {
                if let Some(e) = v.validate(item) {
                    return Err(e);
                }
            }
        }
        let vec_str = items.iter().map(|x| Value::String(x.to_string())).collect::<Vec<Value>>();
        let val_of_vec = Value::Array(vec_str);
        Ok(val_of_vec)
    }
    fn get_label(&self) -> &str {
        // TODO:: should be defined in "impl Field" when all fields are migrated
        &self.label
    }
    // TODO:: rm after migration
    fn get_widget(&self) -> Box<AnyView> {
        let initial = self.initial.join(",");
        self.widget_manager
            .full_widget(&self.label, &self.help, &initial)
    }
    // TODO:: rm after migration
    fn get_widget_value(&self, view: &AnyView) -> String {
        let value = self.widget_manager.get_value(view);
        value
    }
    // TODO:: rm after migration
    fn set_widget_error(&self, view: &mut AnyView, error: &str) {
        self.widget_manager.set_error(view, error);
    }
}

/// Convienient wrapper around Field<MultiselectManager, Vec<String>>
pub struct Multiselect;

impl Multiselect {
    pub fn new<IS: Into<String>, F: Feeder>(
        label: IS,
        feeder: F,
    ) -> Field<MultiselectManager, Vec<String>> {
        let mngr = MultiselectManager {
            feeder: Rc::new(feeder),
        };
        Field::new(label, mngr, Vec::new())
    }
}
