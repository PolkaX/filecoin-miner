mod error;

use async_std::task;

use futures::pin_mut;
use futures::select;
use futures::{future, task::Spawn, Future, FutureExt};

pub use error::Error;

/// Abstraction over a Substrate service.
pub trait AbstractService:
    'static + Future<Output = Result<(), Error>> + Spawn + Send + Unpin
{
    /// Get a handle to a future that will resolve on exit.
    fn on_exit(&self) -> ::exit_future::Exit;
}

#[cfg(target_family = "unix")]
async fn main<F, E>(func: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Future<Output = Result<(), E>> + future::FusedFuture,
    E: 'static + std::error::Error,
{
    use async_std::sync::channel;
    let (s, r) = channel::<()>(1);

    ctrlc::set_handler(move || {
        let s = s.clone();
        task::spawn(async move {
            s.send(()).await;
        });
    })
    .expect("Error setting Ctrl-C handler");

    let interrupt = async {
        r.recv().await;
    };

    let interrupt = interrupt.fuse();
    let t3 = func;

    pin_mut!(interrupt, t3);

    select! {
        _ = interrupt => {},
        res = t3 => res?,
    }
    Ok(())
}

#[cfg(not(unix))]
async fn main<F, E>(func: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Future<Output = Result<(), E>> + future::FusedFuture,
    E: 'static + std::error::Error,
{
    use tokio::signal::ctrl_c;

    let t1 = ctrl_c().fuse();
    let t2 = func;

    pin_mut!(t1, t2);

    select! {
        _ = t1 => {},
        res = t2 => res?,
    }

    Ok(())
}

pub fn run_service_until_exit<S: AbstractService + Unpin>(service: S) -> error::Result<()> {
    let f = service.fuse();
    pin_mut!(f);

    async_std::task::block_on(main(f)).map_err(|e| e.to_string())?;

    Ok(())
}
