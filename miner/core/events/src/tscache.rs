use std::collections::BTreeMap;

use plum_tipset::{Tipset, TipsetKey};

use crate::error::{EventsError, Result};

use crate::TARGET;
use log::warn;

pub struct TipSetCache {
    cache: BTreeMap<usize, Tipset>,
    start: usize,
    len: usize,
    capacity: usize,
    storage: Box<dyn Fn(u64, &TipsetKey) -> Result<Tipset>>,
}

impl TipSetCache {
    pub fn new(cap: usize, storage: Box<dyn Fn(u64, &TipsetKey) -> Result<Tipset>>) -> Self {
        TipSetCache {
            cache: BTreeMap::new(),
            start: 0,
            len: 0,
            capacity: cap,
            storage,
        }
    }

    pub fn add(&mut self, tipset: Tipset) -> Result<()> {
        let mut next_height = tipset.height();
        if self.len > 0 {
            let best = self.best();
            let best_height = best.height();
            if best_height >= next_height {
                return Err(EventsError::HigherThenBest(best_height + 1, next_height));
            }
            next_height = best_height + 1;
        }
        // full null block
        while next_height != tipset.height() {
            self.start = normal_modulo(self.start + 1, self.capacity);
            self.cache.remove(&self.start);
            if self.len < self.capacity {
                self.len += 1;
            }
            next_height += 1;
        }

        self.start = normal_modulo(self.start + 1, self.capacity);
        self.cache.insert(self.start, tipset);
        if self.len < self.capacity {
            self.len += 1;
        }

        Ok(())
    }

    pub fn revert(&mut self, tipset: &Tipset) -> Result<()> {
        if self.len == 0 {
            return Ok(());
        }
        if self.best() != tipset {
            return Err(EventsError::RevertError(
                self.best().clone(),
                tipset.clone(),
            ));
        }
        self.cache.remove(&self.start);
        self.start = normal_modulo(self.start - 1, self.capacity);
        self.len -= 1;
        Ok(())
    }

    pub fn get(&self, height: u64) -> Result<Option<Tipset>> {
        if self.len == 0 {
            warn!(
                target: TARGET,
                "tipSetCache.get: cache is empty, requesting from storage (h={})", height
            );
            return (self.storage)(height, &TipsetKey::default()).map(Some);
        }
        let best_height = self.best().height();
        if height > best_height {
            return Err(EventsError::NotInCache(best_height, height));
        }
        let get_tail = || {
            for i in 1..(self.len + 1) {
                if let Some(t) = self
                    .cache
                    .get(&normal_modulo(self.start - self.len + i, self.capacity))
                {
                    return Some(t);
                }
            }
            None
        };
        let tail = get_tail().ok_or(EventsError::GetTailFailed)?;
        if height < tail.height() {
            warn!(target: TARGET, "tipSetCache.get: requested tipset not in cache, requesting from storage (h={}; tail={})", height, tail.height());
            return (self.storage)(height, tail.key()).map(Some);
        }
        let tipset = self
            .cache
            .get(&normal_modulo(
                self.start - (best_height - height) as usize,
                self.capacity,
            ))
            .map(Clone::clone);
        Ok(tipset)
    }

    pub fn get_non_null(&self, height: u64) -> Result<Tipset> {
        let mut height = height;
        let mut final_check = 0;
        loop {
            if final_check > self.capacity {
                return Err(EventsError::OverflowCacheCapacity(
                    final_check,
                    self.capacity,
                ));
            }
            let tipset = self.get(height)?;
            if let Some(t) = tipset {
                return Ok(t);
            }
            height += 1;
            final_check += 1;
        }
    }

    pub fn best(&self) -> &Tipset {
        self.cache.get(&self.start).expect("best must exist")
    }
}

fn normal_modulo(n: usize, m: usize) -> usize {
    ((n % m) + m) % m
}
