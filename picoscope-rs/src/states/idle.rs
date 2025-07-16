use super::State;
pub struct Idle;

impl Idle {
    pub fn tick(&self) -> State {
        State::idle()
    }
}
