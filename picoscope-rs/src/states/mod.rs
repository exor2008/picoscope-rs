use analysys::Analysys;
use idle::Idle;
use record::Record;

use crate::ring_buffer::RingBuffer;

pub mod analysys;
pub mod idle;
pub mod record;

pub enum State {
    Idle(Idle),
    Analysys(Analysys),
    Record(Record),
}

impl Default for State {
    fn default() -> Self {
        State::idle()
    }
}

impl State {
    pub fn idle() -> Self {
        State::Idle(Idle)
    }

    pub fn record() -> Self {
        State::Record(Record::default())
    }

    pub fn analysys() -> Self {
        State::Analysys(Analysys)
    }

    fn tick(&mut self, rbuffer: &mut RingBuffer, chunk: &[u8]) -> Self {
        match self {
            State::Idle(state) => state.tick(rbuffer, chunk),
            State::Record(state) => state.tick(rbuffer, chunk),
            State::Analysys(state) => state.tick(),
        }
    }
}

#[derive(Default)]
pub struct ScopeStateMachine {
    state: State,
}

impl ScopeStateMachine {
    pub fn new() -> Self {
        ScopeStateMachine {
            state: State::idle(),
        }
    }

    pub fn tick(&mut self, rbuffer: &mut RingBuffer, chunk: &[u8]) {
        self.state = self.state.tick(rbuffer, chunk)
    }
}
