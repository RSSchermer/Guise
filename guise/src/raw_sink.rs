use futures::Sink;
use std::fmt::Debug;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) struct RawSink {
    vtable: VTable,
    ptr: *mut (),
}

impl RawSink {
    pub(crate) fn new<Item, S>(sink: S) -> Self
    where
        S: Sink<Item> + 'static,
        S::Error: Debug,
    {
        let vtable = VTable::new::<Item, S>();
        let ptr = Box::into_raw(Box::new(sink)) as *mut ();

        RawSink { vtable, ptr }
    }

    pub(crate) fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<()> {
        unsafe { (self.vtable.poll_ready)(self.ptr, context) }
    }

    /// # Unsafe
    ///
    /// `item` must be a unique pointer to a value of the item type of the sink from which this raw
    /// sink was created.
    pub(crate) unsafe fn start_send(&mut self, item: *mut ()) {
        (self.vtable.start_send)(self.ptr, item)
    }

    pub(crate) fn poll_flush(&mut self, context: &mut Context<'_>) -> Poll<()> {
        unsafe { (self.vtable.poll_flush)(self.ptr, context) }
    }
}

impl Drop for RawSink {
    fn drop(&mut self) {
        unsafe { (self.vtable.drop)(self.ptr) }
    }
}

struct VTable {
    poll_ready: unsafe fn(*mut (), &mut Context<'_>) -> Poll<()>,
    start_send: unsafe fn(*mut (), *mut ()),
    poll_flush: unsafe fn(*mut (), &mut Context<'_>) -> Poll<()>,
    drop: unsafe fn(*mut ()),
}

impl VTable {
    fn new<Item, S>() -> Self
    where
        S: Sink<Item>,
        S::Error: Debug,
    {
        VTable {
            poll_ready: poll_ready::<Item, S>,
            start_send: start_send::<Item, S>,
            poll_flush: poll_flush::<Item, S>,
            drop: drop::<S>,
        }
    }
}

unsafe fn poll_ready<Item, S>(sink: *mut (), context: &mut Context<'_>) -> Poll<()>
where
    S: Sink<Item>,
    S::Error: Debug,
{
    let ptr = &mut *(sink as *mut S);

    S::poll_ready(Pin::new_unchecked(ptr), context).map(|r| r.unwrap())
}

unsafe fn start_send<Item, S>(sink: *mut (), item: *mut ())
where
    S: Sink<Item>,
    S::Error: Debug,
{
    let ptr = &mut *(sink as *mut S);
    let item = *Box::from_raw(item as *mut Item);

    S::start_send(Pin::new_unchecked(ptr), item).unwrap()
}

unsafe fn poll_flush<Item, S>(sink: *mut (), context: &mut Context<'_>) -> Poll<()>
where
    S: Sink<Item>,
    S::Error: Debug,
{
    let ptr = &mut *(sink as *mut S);

    S::poll_flush(Pin::new_unchecked(ptr), context).map(|r| r.unwrap())
}

unsafe fn drop<S>(sink: *mut ()) {
    let reconstructed = Box::from_raw(sink as *mut S);

    mem::drop(reconstructed)
}
