use crate::sink_spawner::SinkSpawner;
use crate::vdom::{Attribute, Element, Node, VDom};
use crate::ElementRef;
use arwa::collection::Sequence;
use arwa::dom::{
    CharacterData, ChildNode, Document, DynamicChildNode, DynamicDocument, DynamicElement,
    Element as ArwaElement, Name, OwnedNode, ParentNode, Text,
};
use arwa::html::HtmlInputElement;
use std::cmp::min;
use std::convert::TryFrom;

pub fn patch_dom<E>(container: &E, mut old: VDom, new: &mut VDom)
where
    E: ArwaElement + ParentNode + OwnedNode,
{
    let document = container.owner_document();

    old.with_nodes_mut(|old_nodes| {
        new.with_nodes_mut(|new_nodes| {
            patch_children(&document, container, old_nodes, new_nodes);
        });
    });
}

fn patch_node(
    document: &DynamicDocument,
    node: &DynamicChildNode,
    old: &mut Node,
    mut new: &mut Node,
) {
    match (old, &mut new) {
        (Node::Text(old), Node::Text(new)) => {
            if old != new {
                let text = Text::try_from(node.clone())
                    .expect("actual node type does not match v-node type");

                text.set_data(new);
            }

            return;
        }
        (Node::Element(old), Node::Element(new)) => {
            let element = DynamicElement::try_from(node.clone())
                .expect("actual node type does not match v-node type");

            if old.tag_name() == new.tag_name() && old.is() == new.is() {
                patch_attributes(&element, old.attributes(), new.attributes());
                patch_children(document, &element, old.children_mut(), new.children_mut());
                spawn_sinks(&element, new.sink_spawners_mut());
                set_ref_anchors(&element, new.element_refs_mut());

                return;
            }
        }
        _ => (),
    }

    replace_fresh(document, node, new);
}

fn patch_children<E>(document: &DynamicDocument, parent: &E, old: &mut [Node], new: &mut [Node])
where
    E: ArwaElement + ParentNode,
{
    let children = parent.child_nodes();
    let overlap = min(old.len(), new.len());

    for i in 0..overlap {
        let node = children.get(i as u32).unwrap();

        patch_node(document, &node, &mut old[i], &mut new[i])
    }

    let remove_count = old.len() - overlap;

    for _ in 0..remove_count {
        if let Some(child) = children.last() {
            child.disconnect();
        }
    }

    if new.len() > overlap {
        for i in overlap..new.len() {
            append_fresh(document, parent, &mut new[i])
        }
    }
}

fn patch_attributes(element: &DynamicElement, old: &[Attribute], new: &[Attribute]) {
    // Note: special treatment for the "checked" attribute, which maps to the `defaultChecked`
    // property, rather than the `checked` property. We explicitly set the `checked` property to
    // `true` whenever a "checked" attribute is added, and we set the `checked` property to `false`
    // whenever a "checked" attribute is removed.
    fn maybe_check(element: &DynamicElement, attr: &Name) {
        if unicase::eq_ascii(attr.as_ref(), "checked") {
            if let Ok(input) = TryInto::<HtmlInputElement>::try_into(element.clone()) {
                input.set_checked(true);
            }
        }
    }
    fn maybe_uncheck(element: &DynamicElement, attr: &Name) {
        if unicase::eq_ascii(attr.as_ref(), "checked") {
            if let Ok(input) = TryInto::<HtmlInputElement>::try_into(element.clone()) {
                input.set_checked(false);
            }
        }
    }

    let attributes = element.attributes();

    if old.is_empty() {
        for a in new {
            attributes.set(a.name(), a.value());

            maybe_check(element, a.name());
        }

        return;
    }

    if new.is_empty() {
        for a in old {
            attributes.remove(a.name());

            maybe_uncheck(element, a.name());
        }

        return;
    }

    // Add or update any attributes in the new vdom
    'outer: for a_new in new {
        'inner: for a_old in old {
            if a_new.name() == a_old.name() {
                if a_new.value() != a_old.value() {
                    break 'inner;
                }

                continue 'outer;
            }
        }

        attributes.set(a_new.name(), a_new.value());

        maybe_check(element, a_new.name());
    }

    // Remove any old attributes that arnt in the new vdom
    'outer2: for a_old in old {
        for a_new in new {
            if a_old.name() == a_new.name() {
                continue 'outer2;
            }
        }

        attributes.remove(a_old.name());

        maybe_uncheck(element, a_old.name());
    }
}

fn append_fresh<E>(document: &DynamicDocument, parent: &E, node: &mut Node)
where
    E: ParentNode,
{
    match node {
        Node::Text(text) => parent.append_child(&document.create_text(text)),
        Node::Element(element) => parent.append_child(&fresh_element(document, element)),
    }
}

fn replace_fresh(document: &DynamicDocument, target: &DynamicChildNode, node: &mut Node) {
    match node {
        Node::Text(text) => target.replace_with(&document.create_text(text)),
        Node::Element(element) => target.replace_with(&fresh_element(document, element)),
    }
}

fn fresh_element(document: &DynamicDocument, element: &mut Element) -> DynamicElement {
    let e = if let Some(is) = element.is() {
        document.create_customized_element(element.tag_name(), is)
    } else {
        document.create_element(element.tag_name())
    };

    let attributes = e.attributes();

    for a in element.attributes() {
        attributes.set(a.name(), a.value());
    }

    for node in element.children_mut() {
        append_fresh(document, &e, node);
    }

    spawn_sinks(&e, element.sink_spawners_mut());
    set_ref_anchors(&e, element.element_refs_mut());

    e
}

fn spawn_sinks(element: &DynamicElement, spawners: &mut [SinkSpawner]) {
    for spawner in spawners {
        spawner.spawn(&element);
    }
}

fn set_ref_anchors(element: &DynamicElement, element_refs: &mut [ElementRef]) {
    for element_ref in element_refs {
        element_ref.set_element(element.clone());
    }
}
