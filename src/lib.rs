/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod tests;

use core::marker::PhantomData;
use enum_iterator::{cardinality, Sequence};
use std::sync::atomic::{self, AtomicU64};
use std::time::Duration;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Status {
    pub count: u64,
    pub time: Duration,
}
#[cfg(test)]
impl Default for Status {
    fn default() -> Self {
        Status {
            count: 0,
            time: Duration::default(),
        }
    }
}

struct Timer {
    count: AtomicU64,
    nsec: AtomicU64,
}
impl Default for Timer {
    fn default() -> Timer {
        Timer {
            count: AtomicU64::new(0),
            nsec: AtomicU64::new(0),
        }
    }
}
impl Timer {
    fn add(&self, time: Duration) {
        self.count.fetch_add(1, atomic::Ordering::Relaxed);
        self.nsec.fetch_add(
            time.as_nanos().try_into().unwrap(),
            atomic::Ordering::Relaxed,
        );
    }
    fn status(&self) -> Status {
        let nsec = self.nsec.load(atomic::Ordering::Relaxed);
        Status {
            count: self.count.load(atomic::Ordering::Relaxed),
            time: Duration::from_nanos(nsec),
        }
    }
}

struct Timers {
    timers: Vec<Timer>,
}
impl Timers {
    fn new(num_types: usize) -> Timers {
        let mut timers = Vec::with_capacity(num_types);
        timers.resize_with(num_types, Timer::default);
        Timers { timers }
    }
    fn add(&self, t: usize, time: Duration) {
        self.timers[t].add(time);
    }
    fn status(&self) -> Vec<Status> {
        let mut status = Vec::with_capacity(self.timers.len());
        for timer in self.timers.as_slice() {
            status.push(timer.status());
        }
        status
    }
}

pub struct TypedTimers<Type: Sequence + Into<usize>> {
    timers: Timers,
    _type: PhantomData<Type>,
}
impl<Type: Sequence + Into<usize>> TypedTimers<Type> {
    pub fn new() -> Self {
        Self {
            timers: Timers::new(cardinality::<Type>()),
            _type: PhantomData::default(),
        }
    }
    pub fn add(&self, t: Type, time: Duration) {
        self.timers.add(t.into(), time);
    }
    pub fn status(&self) -> Vec<Status> {
        self.timers.status()
    }
}
