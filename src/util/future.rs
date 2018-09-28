use futures::future::Future;

pub trait FutureExt: Future + Sized {
    fn into_box(self) -> Box<Future<Item = Self::Item, Error = Self::Error>>;
}

impl<F: Future + 'static> FutureExt for F {
    // TODO: when trait/impl specialization lands, try to implement this so that
    // it's a no-op when called on already boxed futures.
    fn into_box(self) -> Box<Future<Item = Self::Item, Error = Self::Error>> {
        Box::new(self)
    }
}