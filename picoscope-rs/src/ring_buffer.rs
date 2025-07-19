use crate::BUFFER_SIZE;

pub const RBUFFER_SIZE: usize = 10;

pub struct RingBuffer {
    buffer: [[u8; BUFFER_SIZE]; RBUFFER_SIZE],
    current: usize,
    recorded_since: Option<usize>,
}

impl Default for RingBuffer {
    fn default() -> Self {
        RingBuffer {
            buffer: [[0; BUFFER_SIZE]; RBUFFER_SIZE],
            current: Default::default(),
            recorded_since: Default::default(),
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

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn recorded_since(&self) -> Option<usize> {
        self.recorded_since
    }

    pub fn record(&mut self) {
        self.recorded_since = Some(self.current);
    }
}
