use std::ptr::NonNull;
use std::rc::Rc;
use std::marker;

use arwa::dom::DynamicElement;
use wasm_bindgen::JsCast;

// SAFETY - The pointer shared between all ElementRefs and the ElementAnchor will not escape the
// main thread and will only be mutated in a scope controlled by this library that cannot overlap
// with any scopes that could be dereferencing an ElementRef.

struct Internal {
    ptr: NonNull<Option<DynamicElement>>,
}

impl Drop for Internal {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr.as_ptr());
        }
    }
}

#[derive(Clone)]
pub struct ElementRef<T> {
    internal: Rc<Internal>,
    _marker: marker::PhantomData<T>
}

impl<T> ElementRef<T> {
    pub fn new() -> Self {
        let value = Box::new(None);
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(value)) };
        let internal = Rc::new(Internal { ptr });

        ElementRef {
            internal: internal.clone(),
            _marker: Default::default()
        }
    }

    pub fn get(&self) -> Option<&T> where T: JsCast {
        unsafe { self.internal.ptr.as_ref().as_ref().map(|e| e.unchecked_ref()) }
    }

    pub(crate) fn into_raw(self) -> RawElementRef{
        RawElementRef {
            internal: self.internal
        }
    }
}

pub(crate) struct RawElementRef {
    internal: Rc<Internal>
}

impl RawElementRef {
    pub(crate) fn set_element(&mut self, element: DynamicElement) {
        unsafe {
            *self.internal.ptr.as_ptr() = Some(element);
        }
    }
}
