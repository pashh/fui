use std::rc::Rc;
use std::collections::HashMap;

use cursive::Cursive;
use cursive::event::{Callback, Event, EventResult};
use cursive::view::{View, ViewWrapper};
use cursive::views::LinearLayout;
use serde_json::map::Map;
use serde_json::value::Value;

use fields::FormField;

type OnSubmit = Option<Rc<Fn(&mut Cursive, Value)>>;

pub struct FormView {
    view: LinearLayout,

    fields: Vec<Box<FormField>>,
    on_submit: OnSubmit,
}
impl FormView {
    pub fn new() -> Self {
        //TODO: add buttons submit, cancel
        FormView {
            view: LinearLayout::vertical(),
            fields: Vec::new(),
            on_submit: None,
        }
    }

    pub fn field<V: FormField + 'static>(mut self, field: V) -> Self {
        //let field_label = field.get_label().to_owned();
        let widget = field.get_widget();
        self.view.add_child(widget);
        self.fields.push(Box::new(field));
        self
    }
    pub fn set_on_submit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, Value) + 'static,
    {
        self.on_submit = Some(Rc::new(callback));
    }
    pub fn on_submit<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, Value) + 'static,
    {
        self.set_on_submit(callback);
        self
    }
    fn validate(&self) -> Result<Value, HashMap<String, String>> {
        let mut data = Map::with_capacity(self.fields.len());
        let mut errors = HashMap::with_capacity(self.fields.len());

        for (idx, field) in self.fields.iter().enumerate() {
            let view = self.view.get_child(idx).unwrap();
            let value = field.get_widget_value(view);
            let label = field.get_label();
            match field.validate(value.as_ref()) {
                Ok(v) => {
                    data.insert(label.to_owned(), v);
                }
                Err(e) => {
                    errors.insert(label.to_owned(), e.to_owned());
                }
            }
        }

        if errors.is_empty() {
            Ok(Value::Object(data))
        } else {
            Err(errors)
        }
    }

    pub fn submit(&mut self) -> EventResult {
        match self.validate() {
            Ok(data_map) => {
                let opt_cb = self.on_submit
                    .clone()
                    .map(|cb| Callback::from_fn(move |c| cb(c, data_map.clone())));
                EventResult::Consumed(opt_cb)
            }
            Err(errors) => {
                // TODO: the event focus next required/invalid field?
                for (idx, field) in self.fields.iter().enumerate() {
                    let label = field.get_label();
                    let view = self.view.get_child_mut(idx).unwrap();
                    if let Some(e) = errors.get(label) {
                        field.set_widget_error(view, e);
                    }
                }
                EventResult::Consumed(None)
            }
        }
    }
}
impl ViewWrapper for FormView {
    wrap_impl!(self.view: LinearLayout);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            // TODO: ctlr+enter binding?
            Event::CtrlChar('f') => self.submit(),
            _ => {
                // default behaviour from ViewWrapper
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored)
            }
        }
    }
}
