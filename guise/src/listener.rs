use futures::Sink;
use std::marker;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Listener<T, F> {
    f: F,
    _marker: marker::PhantomData<*const T>,
}

impl<T, F> Listener<T, F>
where
    F: FnMut(T) + Unpin,
{
    pub fn new(f: F) -> Self {
        Listener {
            f,
            _marker: Default::default(),
        }
    }
}

impl<T, F> Sink<T> for Listener<T, F>
where
    F: FnMut(T) + Unpin,
{
    type Error = ();

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let this = self.get_mut();

        (this.f)(item);

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T, F> Clone for Listener<T, F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Listener {
            f: self.f.clone(),
            _marker: Default::default(),
        }
    }
}
