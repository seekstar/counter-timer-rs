/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod tests;

use core::marker::PhantomData;
use enum_iterator::{cardinality, Sequence};
use std::sync::atomic::{self, AtomicUsize};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Status {
    pub count: usize,
    pub nsec: usize,
}
#[cfg(test)]
impl Default for Status {
    fn default() -> Self {
        Status { count: 0, nsec: 0 }
    }
}

struct Timer {
    count: AtomicUsize,
    nsec: AtomicUsize,
}
impl Default for Timer {
    fn default() -> Timer {
        Timer {
            count: AtomicUsize::new(0),
            nsec: AtomicUsize::new(0),
        }
    }
}
impl Timer {
    fn add(&self, nsec: usize) {
        self.count.fetch_add(1, atomic::Ordering::Relaxed);
        self.nsec.fetch_add(nsec, atomic::Ordering::Relaxed);
    }
    fn status(&self) -> Status {
        Status {
            count: self.count.load(atomic::Ordering::Relaxed),
            nsec: self.nsec.load(atomic::Ordering::Relaxed),
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
    fn add(&self, t: usize, nsec: usize) {
        self.timers[t].add(nsec);
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
    pub fn add(&self, t: Type, nsec: usize) {
        self.timers.add(t.into(), nsec);
    }
    pub fn status(&self) -> Vec<Status> {
        self.timers.status()
    }
}
