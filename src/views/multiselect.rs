use std::rc::Rc;

use cursive::Cursive;
use cursive::With;
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::traits::{Boxable, View};
use cursive::view::ViewWrapper;
use cursive::views::{BoxView, DummyView, LinearLayout, OnEventView, Panel, SelectView};

use feeders::Feeder;
use views::Autocomplete;

type OnSelect = Option<Rc<Fn(&mut Cursive, Rc<String>)>>;
type OnDeselect = Option<Rc<Fn(&mut Cursive, Rc<String>)>>;

pub struct Multiselect {
    view: LinearLayout,
    selected_idx: u8,
    options_idx: u8,
    on_select: OnSelect,
    on_deselect: OnDeselect,
}

impl Multiselect {
    pub fn new<T: Feeder>(feeder: T) -> Self {
        let select_width = 30;
        let separator_width = 1;
        let layout = LinearLayout::horizontal()
            .child(Panel::new(Autocomplete::new(feeder)
                    //TODO: allow customization?
                    .full_width()))
            .child(DummyView.fixed_width(separator_width))
            .child(Panel::new(
                OnEventView::new(SelectView::<String>::new()
                            //TODO: allow customization?
                            .full_width())
                    .on_pre_event_inner(Event::CtrlChar('p'), |s| {
                    s.get_inner_mut().select_up(1);
                    Some(EventResult::Consumed(None))
                })
                    .on_pre_event_inner(Event::CtrlChar('n'), |s| {
                        s.get_inner_mut().select_down(1);
                        Some(EventResult::Consumed(None))
                    }),
            ));

        Multiselect {
            view: layout,
            // remove this when suitable tests are added?
            options_idx: 0,
            selected_idx: 2,

            on_select: None,
            on_deselect: None,
        }
    }

    fn get_options_view(&self) -> &Autocomplete {
        let box_view = self.view
            .get_child(self.options_idx as usize)
            .unwrap()
            .as_any()
            .downcast_ref::<Panel<BoxView<Autocomplete>>>()
            .unwrap();
        box_view.get_inner().get_inner()
    }

    fn get_selected_view_mut(&mut self) -> &mut SelectView<String> {
        let box_view = self.view
            .get_child_mut(self.selected_idx as usize)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Panel<OnEventView<BoxView<SelectView<String>>>>>()
            .unwrap();
        box_view.get_inner_mut().get_inner_mut().get_inner_mut()
    }

    fn select_item(&mut self) -> Rc<String> {
        let selected_text = self.get_options_view().get_value();
        self.get_selected_view_mut()
            .add_item_str((&*selected_text).clone());
        selected_text
    }

    fn deselect_item(&mut self) -> Option<Rc<String>> {
        let selected_view = self.get_selected_view_mut();
        if let Some(idx) = selected_view.selected_id() {
            let item = selected_view.selection();
            selected_view.remove_item(idx);
            return Some(item);
        } else {
            None
        }
    }

    pub fn set_on_select<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.on_select = Some(Rc::new(callback));
    }

    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.with(|v| v.set_on_select(callback))
    }

    pub fn set_on_deselect<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.on_deselect = Some(Rc::new(callback));
    }

    pub fn on_deselect<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Cursive, Rc<String>) + 'static,
    {
        self.with(|v| v.set_on_deselect(callback))
    }
}

impl ViewWrapper for Multiselect {
    wrap_impl!(self.view: LinearLayout);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Enter) => {
                self.with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored);

                let focused = self.view.get_focus_index();
                if focused == self.options_idx as usize {
                    // on select
                    let selected = self.select_item();
                    let cb = self.on_select.clone().map(|on_select| {
                        Callback::from_fn(move |c| {
                            on_select(c, selected.clone());
                        })
                    });
                    return EventResult::Consumed(cb);
                }
                if focused == self.selected_idx as usize {
                    // on deselect
                    if let Some(deselected) = self.deselect_item() {
                        let cb = self.on_deselect.clone().map(|on_deselect| {
                            Callback::from_fn(move |c| {
                                on_deselect(c, deselected.clone());
                            })
                        });
                        return EventResult::Consumed(cb);
                    }
                }
                EventResult::Consumed(None)
            }
            _ => self.with_view_mut(|v| v.on_event(event))
                .unwrap_or(EventResult::Ignored),
        }
    }
}
