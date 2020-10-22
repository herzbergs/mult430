/// Keep track of what the followup timer is doing
/// It can be not running, waiting to fall, or waiting to rise

/// Started using article "A Fistful of States" on hoverbear.org then realized
/// I didn't really know how to use the state machine implemented that way
/// so I made something a little more familar and less "rusty"

use crate::pulses::SyncSignal;

pub struct TimerState<RunFn, StopFn> where 
    RunFn: Fn(u16) -> (),
    StopFn: Fn() -> (),
{
    set_and_run_timer: RunFn,
    stop_timer: StopFn,
    state: State,
}

#[derive(PartialEq)]
pub enum State {
    NotRunning,
    Falling,
    Rising,
}

impl<RunFn, StopFn> TimerState<RunFn, StopFn> where
    RunFn: Fn(u16) -> (),
    StopFn: Fn() -> (),
{
    pub fn new(set_and_run_fn: RunFn, stop_fn: StopFn,) -> Self {
        // shouldn't be running anyway, but let's ensure that here
        (stop_fn)();

        TimerState {
            set_and_run_timer: set_and_run_fn,
            stop_timer: stop_fn,
            state: State::NotRunning,
        }
    }

    pub fn next_state(&mut self, s: &SyncSignal) {
        match self.state {
            // "falling" means we are going low next, so load high time
            State::Falling => (self.set_and_run_timer)(s.high_time),
            State::Rising => (self.set_and_run_timer)(s.low_time),
            _ => (self.stop_timer)(),
        };
            
        let next_state = match self.state {
            State::NotRunning => State::Falling,
            State::Falling => if s.multiplicand != 1 {
                State::Rising
            } else {
                State::NotRunning
            },
            State::Rising => State::Falling,
        };

        self.state = next_state;
    }
}
