use super::State;
use crate::ring_buffer::{RingBuffer, RBUFFER_SIZE};
use defmt::info;
use embassy_time::Timer;
pub struct Analysys;

impl Analysys {
    pub async fn tick(&self, rbuffer: &mut RingBuffer) -> State {
        rbuffer.advance();
        for _ in 0..RBUFFER_SIZE {
            let chunk = rbuffer.read();
            info!("{:03b}", chunk);
            Timer::after_micros(100).await;
        }
        Timer::after_secs(1000).await;
        State::analysys()
    }
}
