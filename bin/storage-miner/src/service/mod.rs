mod builder;
mod storageminer;

use std::borrow::Cow;
use std::pin::Pin;
use std::sync::Arc;

use async_std::task;
use exit_future::{Exit, Signal};
use futures::{
    channel::mpsc,
    future::{select, FutureObj},
    task::Spawn,
    task::{Context, Poll, SpawnError},
    Future, FutureExt, SinkExt, Stream,
};

use repo::{FsLockedRepo, Keystore, RepoDatastore};
use sectorbuilder::SectorBuilder;

use log::{debug, error};

pub use builder::ServiceBuilder;

pub struct Service {
    /// repo
    repo: FsLockedRepo,

    /// Sector Builder
    sectorbuilder: Arc<SectorBuilder<RepoDatastore>>,

    /// A future that resolves when the service has exited, this is useful to
    /// make sure any internally spawned futures stop when the service does.
    exit: Exit,
    /// A signal that makes the exit future above resolve, fired on service drop.
    signal: Option<Signal>,

    /// Send a signal when a spawned essential task has concluded. The next time
    /// the service future is polled it should complete with an error.
    essential_failed_tx: mpsc::UnboundedSender<()>,
    /// A receiver for spawned essential-tasks concluding.
    essential_failed_rx: mpsc::UnboundedReceiver<()>,
    /// Sender for futures that must be spawned as background tasks.
    to_spawn_tx:
        mpsc::UnboundedSender<(Pin<Box<dyn Future<Output = ()> + Send>>, Cow<'static, str>)>,
    /// Receiver for futures that must be spawned as background tasks.
    to_spawn_rx:
        mpsc::UnboundedReceiver<(Pin<Box<dyn Future<Output = ()> + Send>>, Cow<'static, str>)>,
}

impl Service {}

impl node_service::AbstractService for Service {
    fn locked_repo(&self) -> &FsLockedRepo {
        &self.repo
    }
    fn keystore(&self) -> Keystore {
        self.repo.keystore()
    }
    fn spawn_task(
        &self,
        name: impl Into<Cow<'static, str>>,
        task: impl Future<Output = ()> + Send + 'static,
    ) {
        let on_exit = self.on_exit();
        let task = async move {
            futures::pin_mut!(task);
            let _ = select(on_exit, task).await;
        };
        let _ = self
            .to_spawn_tx
            .unbounded_send((Box::pin(task), name.into()));
    }

    fn spawn_essential_task(
        &self,
        name: impl Into<Cow<'static, str>>,
        task: impl Future<Output = ()> + Send + 'static,
    ) {
        let mut essential_failed = self.essential_failed_tx.clone();
        let essential_task = std::panic::AssertUnwindSafe(task)
            .catch_unwind()
            .map(move |_| {
                error!("Essential task failed. Shutting down service.");
                let _ = essential_failed.send(());
            });
        let on_exit = self.on_exit();
        let task = async move {
            futures::pin_mut!(essential_task);
            let _ = select(on_exit, essential_task).await;
        };

        let _ = self
            .to_spawn_tx
            .unbounded_send((Box::pin(task), name.into()));
    }

    fn on_exit(&self) -> Exit {
        self.exit.clone()
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        debug!(target: "service", "service shutdown.");
        if let Some(signal) = self.signal.take() {
            let _ = signal.fire();
        }
    }
}

impl Future for Service {
    type Output = Result<(), node_service::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = Pin::into_inner(self);
        // to spawn task
        match Pin::new(&mut this.essential_failed_rx).poll_next(cx) {
            Poll::Pending => {}
            Poll::Ready(_) => {
                // Ready(None) should not be possible since we hold a live
                // sender.
                return Poll::Ready(Err(node_service::Error::Other(
                    "Essential task failed.".into(),
                )));
            }
        }

        while let Poll::Ready(Some((task_to_spawn, name))) =
            Pin::new(&mut this.to_spawn_rx).poll_next(cx)
        {
            task::spawn(Box::pin(futures_diagnose::diagnose(name, task_to_spawn)));
        }

        // The service future never ends.
        Poll::Pending
    }
}

impl Spawn for Service {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        self.to_spawn_tx
            .unbounded_send((Box::pin(future), From::from("unnamed")))
            .map_err(|_| SpawnError::shutdown())
    }
}
