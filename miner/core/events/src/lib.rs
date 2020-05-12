pub mod error;
mod tscache;

use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use plum_tipset::{Tipset, TipsetKey};

use api::{ChainApi, HeadChangeType, HeadChange};
use futures::stream::StreamExt;
use async_std::task::sleep;

use crate::error::*;
use crate::tscache::TipSetCache;

use log::{error, info, warn};

const TARGET: &'static str = "events";

pub type HeightHandler = Box<dyn Fn(&Tipset, u64) -> Result<()>>;
pub type RevertHandler = Box<dyn Fn(&Tipset) -> Result<()>>;

pub type TriggerId = u64;
pub type TriggerHeight = u64;
pub type MsgHeight = u64;

struct HeightHandlerObj {
    confidence: u64,
    called: bool,

    handle: HeightHandler,
    revert: RevertHandler,
}

#[derive(Default)]
struct EventsHeight {
    height_triggers: BTreeMap<TriggerId, HeightHandlerObj>,
    ht_trigger_heights: BTreeMap<TriggerHeight, Vec<TriggerId>>,
    ht_heights: BTreeMap<MsgHeight, Vec<TriggerId>>,
}

pub struct Events {
    ts_cache: TipSetCache,
    gc_confidence: u64,
    //
    ctr: TriggerId,
    events_height: EventsHeight,
    // todo events_call
}

impl Events {
    pub fn new<Api: ChainApi>(
        api: Arc<Api>,
    ) -> (Arc<RwLock<Events>>, Box<dyn Future<Output = ()> + 'static>) {
        let confidence = 2 * plum_params::params().fork_length_threshold;

        let api_to_get_storage = api.clone();
        let storage = Box::new(move |height: u64, key: &TipsetKey| -> Result<Tipset> {
            async_std::task::block_on(async {
                api_to_get_storage
                    .chain_get_tipset_by_height(height, &key)
                    .await
                    .map_err(|e| EventsError::Other(Box::new(e)))
            })
        });

        let e = Events {
            ts_cache: TipSetCache::new(confidence as usize, storage),
            gc_confidence: confidence,
            ctr: 0,
            events_height: Default::default(),
        };
        let s = Arc::new(RwLock::new(e));
        let listen = Box::new(listen_head_changes(api, s.clone()));
        (s, listen)
    }

    pub fn head_change(&mut self, reverts: Vec<Tipset>, applies: Vec<Tipset>) -> Result<()> {
        self.head_change_at(&reverts, &applies)?;
        // TODO head_change_called
        Ok(())
    }
}

fn revert_func(h: u64, events_height: &mut EventsHeight, ts: &Tipset) -> Result<()> {
    for tid in &events_height.ht_heights[&h] {
        let mut handle = events_height.height_triggers.get_mut(tid).expect("");
        let r = (handle.revert)(ts);
        if let Err(e) = r {
            error!(
                target: TARGET,
                "reverting chain trigger (@H {}): {:?}", h, e
            );
        }
        handle.called = false;
    }
    Ok(())
}

fn apply_func(
    h: u64,
    events_height: &mut EventsHeight,
    ts_cache: &TipSetCache,
    ts: &Tipset,
) -> Result<()> {
    for tid in events_height.ht_trigger_heights[&h].iter() {
        // let hnd = &mut events_height.height_triggers[tid];
        let hnd = events_height.height_triggers.get_mut(tid).expect("");
        if hnd.called {
            // TODO return???
            return Ok(());
        }
        hnd.called = true;
        let trigger_height = h - hnd.confidence;

        let inc_tipset = ts_cache.get_non_null(trigger_height)?;
        let r = (hnd.handle)(&inc_tipset, h);
        if let Err(e) = r {
            error!(
                "chain trigger (@H {}, called @ {}) failed: {:?}",
                trigger_height,
                ts.height(),
                e
            );
        }
    }
    Ok(())
}

