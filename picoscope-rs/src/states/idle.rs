use super::State;
use crate::ring_buffer::RingBuffer;
use defmt::debug;

static mut CNT: usize = 0;

pub struct Idle;

impl Idle {
    pub fn tick(&mut self, rbuffer: &mut RingBuffer, chunk: &[u8]) -> State {
        debug!("Idle state tick {}", unsafe { CNT });

        rbuffer.write(chunk);

        unsafe { CNT += 1 };
        if unsafe { CNT } <= 20 {
            State::idle()
        } else {
            State::record()
        }
    }
}
