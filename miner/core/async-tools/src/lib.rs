pub mod task_manager;

use std::future::Future;
use std::pin::Pin;

pub trait Executor {
    /// Run the given future in the background until it ends.
    fn exec(&self, future: Pin<Box<dyn Future<Output = ()> + Send>>);
}

impl<'a, T: ?Sized + Executor> Executor for &'a T {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        T::exec(&**self, f)
    }
}

impl<'a, T: ?Sized + Executor> Executor for &'a mut T {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        T::exec(&**self, f)
    }
}

impl<T: ?Sized + Executor> Executor for Box<T> {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        T::exec(&**self, f)
    }
}

pub struct SpawnImpl<F>(pub F);
impl<F: Fn(Pin<Box<dyn Future<Output = ()> + Send>>)> Executor for SpawnImpl<F> {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        (self.0)(f)
    }
}
