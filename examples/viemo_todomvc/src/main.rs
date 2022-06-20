#![feature(async_closure, generic_associated_types)]

mod model;
mod todo_app;
mod todo_item;

use arwa::html::custom_element_name;
use arwa::window::window;

fn main() {
    let registry = window().custom_elements();

    guise::register(
        &registry,
        &custom_element_name!("todo-item"),
        todo_item::init,
    );
    guise::register(&registry, &custom_element_name!("todo-app"), todo_app::init);
}
