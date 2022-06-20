use arwa::dom::{name, DynamicElement};
use arwa::event::Event;
use arwa::html::{custom_element_name, GenericExtendableElement, HtmlInputElement};
use arwa::spawn_local;
use arwa::ui::{ClickEvent, InputEvent, KeyDownEvent, KeyboardEvent};
use atomic_counter::AtomicCounter;
use futures::{Stream, StreamExt};
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

    let new_todo_listener = Listener::new(|e: KeyDownEvent<DynamicElement>| {
        if &e.key() == "Enter" {
            let input: HtmlInputElement = e.current_target().unwrap().try_into().unwrap();
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

    let check_toggle_all_listener = Listener::new(move |e: InputEvent<DynamicElement>| {
        let input: HtmlInputElement = e.current_target().unwrap().try_into().unwrap();

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

        Listener::new(move |_: ClickEvent<DynamicElement>| {
            updater
                .update(|component| component.filter_mode = FilterMode::All)
                .unwrap();
        })
    };

    let change_filter_mode_active_listener = {
        let updater = view_model.updater();

        Listener::new(move |_: ClickEvent<DynamicElement>| {
            updater
                .update(|component| component.filter_mode = FilterMode::Active)
                .unwrap();
        })
    };

    let change_filter_mode_completed_listener = {
        let updater = view_model.updater();

        Listener::new(move |_: ClickEvent<DynamicElement>| {
            updater
                .update(|component| component.filter_mode = FilterMode::Completed)
                .unwrap();
        })
    };

    let clear_completed_listener = Listener::new(move |_: ClickEvent<DynamicElement>| {
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

        vdom.element(name!("div"), |mut container| {
            container.attribute(name!("class"), "todoapp");

            container.element(name!("header"), |mut header| {
                header.attribute(name!("class"), "header");
                header.element(name!("h1"), |mut h1| {
                    h1.text("todos");
                });
                header.element(name!("input"), |mut input| {
                    input.attribute(name!("type"), "text");
                    input.attribute(name!("class"), "new-todo");
                    input.attribute(name!("placeholder"), "What needs to be done?");
                    input.boolean_attribute(name!("autofocus"));
                    input.event_sink(new_todo_listener.clone());
                })
            });

            if !component.is_empty() {
                container.element(name!("section"), |mut main| {
                    main.attribute(name!("class"), "main");

                    main.element(name!("input"), |mut input| {
                        input.attribute(name!("type"), "checkbox");
                        input.attribute(name!("id"), "toggle-all");
                        input.attribute(name!("class"), "toggle-all");

                        if component.all_completed() {
                            input.boolean_attribute(name!("checked"));
                        }

                        input.event_sink(check_toggle_all_listener.clone());
                    });

                    main.element(name!("label"), |mut label| {
                        label.attribute(name!("for"), "toggle-all");

                        label.text("Mark all as complete");
                    });

                    main.element(name!("ul"), |mut list| {
                        list.attribute(name!("class"), "todo-list");

                        for id in component.render_ids() {
                            list.element_is(
                                name!("li"),
                                custom_element_name!("todo-item"),
                                |mut todo_item| {
                                    todo_item.attribute(name!("todo-id"), &id.to_string());
                                },
                            );
                        }
                    });
                });

                container.element(name!("footer"), |mut footer| {
                    footer.attribute(name!("class"), "footer");

                    footer.element(name!("span"), |mut span| {
                        span.attribute(name!("class"), "todo-count");

                        span.element(name!("strong"), |mut strong| {
                            strong.text(&component.active_count().to_string());
                        });

                        let text = if component.active_count() == 1 {
                            " item left"
                        } else {
                            " items left"
                        };

                        span.text(text);
                    });

                    footer.element(name!("ul"), |mut ul| {
                        ul.attribute(name!("class"), "filters");

                        ul.element(name!("li"), |mut li| {
                            li.element(name!("button"), |mut button| {
                                if component.filter_mode == FilterMode::All {
                                    button.attribute(name!("class"), "selected");
                                } else {
                                    button.event_sink(change_filter_mode_all_listener.clone());
                                }

                                button.text("All");
                            });
                        });

                        ul.element(name!("li"), |mut li| {
                            li.element(name!("button"), |mut button| {
                                if component.filter_mode == FilterMode::Active {
                                    button.attribute(name!("class"), "selected");
                                } else {
                                    button.event_sink(change_filter_mode_active_listener.clone());
                                }

                                button.text("Active");
                            });
                        });

                        ul.element(name!("li"), |mut li| {
                            li.element(name!("button"), |mut button| {
                                if component.filter_mode == FilterMode::Completed {
                                    button.attribute(name!("class"), "selected");
                                } else {
                                    button
                                        .event_sink(change_filter_mode_completed_listener.clone());
                                }

                                button.text("Complete");
                            });
                        });
                    });

                    if component.any_completed() {
                        footer.element(name!("button"), |mut button| {
                            button.attribute(name!("class"), "clear-completed");

                            button.text("Clear completed");

                            button.event_sink(clear_completed_listener.clone());
                        });
                    }
                });
            }
        });

        vdom
    })
}
