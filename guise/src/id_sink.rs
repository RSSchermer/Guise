use futures::Sink;

pub trait IdSink<T>: Sink<T> {
    fn id(&self) -> u64;
}
