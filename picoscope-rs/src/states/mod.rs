use analysys::Analysys;
use idle::Idle;
use record::Record;

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

    pub fn analysys() -> State {
        State::Analysys(Analysys)
    }
}
