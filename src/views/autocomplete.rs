use std::rc::Rc;

use cursive::Cursive;
use cursive::With;
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::traits::View;
use cursive::view::ViewWrapper;
use cursive::views::{EditView, LinearLayout, SelectView};

use feeders::Feeder;

// TODO: selecting should auto load next/prev items
// TODO: better performance while typing

pub type OnSubmit = Option<Rc<Fn(&mut Cursive, Rc<String>)>>;

pub struct Autocomplete {
    view: LinearLayout,

    feeder: Rc<Feeder>,
    shown_count: u8,
    submit_anything: bool,

    on_submit: OnSubmit,
}

impl Autocomplete {
    pub fn new<T: Feeder>(feeder: T) -> Autocomplete {
        let shown_count = 5;

        let select = SelectView::<String>::new()
                .with_all_str(feeder.query("", 0, shown_count).into_iter())
                //TODO: make fixed height for select equal to shown_count
                // use cursive::traits::Boxable;
                //.fixed_height(shown_count)
                // * using fixed_height converts SelectView to BoxView
                // * each shown_count update should update size of select (which'd be BoxView)
                ;

        let layout = LinearLayout::vertical()
            .child(EditView::new())
            .child(select);

        let ac = Autocomplete {
            view: layout,

            feeder: Rc::new(feeder),
            shown_count: shown_count as u8,
            submit_anything: false,

            on_submit: None,
        };

        ac
    }

    /// Get typed in value
    pub fn get_value(&self) -> Rc<String> {
        self.get_edit_view().get_content()
    }

    /// Allow to submit values outside of completition
    pub fn submit_anything(mut self) -> Self {
        self.submit_anything = true;
        self
    }

    /// Sets text value
    pub fn value(mut self, initial: &str) -> Self {
        self.get_edit_view_mut().set_content(initial);
        self.refresh_listing();
        self
    }

    /// Refresh listing
    fn refresh_listing(&mut self) {
        let feeder = Rc::clone(&self.feeder);
        let text = self.get_edit_view().get_content();
        let shown_count = self.shown_count as usize;
        let select = self.get_select_view_mut();
        select.clear();
        select.add_all_str((*feeder).query(text.as_ref(), 0, shown_count).into_iter());
    }

    /// Copy selected text to edit view
    fn selection_to_edit(&mut self) {
        let selection = self.get_select_view_mut().selection();
        self.get_edit_view_mut().set_content((&*selection).clone());
    }

    /// Checks if value comes from completition
    pub fn is_value_from_select(&self, to_check: &str) -> bool {
        let select = self.get_select_view();
        let mut idx = 0;
        while let Some((_, v)) = select.get_item(idx) {
            idx += 1;
            if to_check == *v {
                return true;
            }
        }
        return false;
    }

    fn get_edit_view(&self) -> &EditView {
        self.view
            .get_child(0)
            .unwrap()
            .as_any()
            .downcast_ref::<EditView>()
            .unwrap()
    }

    fn get_edit_view_mut(&mut self) -> &mut EditView {
        self.view
            .get_child_mut(0)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<EditView>()
            .unwrap()
    }

    fn get_select_view(&self) -> &SelectView {
        self.view
            .get_child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<SelectView<String>>()
            .unwrap()
    }

    fn get_select_view_mut(&mut self) -> &mut SelectView {
        self.view
            .get_child_mut(1)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<SelectView<String>>()
            .unwrap()
    }

    pub fn set_on_submit<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.on_submit = Some(Rc::new(callback));
    }

    pub fn on_submit<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.with(|v| v.set_on_submit(callback))
    }
}

impl ViewWrapper for Autocomplete {
    wrap_impl!(self.view: LinearLayout);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(_) | Event::Key(Key::Backspace) | Event::Key(Key::Del) => {
                // typing
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored);
                self.refresh_listing();
                EventResult::Consumed(None)
            }
            Event::CtrlChar('u') => {
                self.get_edit_view_mut().set_content("");
                self.refresh_listing();
                EventResult::Consumed(None)
            }
            Event::CtrlChar('p') => {
                // move selection up
                {
                    let select = self.get_select_view_mut();
                    select.select_up(1);
                }
                self.selection_to_edit();
                EventResult::Consumed(None)
            }
            Event::CtrlChar('n') => {
                // move selection down
                {
                    let select = self.get_select_view_mut();
                    select.select_down(1);
                }
                self.selection_to_edit();
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) => {
                // submitting
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored);

                let to_submit = self.get_edit_view().get_content();

                if !self.submit_anything {
                    let from_select = self.is_value_from_select(&*to_submit);
                    if !from_select {
                        return EventResult::Ignored;
                    }
                }

                let cb = self.on_submit
                    .clone()
                    .map(|on_submit| Callback::from_fn(move |c| on_submit(c, to_submit.clone())));
                EventResult::Consumed(cb)
            }
            _ => {
                // default behaviour from ViewWrapper
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored)
            }
        }
    }
}