/// Events function impl for events_height
impl Events {
    pub fn chain_at(
        &mut self,
        hnd: HeightHandler,
        rev: RevertHandler,
        confidence: u64,
        h: u64,
    ) -> Result<()> {
        let best_height = self.ts_cache.best().height();
        if best_height >= (h + confidence) {
            let ts = self.ts_cache.get_non_null(h)?;
            (hnd)(&ts, best_height)?;
            // TODO split lock
        }
        if best_height >= h + confidence + self.gc_confidence {
            return Ok(());
        }

        let trigger_at = h + confidence;
        let id = self.ctr;
        self.ctr += 1;

        self.events_height.height_triggers.insert(
            id,
            HeightHandlerObj {
                confidence,
                called: false,
                handle: hnd,
                revert: rev,
            },
        );
        // msg height
        self.events_height
            .ht_heights
            .entry(h)
            .or_insert(Default::default())
            .push(id);
        // trigger height
        self.events_height
            .ht_trigger_heights
            .entry(trigger_at)
            .or_insert(Default::default())
            .push(id);

        Ok(())
    }

    fn head_change_at(&mut self, reverts: &[Tipset], applies: &[Tipset]) -> Result<()> {
        for ts in reverts {
            revert_func(ts.height(), &mut self.events_height, ts)?;

            let mut sub_height = ts.height() - 1;
            loop {
                let cts = self.ts_cache.get(sub_height)?;
                if cts.is_some() {
                    break;
                }
                revert_func(sub_height, &mut self.events_height, ts)?;
                sub_height -= 1;
            }
            self.ts_cache.revert(ts)?;
        }

        for ts in applies {
            self.ts_cache.add(ts.clone())?;
            // height triggers
            apply_func(ts.height(), &mut self.events_height, &self.ts_cache, ts)?;

            let mut sub_height = ts.height() - 1;
            loop {
                let cts = self.ts_cache.get(sub_height)?;
                if cts.is_some() {
                    break;
                }
                apply_func(sub_height, &mut self.events_height, &self.ts_cache, ts)?;
                sub_height -= 1;
            }
        }
        Ok(())
    }
}

/// Events function impl for events_called
impl Events {
    fn head_change_called(&mut self, _reverts: &[Tipset], _applies: &[Tipset]) -> Result<()> {
        // TODO
        Ok(())
    }
}

async fn listen_head_changes<Api: ChainApi>(api: Arc<Api>, event: Arc<RwLock<Events>>) {
    loop {
        let r = listen_head_changes_once(api.clone(), event.clone()).await;
        if let Err(msg) = r {
            error!(target: TARGET, "listen head changes errored: {}", msg);
        }
        sleep(Duration::from_secs(1)).await;
        info!(target: TARGET, "restarting listen_head_changes");
    }
}

async fn listen_head_changes_once<Api: ChainApi>(
    api: Arc<Api>,
    event: Arc<RwLock<Events>>,
) -> Result<()> {
    let (_subscription_id, mut notify) = api
        .chain_notify()
        .await
        .map_err(|e| EventsError::Other(Box::new(e)))?;
    let mut current: Vec<HeadChange> = notify.next().await.unwrap();
    if current.len() != 1 {
        return Err(EventsError::UnexpectedInitial(current.len()));
    }
    let c = current.remove(0);
    match c.r#type {
        HeadChangeType::Current => {}
        _ => Err(EventsError::UnexpectedInitialType(c.r#type))?,
    }
    let r = {
        let mut e = event.write().unwrap();
        e.ts_cache.add(c.val)
    };
    if let Err(e) = r {
        warn!(
            target: TARGET,
            "tsc.add: adding current tipset failed: {:?}", e
        );
    }

    #[allow(irrefutable_let_patterns)]
    while let Some(head_change) = notify.next().await {
        let mut reverts = vec![];
        let mut applies = vec![];
        for notif in head_change {
            match notif.r#type {
                HeadChangeType::Revert => reverts.push(notif.val),
                HeadChangeType::Apply => applies.push(notif.val),
                _ => warn!(
                    target: TARGET,
                    "unexpected head change notification type: '{:?}'", notif.r#type
                ),
            }
        }
        {
            let mut e = event.write().unwrap();
            if let Err(err) = e.head_change(reverts, applies) {
                warn!(target: TARGET, "headChange failed: {:?}", err);
            }
        }
    }

    Ok(())
}
