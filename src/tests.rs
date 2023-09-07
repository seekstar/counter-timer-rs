/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::TypedTimers;
    use enum_iterator::Sequence;
    use num_derive::ToPrimitive;
    use num_traits::ToPrimitive;

    #[test]
    fn no_timer() {
        #[derive(Sequence, ToPrimitive)]
        enum TimerType {}
        impl Into<usize> for TimerType {
            fn into(self) -> usize {
                self.to_usize().unwrap()
            }
        }
        let timers = TypedTimers::<TimerType>::new();
        assert!(timers.status().len() == 0);
    }
    #[test]
    fn one_timer() {
        #[derive(Sequence, ToPrimitive)]
        enum TimerType {
            Timer1,
        }
        impl Into<usize> for TimerType {
            fn into(self) -> usize {
                self.to_usize().unwrap()
            }
        }
        let timers = TypedTimers::<TimerType>::new();
        assert_eq!(timers.status(), vec![crate::Status::default()]);
        timers
            .timer(TimerType::Timer1)
            .add(Duration::from_nanos(233));
        assert_eq!(
            timers.status(),
            vec![crate::Status {
                count: 1,
                time: Duration::from_nanos(233),
            }]
        );
        timers
            .timer(TimerType::Timer1)
            .add(Duration::from_nanos(2333 - 233));
        assert_eq!(
            timers.status(),
            vec![crate::Status {
                count: 2,
                time: Duration::from_nanos(2333),
            }]
        );
    }
}
