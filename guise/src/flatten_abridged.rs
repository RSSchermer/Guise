use futures::stream::{FusedStream, Map};
use futures::{ready, Stream, StreamExt as BaseStreamExt};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Stream for the [`flatten`](super::StreamExt::flatten) method.
    #[derive(Debug)]
    #[must_use = "streams do nothing unless polled"]
    pub struct FlattenAbridged<St, U> {
        #[pin]
        stream: St,
        #[pin]
        next: Option<U>,
    }
}

impl<St, U> FlattenAbridged<St, U> {
    pub(super) fn new(stream: St) -> Self {
        Self { stream, next: None }
    }
}

impl<St> FusedStream for FlattenAbridged<St, St::Item>
where
    St: FusedStream,
    St::Item: Stream,
{
    fn is_terminated(&self) -> bool {
        self.next.is_none() && self.stream.is_terminated()
    }
}

impl<St> Stream for FlattenAbridged<St, St::Item>
where
    St: Stream,
    St::Item: Stream,
{
    type Item = <St::Item as Stream>::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        Poll::Ready(loop {
            if let Poll::Ready(s) = this.stream.as_mut().poll_next(cx) {
                if let Some(s) = s {
                    this.next.set(Some(s));
                } else {
                    break None;
                }
            } else if let Some(s) = this.next.as_mut().as_pin_mut() {
                if let Some(item) = ready!(s.poll_next(cx)) {
                    break Some(item);
                } else {
                    this.next.set(None);
                }
            } else {
                return Poll::Pending;
            }
        })
    }
}
pin_project! {
    pub struct FlatMapAbridged<St, U, F> {
        #[pin]
        inner: FlattenAbridged<Map<St, F>, U>
    }
}

impl<St, U, F> FlatMapAbridged<St, U, F>
where
    St: Stream,
    F: FnMut(St::Item) -> U,
    U: Stream,
{
    fn new(stream: St, f: F) -> Self {
        FlatMapAbridged {
            inner: FlattenAbridged::new(stream.map(f)),
        }
    }
}

impl<St, U, F> Stream for FlatMapAbridged<St, U, F>
where
    St: Stream,
    F: FnMut(St::Item) -> U,
    U: Stream,
{
    type Item = U::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}

pub trait StreamExt: Stream {
    fn flatten_abridged(self) -> FlattenAbridged<Self, Self::Item>
    where
        Self::Item: Stream,
        Self: Sized,
    {
        FlattenAbridged::new(self)
    }

    fn flat_map_abridged<U, F>(self, f: F) -> FlatMapAbridged<Self, U, F>
    where
        F: FnMut(Self::Item) -> U,
        U: Stream,
        Self: Sized,
    {
        FlatMapAbridged::new(self, f)
    }
}

impl<T> StreamExt for T where T: Stream {}
