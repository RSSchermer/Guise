use arwa::dom::name;
use arwa::event::Event;
use arwa::html::{custom_element_name, GenericExtendableElement, HtmlInputElement};
use arwa::spawn_local;
use arwa::ui::{InputEvent, KeyDownEvent, KeyboardEvent};
use atomic_counter::AtomicCounter;
use futures::{Stream, StreamExt};
use guise::vdom_builder_ext::*;
use guise::view_model::ViewModel;
use guise::{AttributesChanged, Listener, VDom};
use viemo::memo::OwnedMemo;
use viemo::versioned_cell::VersionedCell;
use viemo::watcher::Watcher3;

use crate::model::{TodoItem, APP_DATA, TODO_ID_PROVIDER};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FilterMode {
    Active,
    Completed,
    All,
}

impl Default for FilterMode {
    fn default() -> Self {
        FilterMode::All
    }
}

#[derive(Clone, Default)]
struct Component {
    all_todo_ids: Vec<usize>,
    active_todo_ids: Vec<usize>,
    completed_todo_ids: Vec<usize>,
    filter_mode: FilterMode,
}

impl Component {
    fn is_empty(&self) -> bool {
        self.all_todo_ids.len() == 0
    }

    fn render_ids(&self) -> &[usize] {
        match self.filter_mode {
            FilterMode::Active => &self.active_todo_ids,
            FilterMode::Completed => &self.completed_todo_ids,
            FilterMode::All => &self.all_todo_ids,
        }
    }

    fn active_count(&self) -> usize {
        self.active_todo_ids.len()
    }

    fn any_completed(&self) -> bool {
        self.completed_todo_ids.len() > 0
    }

    fn all_completed(&self) -> bool {
        self.completed_todo_ids.len() == self.all_todo_ids.len()
    }
}

