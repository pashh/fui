//! `fui` lets you build a form based user interfaces for a [CLI] program.
//!
//! [CLI]: https://en.wikipedia.org/wiki/Command-line_interface
//!
//! ## Examples
//!
//! ### Cargo.toml
//! ```toml, no_run
//! [dependencies]
//! fui = "0.7"
//! ```
//!
//! ### main.rs
//! ```no_run
//! extern crate fui;
//! ```
//!
//!
//!
//! ```no_run
//! extern crate fui;
//!
//! use fui::{Fui, Value};
//! use fui::form::FormView;
//! use fui::fields::Text;
//!
//! fn hdlr(v: Value) {
//!     println!("user input (from hdlr) {:?}", v);
//! }
//!
//! fn main() {
//!     Fui::new()
//!         .action(
//!             "ACTION1: description",
//!             FormView::new().field(Text::new("action1 data").help("help for action1 data")),
//!             |v| {
//!                 println!("user input (from callback) {:?}", v);
//!             },
//!         )
//!         .action(
//!             "ACTION2: description",
//!             FormView::new().field(Text::new("action2 data").help("help for action2 data")),
//!             hdlr,
//!         )
//!         .run();
//! }
//! ```
//!
//! <div>
//! <a href="https://github.com/xliiv/fui/blob/master/examples/app_basic.rs"><img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_basic.png" alt="app_basic.rs example", width="48%" /></a>
//! <a href="https://github.com/xliiv/fui/blob/master/examples/app_ln_like.rs"><img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_ln_like.png" alt="app_ln_like.rs example", width="48%" /></a>
//! <a href="https://github.com/xliiv/fui/blob/master/examples/app_tar_like.rs"><img src="https://raw.githubusercontent.com/xliiv/fui/master/doc/app_tar_like.png" alt="app_tar_like.rs example", width="48%" /></a>
//!
//! </div>
//!
//! ## Description
//!
//! If you look at the example above you'll notice a few entities:
//!
//! * [Fui]
//! * [FormView]
//! * [fields]
//!
//! These components will be most frequently used building blocks, especially [FormView] and
//! [fields].
//!
//! [Fui]: struct.Fui.html
//! [FormView]: form/struct.FormView.html
//! [fields]: fields/index.html
//!
//! Here's the logic behind those components:
//!
//! * [Fui] is a struct which gathers your program `action`s
//! * `action`s are things which program does (like, `git pull`, `git push`, etc.)
//! * `action` includes:
//!     * description: this should shortly explain to `user` what `action` does
//!     * [FormView]: is a container for [fields]
//!         * [fields] represents data used during `action` execution
//!     * handler: is a `fn`/`callback` called after user fills the `Form`
//!
//!
//! ### Flow:
//!
//! 1) user picks `action` (then `form` is shown)
//! 2) user submit `form` with `data`
//! 3) `handler` is called with `data` (point 2)
//!
#![deny(missing_docs)]

#[macro_use]
extern crate cursive as _cursive;
extern crate glob;
extern crate regex;
extern crate serde_json;

/// Re-export of [Cursive](../cursive/index.html) crate.
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

/// Top level building block of `fui` crate
pub struct Fui {
    descs: Vec<String>,
    forms: Vec<FormView>,
    hdlrs: Vec<Box<Fn(Value) + 'static>>,
}
impl Fui {
    /// Creates a new `Fui` with empty actions
    pub fn new() -> Self {
        Fui {
            descs: Vec::new(),
            forms: Vec::new(),
            hdlrs: Vec::new(),
        }
    }
    /// Defines action by providing `desc`, `form`, `hdlr`
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
    /// Coordinates flow from action picking to handler running
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
