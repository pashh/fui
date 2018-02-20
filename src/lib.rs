#[macro_use]
extern crate cursive as _cursive;
extern crate glob;
extern crate regex;
extern crate serde_json;
#[cfg(test)]
extern crate tempdir;

// Re-export of cursive
pub mod cursive {
    pub use _cursive::*;
}
pub use serde_json::value::Value;
pub mod feeders;
pub mod fields;
pub mod form;
pub mod utils;
pub mod validators;
pub mod views;

use cursive::Cursive;
use cursive::traits::Boxable;
use form::FormView;
use std::cell::RefCell;
use std::rc::Rc;
use validators::OneOf;

pub struct Fui {
    descs: Vec<String>,
    forms: Vec<FormView>,
    hdlrs: Vec<Box<Fn(Value) + 'static>>,
}
impl Fui {
    pub fn new() -> Self {
        Fui {
            descs: Vec::new(),
            forms: Vec::new(),
            hdlrs: Vec::new(),
        }
    }
    pub fn action<IS, F>(mut self, desc: IS, form: FormView, hdlr: F) -> Self
    where
        IS: Into<String>,
        F: Fn(Value) + 'static,
    {
        self.descs.push(desc.into());
        self.forms.push(form);
        self.hdlrs.push(Box::new(hdlr));
        self
    }
    pub fn run(mut self) {
        let (form_data, selected_idx) = {
            // cursive instance breaks println!, enclose it with scope to fix printing
            let mut c = cursive::Cursive::new();

            // cmd picker
            let mut cmd: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
            let cmd_clone = Rc::clone(&cmd);
            c.add_layer(
                FormView::new()
                    .field(
                        fields::Autocomplete::new("action", self.descs.clone())
                            .help("Pick action")
                            .validator(OneOf(self.descs.clone())),
                    )
                    .on_submit(move |c, data| {
                        let value = data.get("action").unwrap().clone();
                        *cmd_clone.borrow_mut() = Some(value.as_str().unwrap().to_string());
                        c.quit();
                    })
                    .on_cancel(|c| c.quit())
                    .full_screen(),
            );
            c.run();
            let selected_idx = cmd.borrow()
                .clone()
                .and_then(|v| self.descs.iter().position(|item| item == v.as_str()));
            let selected_idx = match selected_idx {
                None => return,
                Some(idx) => idx,
            };

            // form
            let mut form_view = self.forms.remove(selected_idx);
            let mut form_data: Rc<RefCell<Option<Value>>> = Rc::new(RefCell::new(None));
            let mut form_data_submit = Rc::clone(&form_data);
            form_view.set_on_submit(move |c: &mut Cursive, data: Value| {
                *form_data_submit.borrow_mut() = Some(data);
                c.quit();
            });
            form_view.set_on_cancel(move |c: &mut Cursive| {
                //TODO: this should return to action picker
                //TODO: self.forms are drained so can't be done now
                c.quit();
            });
            c.add_layer(form_view.full_width());
            c.run();
            (form_data, selected_idx)
        };

        // run handler
        let form_data = Rc::try_unwrap(form_data).unwrap().into_inner();
        if let Some(data) = form_data {
            let hdlr = self.hdlrs.remove(selected_idx);
            hdlr(data)
        }
    }
}
