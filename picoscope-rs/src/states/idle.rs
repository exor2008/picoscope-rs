use super::State;
use crate::ring_buffer::RingBuffer;
pub struct Idle;

impl Idle {
    pub fn tick(&self, rbuffer: &mut RingBuffer, chunk: &[u8]) -> State {
        rbuffer.write(chunk);
        State::idle()
    }
}
