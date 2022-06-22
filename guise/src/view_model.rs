use std::cell::{Cell, RefCell};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

use futures::Stream;

use crate::VDom;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Gone;

struct InnerState<T> {
    value: T,
    waker: Option<Waker>,
}

struct State<T> {
    inner: Rc<RefCell<InnerState<T>>>,
    gone: Rc<Cell<bool>>,
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        State {
            inner: self.inner.clone(),
            gone: self.gone.clone(),
        }
    }
}

struct ViewModelInternal<T> {
    state: State<T>,
}

impl<T> Drop for ViewModelInternal<T> {
    fn drop(&mut self) {
        self.state.gone.replace(true);
    }
}

pub struct ViewModel<T> {
    internal: ViewModelInternal<T>,
}

impl<T> ViewModel<T> {
    pub fn new(initial: T) -> Self {
        ViewModel {
            internal: ViewModelInternal {
                state: State {
                    inner: Rc::new(RefCell::new(InnerState {
                        value: initial,
                        waker: None,
                    })),
                    gone: Rc::new(Cell::new(false)),
                },
            },
        }
    }

    pub fn updater(&self) -> Updater<T> {
        Updater {
            internal: self.internal.state.clone(),
        }
    }

    pub fn rendered<F>(self, f: F) -> Rendered<T, F>
    where
        F: FnMut(&T) -> VDom + Unpin,
    {
        Rendered {
            internal: self.internal,
            f,
        }
    }
}

pub struct Updater<T> {
    internal: State<T>,
}

impl<T> Updater<T> {
    pub fn update<F>(&self, f: F) -> Result<(), Gone>
    where
        F: FnOnce(&mut T),
    {
        if self.internal.gone.get() {
            return Err(Gone);
        }

        let mut state = self.internal.inner.borrow_mut();

        f(&mut state.value);

        if let Some(waker) = state.waker.take() {
            waker.wake();
        }

        Ok(())
    }
}

impl<T> Clone for Updater<T> {
    fn clone(&self) -> Self {
        Updater {
            internal: self.internal.clone(),
        }
    }
}

pub struct Rendered<T, F> {
    internal: ViewModelInternal<T>,
    f: F,
}

impl<T, F> Stream for Rendered<T, F>
where
    F: FnMut(&T) -> VDom + Unpin,
{
    type Item = VDom;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let mut state = this.internal.state.inner.borrow_mut();

        if state.waker.is_some() {
            Poll::Pending
        } else {
            state.waker = Some(cx.waker().clone());

            let vdom = (this.f)(&state.value);

            Poll::Ready(Some(vdom))
        }
    }
}

impl<T, F> Drop for Rendered<T, F> {
    fn drop(&mut self) {
        self.internal.state.gone.replace(true);
    }
}
