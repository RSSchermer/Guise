use std::fmt::Debug;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use arwa::dom::DynamicElement;
use arwa::event::{EventTarget, OnEvent, TypedEvent};
use arwa::spawn_local;
use futures::future::AbortHandle;
use futures::ready;
use futures::stream::Abortable;
use futures::{Sink, Stream};

use crate::raw_sink::RawSink;

enum State {
    Unused(RawSink),
    Spawned(AbortHandle),
    Gone,
}

pub(crate) struct SinkSpawner {
    state: State,
    spawner: fn(&DynamicElement, RawSink) -> AbortHandle,
}

impl SinkSpawner {
    pub(crate) fn new<E, T, S>(sink: S) -> Self
    where
        E: EventTarget,
        T: TypedEvent<CurrentTarget = E> + 'static,
        S: Sink<T> + 'static,
        S::Error: Debug,
    {
        SinkSpawner {
            state: State::Unused(RawSink::new(sink)),
            spawner: spawn::<E, T>,
        }
    }

    pub(crate) fn spawn(&mut self, target: &DynamicElement) {
        let SinkSpawner { state, spawner } = self;

        if let State::Unused(sink) = mem::replace(state, State::Gone) {
            *state = State::Spawned(spawner(target, sink));
        } else {
            panic!("already spawned")
        }
    }
}

impl Drop for SinkSpawner {
    fn drop(&mut self) {
        if let State::Spawned(abort_handle) = &self.state {
            abort_handle.abort();
        }
    }
}

fn spawn<E: EventTarget, T: TypedEvent<CurrentTarget = E> + 'static>(
    target: &DynamicElement,
    sink: RawSink,
) -> AbortHandle {
    // Safety: the actual concrete type of the DynamicElement does not matter to Arwa, it is only
    // used to cast the `current_target` on events in the stream, which never leads to unsafety (at
    // worst incorrectness). The concrete type of the DynamicElement should also always match `E`,
    // this is internal to Guise.
    let target: &E = unsafe { mem::transmute(target) };
    let stream = target.on_typed_event::<T>();
    let (abort_handle, registration) = AbortHandle::new_pair();

    let stream = Abortable::new(stream, registration);

    spawn_local(SinkTask {
        stream,
        raw_sink: sink,
        buffered: None,
    });

    abort_handle
}

struct SinkTask<T> {
    stream: Abortable<OnEvent<T>>,
    raw_sink: RawSink,
    buffered: Option<*mut ()>,
}

impl<T> SinkTask<T> {
    fn start_send(&mut self, cx: &mut Context<'_>, event: *mut ()) -> Poll<()> {
        debug_assert!(self.buffered.is_none());

        match self.raw_sink.poll_ready(cx) {
            Poll::Ready(()) => {
                unsafe {
                    self.raw_sink.start_send(event);
                }

                Poll::Ready(())
            }
            Poll::Pending => {
                self.buffered = Some(event);

                Poll::Pending
            }
        }
    }
}

impl<T> Future for SinkTask<T>
where
    T: TypedEvent + 'static,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        if let Some(event) = this.buffered.take() {
            ready!(this.start_send(cx, event));
        }

        loop {
            let pinned = unsafe { Pin::new_unchecked(&mut this.stream) };

            match pinned.poll_next(cx) {
                Poll::Ready(Some(event)) => {
                    let ptr = Box::into_raw(Box::new(event)) as *mut ();

                    ready!(this.start_send(cx, ptr))
                }
                Poll::Ready(None) => {
                    ready!(this.raw_sink.poll_flush(cx));

                    return Poll::Ready(());
                }
                Poll::Pending => {
                    ready!(this.raw_sink.poll_flush(cx));

                    return Poll::Pending;
                }
            }
        }
    }
}
