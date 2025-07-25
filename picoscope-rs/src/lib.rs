#![no_std]
#![no_main]

pub mod double_buffer;
pub mod pio_pins_listener;
pub mod ring_buffer;
pub mod states;
pub mod trigger;
pub mod epd2in7v2;

pub const BUFFER_SIZE: usize = 1024;
