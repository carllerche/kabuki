use futures::Future;
use tokio_core::reactor::{Core, Handle, Remote};

/// Value that can spawn an actor
pub trait Spawn<T: Future<Item = (), Error = ()>> {
    /// Spawn the provided actor
    fn spawn(&self, future: T);
}

impl<T: Future<Item = (), Error = ()> + 'static> Spawn<T> for Handle {
    fn spawn(&self, future: T) {
        self.spawn(future);
    }
}

impl<T: Future<Item = (), Error = ()> + 'static> Spawn<T> for Core {
    fn spawn(&self, future: T) {
        self.handle().spawn(future);
    }
}

impl<T: Future<Item = (), Error = ()> + Send + 'static> Spawn<T> for Remote {
    fn spawn(&self, future: T) {
        self.spawn(move |_| future);
    }
}
