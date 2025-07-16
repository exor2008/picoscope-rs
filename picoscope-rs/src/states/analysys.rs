use super::State;
pub struct Analysys;

impl Analysys {
    pub fn tick(&self) -> State {
        State::analysys()
    }
}
