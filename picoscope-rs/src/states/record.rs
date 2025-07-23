use super::State;
use crate::ring_buffer::{RingBuffer, RBUFFER_SIZE};
use defmt::info;

#[derive(Default, Clone, Copy)]
pub struct Record {
    counter: usize,
}

impl Record {
    pub fn tick(&mut self, rbuffer: &mut RingBuffer, chunk: &[u8]) -> State {
        rbuffer.write(chunk);

        self.counter += 1;

        if self.counter == RBUFFER_SIZE - 2 {
            info!("Analysys state {}", self.counter);
            State::analysys()
        } else {
            State::Record(*self)
        }
    }
}
