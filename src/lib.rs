#[macro_use]
extern crate cursive as _cursive;
extern crate regex;
extern crate serde_json;
extern crate walkdir;

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

use std::rc::Rc;
use std::sync::mpsc::channel;

use cursive::Cursive;
use cursive::traits::{Boxable, Identifiable};
use cursive::views::Dialog;

use form::FormView;

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
            // cursive instance breaks println!, enclose it in scope to fix a println!
            let mut c = cursive::Cursive::new();
            //TODO: should be seperate view (CmdPicker + FormView)?

            // cmd picker
            //TODO: replace mspc with rc for cmd_picker & form (this fn)
            let (picker_sender, picker_receiver) = channel();
            let picker_on_submit = picker_sender.clone();
            let picker_on_cancel = picker_sender.clone();
            let cmd_picker = views::Autocomplete::new(self.descs.clone()).on_submit(
                move |c: &mut Cursive, text: Rc<String>| {
                    picker_on_submit.send(Some(text)).unwrap();
                    c.quit();
                },
            );
            c.add_fullscreen_layer(
                Dialog::around(cmd_picker)
                    .button("Cancel", move |c| {
                        picker_on_cancel.send(None).unwrap();
                        c.quit()
                    })
                    .full_screen(),
            );
            c.run();
            let selected_idx = picker_receiver
                .recv()
                .unwrap()
                .and_then(|selected| self.descs.iter().position(|item| item == &**selected));
            if selected_idx.is_none() {
                return;
            }

            // form
            let (form_sender, form_receiver) = channel();
            let form_on_submit = form_sender.clone();
            let form_on_cancel = form_sender.clone();
            // TODO: this whole code needs redesign, example quirks
            // * collecting views in vec,
            // * then moving views out of vec. because they can't be clone
            // .. and many more :)
            let mut form_view = self.forms.remove(selected_idx.unwrap());
            form_view.set_on_submit(move |c: &mut Cursive, data: Value| {
                form_on_submit.send(Some(data)).unwrap();
                c.quit();
            });
            c.add_layer(
                Dialog::around(form_view.with_id("form"))
                    //TODO: add cancel & submit buttons to FormView then
                    //use form on_submit & on_cancel here
                    .button("Cancel", move |c| {
                        form_on_cancel.send(None).unwrap();
                        c.quit();
                    })
                    .button("Submit (CTRL+f)", move |c| {
                        let mut form = c.find_id::<FormView>("form").unwrap();
                        form.submit();
                    }).full_width(),
            );
            c.run();
            (form_receiver.recv().unwrap(), selected_idx.unwrap())
        };

        // run handler
        if let Some(data) = form_data {
            let hdlr = self.hdlrs.remove(selected_idx);
            hdlr(data)
        }
    }
}
