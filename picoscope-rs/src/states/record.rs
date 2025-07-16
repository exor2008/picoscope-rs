use super::State;
pub struct Record;

impl Record {
    pub fn tick(&self) -> State {
        State::record()
    }
}
