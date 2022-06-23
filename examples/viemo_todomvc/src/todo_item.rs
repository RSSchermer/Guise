use std::str::FromStr;

use arwa::dom::SelectionDirection;
use arwa::event::Event;
use arwa::html::{HtmlElement, HtmlInputElement, HtmlLiElement};
use arwa::spawn_local;
use arwa::ui::{FocusOutEvent, InputEvent, KeyDownEvent, KeyboardEvent};
use futures::{Stream, StreamExt as BaseStreamExt};
use guise::flatten_abridged::StreamExt as FlattenAbridgedStreamExt;
use guise::vdom_builder_ext::*;
use guise::view_model::ViewModel;
use guise::{AttributesChanged, ElementRef, Listener, VDom};
use viemo::memo::OptionCellMemo;
use viemo::watcher::Watcher;

use crate::model::APP_DATA;

#[derive(guise::Attributes, Clone, Default)]
pub struct Attributes {
    #[attribute_name = "todo-id"]
    id: Option<String>,
}

#[derive(Default)]
struct Component {
    note: String,
    complete: bool,
    edit_mode: bool,
}

pub fn init(
    _: &HtmlLiElement,
    attribute_changes: AttributesChanged<Attributes>,
) -> impl Stream<Item = VDom> {
    attribute_changes.flat_map_abridged(|attributes| {
        let id =
            usize::from_str(attributes.id.as_deref().unwrap_or_default()).expect("invalid index");
        let view_model = ViewModel::new(Component::default());
        let edit_ref = ElementRef::new();

        let todo = OptionCellMemo::new(&APP_DATA, move |app, cx| app.todos.deref(cx).get(id));

        spawn_local({
            let updater = view_model.updater();

            Watcher::new(&APP_DATA, todo, move |todo, cx| {
                todo.and_then(|todo| {
                    let todo = todo.deref(cx);

                    updater
                        .update(|component| {
                            component.note = todo.note.clone();
                            component.complete = todo.complete;
                        })
                        .ok()
                })
            })
            .for_each(async move |_| ())
        });

        let save = {
            let updater = view_model.updater();

            move |note: String| {
                APP_DATA.update(|app, cx| {
                    let todos = app.todos.borrow(cx);

                    if let Some(todo) = todos.get(id) {
                        let mut todo = todo.borrow_mut(cx);

                        todo.note = note;
                    }
                });

                updater
                    .update(|component| {
                        component.edit_mode = false;
                    })
                    .unwrap();
            }
        };

        let save_enter_listener = Listener::new({
            let save = save.clone();

            move |e: KeyDownEvent<HtmlInputElement>| {
                if &e.key() == "Enter" {
                    let input = e.current_target().unwrap();

                    save(input.value());
                }
            }
        });

        let save_blur_listener = Listener::new({
            let save = save.clone();

            move |e: FocusOutEvent<HtmlInputElement>| {
                let input = e.current_target().unwrap();

                save(input.value());
            }
        });

        let check_complete_listener = Listener::new(move |e: InputEvent<HtmlInputElement>| {
            let input = e.current_target().unwrap();

            APP_DATA.update(|app, cx| {
                let todos = app.todos.borrow(cx);

                if let Some(todo) = todos.get(id) {
                    let mut todo = todo.borrow_mut(cx);

                    todo.complete = input.checked();
                }
            })
        });

        let enter_edit_mode_listener = Listener::new({
            let updater = view_model.updater();

            move |_| {
                updater
                    .update(|component| {
                        component.edit_mode = true;
                    })
                    .unwrap();
            }
        });

        let destroy_listener = Listener::new(move |_| {
            APP_DATA.update(|app, cx| {
                let mut todos = app.todos.borrow_mut(cx);

                todos.remove(id);
            })
        });

        view_model.rendered(move |component| {
            let mut vdom = VDom::new();

            vdom.child_div(|mut e| {
                let mut class = String::new();

                if component.complete {
                    class.push_str(" completed");
                }

                if component.edit_mode {
                    class.push_str(" editing");
                }

                e.attr_class(&class);

                if component.edit_mode {
                    e.child_input(|mut e| {
                        e.attr_type("text");
                        e.attr_class("edit");
                        e.attr_autofocus();
                        e.attr_value(&component.note);

                        e.sink_key_down(save_enter_listener.clone());
                        e.sink_focus_out(save_blur_listener.clone());

                        // Attach an `ElementRef` to this element. When the VDom gets rendered, a
                        // reference to element associated with this VDom node will be stored
                        // inside the `ElementRef`. We'll use this on the VDom's `on_rendered`
                        // callback (see below) to set cursor inside this text input.
                        e.element_ref(edit_ref.clone());
                    });
                } else {
                    e.child_div(|mut e| {
                        e.attr_class("view");

                        e.child_input(|mut e| {
                            e.attr_type("checkbox");
                            e.attr_class("toggle");

                            if component.complete {
                                e.attr_checked();
                            }

                            e.sink_input(check_complete_listener.clone());
                        });

                        e.child_label(|mut e| {
                            e.sink_dbl_click(enter_edit_mode_listener.clone());

                            e.text(&component.note);
                        });

                        e.child_button(|mut e| {
                            e.attr_class("destroy");

                            e.sink_click(destroy_listener.clone());
                        });
                    })
                }
            });

            // We've attached an `ElementRef` to the note `edit` input element earlier. We'll use
            // that "ref" and the `on_rendered` callback on the vdom to set the cursor at the end
            // of the text in the input, if it is being rendered.
            vdom.on_rendered({
                let edit_ref = edit_ref.clone();

                move |_| {
                    if let Some(input) = edit_ref.get() {
                        let end = input.value().len() as u32;

                        input.set_selection(end..end, SelectionDirection::Forward);
                        input.focus()
                    }
                }
            });

            vdom
        })
    })
}
