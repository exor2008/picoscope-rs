use super::State;
use crate::ring_buffer::{RingBuffer, RBUFFER_SIZE};
use defmt::debug;

#[derive(Default, Clone, Copy)]
pub struct Record {
    counter: usize,
}

impl Record {
    pub fn tick(&mut self, rbuffer: &mut RingBuffer, chunk: &[u8]) -> State {
        debug!("Record tick, counter: {}", self.counter);

        rbuffer.write(chunk);

        self.counter += 1;

        if self.counter == RBUFFER_SIZE - 1 {
            State::analysys()
        } else {
            State::Record(*self)
        }
    }
}
