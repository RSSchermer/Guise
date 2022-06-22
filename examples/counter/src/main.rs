use std::str::FromStr;

use arwa::html::{custom_element_name, GenericExtendableElement};
use arwa::spawn_local;
use arwa::window::window;
use futures::StreamExt;
use guise::vdom_builder_ext::*;
use guise::view_model::ViewModel;
use guise::{Listener, VDom};

#[derive(guise::Attributes, Clone, Default)]
struct CounterAttributes {
    #[attribute_name = "initial-count"]
    initial_count: Option<String>,
}

fn main() {
    let registry = window().custom_elements();

    guise::register::<GenericExtendableElement, CounterAttributes, _, _>(
        &registry,
        &custom_element_name!("x-counter"),
        |_, mut attribute_changes| {
            let view_model = ViewModel::new(0u32);

            spawn_local({
                let updater = view_model.updater();

                async move {
                    while let Some(change) = attribute_changes.next().await {
                        updater
                            .update(|count| {
                                *count = change
                                    .initial_count
                                    .as_ref()
                                    .and_then(|c| u32::from_str(c).ok())
                                    .unwrap_or(0)
                            })
                            .unwrap();
                    }
                }
            });

            let click_listener = {
                let updater = view_model.updater();

                Listener::new(move |_| {
                    updater.update(|count| *count += 1).unwrap();
                })
            };

            view_model.rendered(move |count| {
                let mut vdom = VDom::new();

                vdom.text(&count.to_string());
                vdom.child_button(|mut e| {
                    e.sink_click(click_listener.clone());
                    e.text("Increment!")
                });

                vdom
            })
        },
    );
}
