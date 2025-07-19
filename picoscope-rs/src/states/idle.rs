use super::State;
use crate::{
    ring_buffer::RingBuffer,
    trigger::{Trigger, TriggerKind},
};
use defmt::debug;

pub struct Idle;

impl Idle {
    pub fn tick(&mut self, rbuffer: &mut RingBuffer, chunk: &[u8], trigger: &Trigger) -> State {
        debug!("Idle state tick");

        rbuffer.write(chunk);

        if self.scan_for_triggers(chunk, trigger) {
            rbuffer.record();
            State::record()
        } else {
            State::idle()
        }
    }

    fn scan_for_triggers(&self, chunk: &[u8], trigger: &Trigger) -> bool {
        match trigger.kind {
            TriggerKind::Rising => chunk.windows(2).any(|c| {
                let a = c[0] & trigger.mask;
                let b = c[1] & trigger.mask;
                a | b > a
            }),

            TriggerKind::Falling => chunk.windows(2).any(|c| {
                let a = c[0] & trigger.mask;
                let b = c[1] & trigger.mask;
                a & b < a
            }),
        }
    }
}
