use crate::BUFFER_SIZE;
use heapless::Vec;

pub const RBUFFER_SIZE: usize = 10;

pub struct RingBuffer {
    buffer: Vec<[u8; BUFFER_SIZE], RBUFFER_SIZE>,
}
