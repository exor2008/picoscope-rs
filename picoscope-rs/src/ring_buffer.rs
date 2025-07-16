use crate::BUFFER_SIZE;

pub const RBUFFER_SIZE: usize = 10;

pub struct RingBuffer {
    buffer: [[u8; BUFFER_SIZE]; RBUFFER_SIZE],
    current: usize,
}

impl Default for RingBuffer {
    fn default() -> Self {
        RingBuffer {
            buffer: [[0; BUFFER_SIZE]; RBUFFER_SIZE],
            current: 0,
        }
    }
}

impl RingBuffer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write(&mut self, chunk: &[u8]) {
        self.buffer[self.current] = chunk.try_into().unwrap();
        self.advance();
    }

    fn advance(&mut self) {
        self.current = match self.current {
            current if current == RBUFFER_SIZE - 1 => 0,
            current => current + 1,
        }
    }
}
