use exit_future::{Exit, Signal};
use futures::future::FutureObj;
use futures::task::{Context, Poll, SpawnError};
use futures::{task::Spawn, Future};
use std::pin::Pin;

pub struct Mock {
    /// A future that resolves when the service has exited, this is useful to
    /// make sure any internally spawned futures stop when the service does.
    pub exit: Exit,
    /// A signal that makes the exit future above resolve, fired on service drop.
    pub signal: Option<Signal>,
}
impl node_service::AbstractService for Mock {
    fn on_exit(&self) -> Exit {
        self.exit.clone()
    }
}

impl Drop for Mock {
    fn drop(&mut self) {
        if let Some(signal) = self.signal.take() {
            let _ = signal.fire();
        }
    }
}

impl Future for Mock {
    type Output = Result<(), node_service::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = Pin::into_inner(self);
        // to spawn task
        Poll::Pending
    }
}

impl Spawn for Mock {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        // self.to_spawn_tx.unbounded_send((Box::pin(future), From::from("unnamed")))
        // 			.map_err(|_| SpawnError::shutdown())
        unimplemented!()
    }
}