pub fn init(_: &GenericExtendableElement, _: AttributesChanged<()>) -> impl Stream<Item = VDom> {
    let all_todo_ids = OwnedMemo::new(&APP_DATA, |app, cx| {
        app.todos.deref(cx).keys().copied().collect::<Vec<usize>>()
    });
    let active_todo_ids = OwnedMemo::new(&APP_DATA, |app, cx| {
        app.todos
            .deref(cx)
            .iter()
            .filter(|(_, todo)| !todo.deref(cx).complete)
            .map(|(id, _)| *id)
            .collect::<Vec<usize>>()
    });
    let completed_todo_ids = OwnedMemo::new(&APP_DATA, |app, cx| {
        app.todos
            .deref(cx)
            .iter()
            .filter(|(_, todo)| todo.deref(cx).complete)
            .map(|(id, _)| *id)
            .collect::<Vec<usize>>()
    });

    let view_model = ViewModel::new(Component::default());

    spawn_local({
        let updater = view_model.updater();

        Watcher3::new(
            &APP_DATA,
            all_todo_ids,
            active_todo_ids,
            completed_todo_ids,
            move |(all_todo_ids, active_todo_ids, completed_todo_ids), _| {
                updater
                    .update(|component| {
                        component.all_todo_ids.clear();
                        component.all_todo_ids.extend_from_slice(all_todo_ids);

                        component.active_todo_ids.clear();
                        component.active_todo_ids.extend_from_slice(active_todo_ids);

                        component.completed_todo_ids.clear();
                        component
                            .completed_todo_ids
                            .extend_from_slice(completed_todo_ids);
                    })
                    .ok()
            },
        )
        .for_each(async move |_| ())
    });

    let new_todo_listener = Listener::new(|e: KeyDownEvent<HtmlInputElement>| {
        if &e.key() == "Enter" {
            let input = e.current_target().unwrap();
            let value = input.value();
            let trimmed = value.trim();

            if !trimmed.is_empty() {
                APP_DATA.update(|app, cx| {
                    let mut todos = app.todos.borrow_mut(cx);

                    todos.insert(
                        TODO_ID_PROVIDER.inc(),
                        VersionedCell::new(
                            cx,
                            TodoItem {
                                note: trimmed.to_string(),
                                complete: false,
                            },
                        ),
                    );
                });
            }

            input.set_value("");
        }
    });

    let check_toggle_all_listener = Listener::new(move |e: InputEvent<HtmlInputElement>| {
        let input = e.current_target().unwrap();

        let complete = input.checked();

        APP_DATA.update(|app, cx| {
            let todos = app.todos.borrow(cx);

            for todo in todos.values() {
                let mut todo = todo.borrow_mut(cx);

                todo.complete = complete;
            }
        });
    });

    let change_filter_mode_all_listener = {
        let updater = view_model.updater();

        Listener::new(move |_| {
            updater
                .update(|component| component.filter_mode = FilterMode::All)
                .unwrap();
        })
    };

    let change_filter_mode_active_listener = {
        let updater = view_model.updater();

        Listener::new(move |_| {
            updater
                .update(|component| component.filter_mode = FilterMode::Active)
                .unwrap();
        })
    };

    let change_filter_mode_completed_listener = {
        let updater = view_model.updater();

        Listener::new(move |_| {
            updater
                .update(|component| component.filter_mode = FilterMode::Completed)
                .unwrap();
        })
    };

    let clear_completed_listener = Listener::new(move |_| {
        APP_DATA.update(|app, cx| {
            let mut todos = app.todos.borrow_mut(cx);

            todos.retain(|_, todo| {
                let todo = todo.borrow(cx);

                !todo.complete
            });
        })
    });

    view_model.rendered(move |component| {
        let mut vdom = VDom::new();

        vdom.child_div(|mut e| {
            e.attr_class("todoapp");

            e.child_header(|mut e| {
                e.attr_class("header");
                e.child_h1(|mut h1| {
                    h1.text("todos");
                });
                e.child_input(|mut input| {
                    input.attr_type("text");
                    input.attr_class("new-todo");
                    input.attr_placeholder("What needs to be done?");
                    input.attr_autofocus();
                    input.sink_key_down(new_todo_listener.clone());
                })
            });

            if !component.is_empty() {
                e.child_section(|mut e| {
                    e.attr_class("main");

                    e.child_input(|mut e| {
                        e.attr_type("checkbox");
                        e.attr_id("toggle-all");
                        e.attr_class("toggle-all");

                        if component.all_completed() {
                            e.attr_checked();
                        }

                        e.sink_input(check_toggle_all_listener.clone());
                    });

                    e.child_label(|mut e| {
                        e.attr_for("toggle-all");

                        e.text("Mark all as complete");
                    });

                    e.child_ul(|mut e| {
                        e.attr_class("todo-list");

                        for id in component.render_ids() {
                            e.child_customized(
                                name!("li"),
                                custom_element_name!("todo-item"),
                                |mut e| {
                                    e.attr(name!("todo-id"), &id.to_string());
                                },
                            );
                        }
                    });
                });

                e.child_footer(|mut e| {
                    e.attr_class("footer");

                    e.child_span(|mut e| {
                        e.attr_class("todo-count");

                        e.child_strong(|mut e| {
                            e.text(&component.active_count().to_string());
                        });

                        let text = if component.active_count() == 1 {
                            " item left"
                        } else {
                            " items left"
                        };

                        e.text(text);
                    });

                    e.child_ul(|mut e| {
                        e.attr_class("filters");

                        e.child_li(|mut e| {
                            e.child_button(|mut e| {
                                if component.filter_mode == FilterMode::All {
                                    e.attr_class("selected");
                                } else {
                                    e.sink_click(change_filter_mode_all_listener.clone());
                                }

                                e.text("All");
                            });
                        });

                        e.child_li(|mut e| {
                            e.child_button(|mut e| {
                                if component.filter_mode == FilterMode::Active {
                                    e.attr_class("selected");
                                } else {
                                    e.sink_click(change_filter_mode_active_listener.clone());
                                }

                                e.text("Active");
                            });
                        });

                        e.child_li(|mut e| {
                            e.child_button(|mut e| {
                                if component.filter_mode == FilterMode::Completed {
                                    e.attr_class("selected");
                                } else {
                                    e.sink_click(change_filter_mode_completed_listener.clone());
                                }

                                e.text("Complete");
                            });
                        });
                    });

                    if component.any_completed() {
                        e.child_button(|mut e| {
                            e.attr_class("clear-completed");

                            e.text("Clear completed");

                            e.sink_click(clear_completed_listener.clone());
                        });
                    }
                });
            }
        });

        vdom
    })
}
