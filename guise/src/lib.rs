#![feature(allocator_api)]

mod attributes;
mod element_ref;
mod id_sink;
mod listener;
mod patch_dom;
mod raw_sink;
mod sink_spawner;
mod vdom;

pub mod flatten_abridged;
pub mod vdom_builder_ext;
pub mod view_model;

use std::cell::RefCell;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

use arwa::dom::{Element, OwnedNode, ParentNode, ShadowHost, ShadowRootOptions};
use arwa::html::{
    AttributeChange, CustomElement, CustomElementDescriptor, CustomElementName,
    CustomElementRegistry, ExtendableElement,
};
use arwa::spawn_local;
use futures::stream::{abortable, AbortHandle};
use futures::{Stream, StreamExt};
use wasm_bindgen::{JsCast, JsValue};

use crate::patch_dom::patch_dom;

pub use crate::attributes::{Attribute, Attributes};
pub use crate::element_ref::ElementRef;
pub use crate::id_sink::IdSink;
pub use crate::listener::Listener;
pub use crate::vdom::VDom;

pub use guise_macro::Attributes;

#[doc(hidden)]
pub use arwa::dom::{name, Name};

struct ComponentData<A> {
    attribute_change_director: Rc<RefCell<AttributeChangeDirector<A>>>,
    last_vdom: RefCell<Option<VDom>>,
    abort_handle: RefCell<Option<AbortHandle>>,
}

struct AttributeChangeDirector<A> {
    attributes: A,
    waker: Option<Waker>,
    disconnected: bool,
}

pub struct AttributesChanged<A> {
    director: Rc<RefCell<AttributeChangeDirector<A>>>,
}

impl<A> Stream for AttributesChanged<A>
where
    A: Attributes,
{
    type Item = A;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let mut director = this.director.borrow_mut();

        if director.disconnected {
            Poll::Ready(None)
        } else if director.waker.is_some() {
            Poll::Pending
        } else {
            director.waker = Some(cx.waker().clone());

            Poll::Ready(Some(director.attributes.clone()))
        }
    }
}

pub fn register<E, A, S, F>(registry: &CustomElementRegistry, name: &CustomElementName, mut init: F)
where
    E: Element + ParentNode + OwnedNode + ExtendableElement + Clone + AsRef<JsValue> + 'static,
    A: Attributes + 'static,
    S: Stream<Item = VDom> + Unpin + 'static,
    F: FnMut(&E, AttributesChanged<A>) -> S + 'static,
{
    let descriptor = CustomElementDescriptor::new(move |_: &E| ComponentData {
        attribute_change_director: Rc::new(RefCell::new(AttributeChangeDirector {
            attributes: A::default(),
            waker: None,
            disconnected: true,
        })),
        last_vdom: RefCell::new(None),
        abort_handle: RefCell::new(None),
    })
    .connected_callback(move |element| {
        let element = element.clone();
        let director = element.data().attribute_change_director.clone();

        {
            let mut director = director.borrow_mut();

            director.disconnected = false;
        }

        let attributes_changed = AttributesChanged { director };
        let (mut vdoms, abort_handle) = abortable(init(element.deref(), attributes_changed));

        element.data().abort_handle.replace(Some(abort_handle));

        let document = element
            .owner_document()
            .try_into()
            .expect("Guise only supports HTML documents");

        spawn_local(async move {
            while let Some(mut new) = vdoms.next().await {
                let mut last_vdom = element.data().last_vdom.borrow_mut();

                let old = last_vdom.take().unwrap_or(VDom::new());

                patch_dom(&document, element.deref(), old, &mut new);

                if let Some(on_rendered) = new.on_rendered.take() {
                    let js_ref: &JsValue = element.as_ref();

                    on_rendered(js_ref.unchecked_ref());
                }

                // Note: this drops the previous vdom (if any), which should abort all old sink
                // tasks.
                *last_vdom = Some(new);
            }
        });
    })
    .disconnected_callback(disconnected_callback::<A, E>)
    .attribute_changed_callback(A::OBSERVED, attribute_changed_callback::<A, E>);

    registry.register(name, descriptor);
}

pub fn register_with_shadow_root<E, A, S, F>(
    registry: &CustomElementRegistry,
    shadow_root_options: ShadowRootOptions,
    name: &CustomElementName,
    mut init: F,
) where
    E: ShadowHost + Element + ParentNode + OwnedNode + ExtendableElement + Clone + AsRef<JsValue> + 'static,
    A: Attributes + 'static,
    S: Stream<Item = VDom> + Unpin + 'static,
    F: FnMut(&E, AttributesChanged<A>) -> S + 'static,
{
    let descriptor = CustomElementDescriptor::new(move |element: &E| {
        element.attach_shadow(shadow_root_options);

        ComponentData {
            attribute_change_director: Rc::new(RefCell::new(AttributeChangeDirector {
                attributes: A::default(),
                waker: None,
                disconnected: true,
            })),
            last_vdom: RefCell::new(None),
            abort_handle: RefCell::new(None),
        }
    })
    .connected_callback(move |element| {
        let element = element.clone();
        let director = element.data().attribute_change_director.clone();

        {
            let mut director = director.borrow_mut();

            director.disconnected = false;
        }

        let attributes_changed = AttributesChanged { director };
        let (mut vdoms, abort_handle) = abortable(init(element.deref(), attributes_changed));

        element.data().abort_handle.replace(Some(abort_handle));

        let document = element
            .owner_document()
            .try_into()
            .expect("Guise only supports HTML documents");
        let shadow_root = element.shadow_root().unwrap();

        spawn_local(async move {
            while let Some(mut new) = vdoms.next().await {
                let mut last_vdom = element.data().last_vdom.borrow_mut();

                let old = last_vdom.take().unwrap_or(VDom::new());

                patch_dom(&document, &shadow_root, old, &mut new);

                if let Some(on_rendered) = new.on_rendered.take() {
                    let js_ref: &JsValue = element.as_ref();

                    on_rendered(js_ref.unchecked_ref());
                }

                // Note: this drops the previous vdom (if any), which should abort all old sink
                // tasks.
                *last_vdom = Some(new);
            }
        });
    })
    .disconnected_callback(disconnected_callback::<A, E>)
    .attribute_changed_callback(A::OBSERVED, attribute_changed_callback::<A, E>);

    registry.register(name, descriptor);
}

fn disconnected_callback<A, E>(element: &CustomElement<ComponentData<A>, E>)
where
    A: Default + 'static,
{
    if let Some(abort_handle) = element.data().abort_handle.borrow_mut().take() {
        abort_handle.abort();
    }

    let mut director = element.data().attribute_change_director.borrow_mut();

    if let Some(waker) = director.waker.take() {
        waker.wake();
    }

    director.attributes = A::default();
    director.disconnected = true;
}

fn attribute_changed_callback<A, E>(
    element: &CustomElement<ComponentData<A>, E>,
    change: AttributeChange,
) where
    A: Attributes + 'static,
{
    let mut director = element.data().attribute_change_director.borrow_mut();

    director
        .attributes
        .update(&change.attribute_name, change.new_value);

    // Wake the attributes changed task. If there's no waker, assume the task is already
    // awake and queued to be polled. Note that if multiple attributes change at once, this
    // callback should be queued as multiple consecutive micro-tasks; only the first will wake
    // the attributes changed task, and the browser will enqueue it after all callback tasks, so
    // it will run only once.
    if let Some(waker) = director.waker.take() {
        waker.wake();
    }
}
