use std::fmt::Debug;

use arwa::dom::{DynamicElement, Name};
use arwa::event::TypedEvent;
use arwa::html::CustomElementName;
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::Sink;
use ouroboros::self_referencing;

use crate::sink_spawner::SinkSpawner;
use crate::ElementRef;

pub struct VDom {
    pub(crate) internal: VDomInternal,
    pub(crate) on_rendered: Option<Box<dyn FnOnce(&DynamicElement)>>,
}

impl VDom {
    pub fn new() -> Self {
        let alloc = Bump::new();

        VDom {
            internal: VDomInternal::new(alloc, |alloc| &alloc, |alloc| BumpVec::new_in(alloc)),
            on_rendered: None,
        }
    }

    pub fn text(&mut self, text: &str) {
        self.internal.with_mut(|fields| {
            let text = fields.alloc_ref.alloc_str(text);

            fields.nodes.push(Node::Text(text))
        });
    }

    fn element_internal<F>(&mut self, tag_name: Name, is: Option<CustomElementName>, f: F)
    where
        F: FnOnce(ElementBuilder),
    {
        self.internal.with_mut(|fields| {
            let tag_name = fields.alloc_ref.alloc(tag_name);
            let is = is.map(|n| fields.alloc_ref.alloc(n));

            let mut element = Element {
                tag_name,
                is,
                attributes: BumpVec::new_in(fields.alloc_ref),
                children: BumpVec::new_in(fields.alloc_ref),
                sink_spawners: BumpVec::new_in(fields.alloc_ref),
                element_refs: BumpVec::new_in(fields.alloc_ref),
            };

            f(ElementBuilder {
                alloc: fields.alloc_ref,
                element: &mut element,
            });

            fields.nodes.push(Node::Element(element))
        });
    }

    pub fn element<F>(&mut self, tag_name: Name, f: F)
    where
        F: FnOnce(ElementBuilder),
    {
        self.element_internal(tag_name, None, f);
    }

    pub fn element_is<F>(&mut self, tag_name: Name, is: CustomElementName, f: F)
    where
        F: FnOnce(ElementBuilder),
    {
        self.element_internal(tag_name, Some(is), f);
    }

    pub fn on_rendered<F>(&mut self, f: F)
    where
        F: FnOnce(&DynamicElement) + 'static,
    {
        self.on_rendered = Some(Box::new(f))
    }

    pub(crate) fn with_nodes_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Node]),
    {
        self.internal.with_nodes_mut(|nodes| f(nodes));
    }
}

#[self_referencing]
pub struct VDomInternal {
    alloc: Bump,
    #[borrows(alloc)]
    alloc_ref: &'this Bump,
    #[covariant]
    #[borrows(alloc)]
    nodes: BumpVec<'this, Node<'this>>,
}

pub struct ElementBuilder<'a, 'b> {
    alloc: &'b Bump,
    element: &'a mut Element<'b>,
}

impl<'a, 'b> ElementBuilder<'a, 'b> {
    pub fn attribute(&mut self, name: Name, value: &str) {
        let name = self.alloc.alloc(name);
        let value = self.alloc.alloc_str(value);

        self.element.attributes.push(Attribute { name, value });
    }

    pub fn boolean_attribute(&mut self, name: Name) {
        let name = self.alloc.alloc(name);

        self.element.attributes.push(Attribute { name, value: "" });
    }

    pub fn text(&mut self, text: &str) {
        let text = self.alloc.alloc_str(text);

        self.element.children.push(Node::Text(text));
    }

    fn element_internal<F>(&mut self, tag_name: Name, is: Option<CustomElementName>, f: F)
    where
        F: FnOnce(ElementBuilder),
    {
        let tag_name = self.alloc.alloc(tag_name);
        let is = is.map(|n| self.alloc.alloc(n));

        let mut element = Element {
            tag_name,
            is,
            attributes: BumpVec::new_in(self.alloc),
            children: BumpVec::new_in(self.alloc),
            sink_spawners: BumpVec::new_in(self.alloc),
            element_refs: BumpVec::new_in(self.alloc),
        };

        f(ElementBuilder {
            alloc: self.alloc,
            element: &mut element,
        });

        self.element.children.push(Node::Element(element));
    }

    pub fn element<F>(&mut self, tag_name: Name, f: F)
    where
        F: FnOnce(ElementBuilder),
    {
        self.element_internal(tag_name, None, f);
    }

    pub fn element_is<F>(&mut self, tag_name: Name, is: CustomElementName, f: F)
    where
        F: FnOnce(ElementBuilder),
    {
        self.element_internal(tag_name, Some(is), f);
    }

    pub fn event_sink<T, S>(&mut self, sink: S)
    where
        T: TypedEvent<CurrentTarget = DynamicElement> + 'static,
        S: Sink<T> + 'static,
        S::Error: Debug,
    {
        self.element.sink_spawners.push(SinkSpawner::new(sink));
    }

    pub fn element_ref(&mut self, element_ref: ElementRef) {
        self.element.element_refs.push(element_ref);
    }
}

pub(crate) enum Node<'a> {
    Text(&'a str),
    Element(Element<'a>),
}

pub(crate) struct Element<'a> {
    tag_name: &'a Name,
    is: Option<&'a mut CustomElementName>,
    attributes: BumpVec<'a, Attribute<'a>>,
    children: BumpVec<'a, Node<'a>>,
    sink_spawners: BumpVec<'a, SinkSpawner>,
    element_refs: BumpVec<'a, ElementRef>,
}

impl<'a> Element<'a> {
    pub(crate) fn tag_name(&self) -> &Name {
        &self.tag_name
    }

    pub(crate) fn is(&self) -> Option<&CustomElementName> {
        self.is.as_deref()
    }

    pub(crate) fn attributes(&self) -> &[Attribute<'a>] {
        &self.attributes
    }

    pub(crate) fn children_mut(&mut self) -> &mut [Node<'a>] {
        &mut self.children
    }

    pub(crate) fn sink_spawners_mut(&mut self) -> &mut [SinkSpawner] {
        &mut self.sink_spawners
    }

    pub(crate) fn element_refs_mut(&mut self) -> &mut [ElementRef] {
        &mut self.element_refs
    }
}

pub(crate) struct Attribute<'a> {
    name: &'a Name,
    value: &'a str,
}

impl<'a> Attribute<'a> {
    pub(crate) fn name(&self) -> &Name {
        &self.name
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}
