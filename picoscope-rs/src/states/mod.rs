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

impl State {
    pub fn init() -> Self {
        State::Idle(Idle)
    }
}
