#![allow(static_mut_refs)]

use core::slice::from_raw_parts_mut;
use core::{mem::swap, sync::atomic::Ordering};

use embassy_time::Timer;
use portable_atomic::AtomicBool;

use crate::BUFFER_SIZE;

pub struct DBuffer {
    active: *mut u8,
    background: *mut u8,
}

static mut BUFFER1: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut BUFFER2: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
pub static IS_READING_DONE: AtomicBool = AtomicBool::new(true);
pub static IS_SWAPPED: AtomicBool = AtomicBool::new(false);
pub static mut DBUFFER: DBuffer = DBuffer {
    active: unsafe { BUFFER1.as_mut_ptr() },
    background: unsafe { BUFFER2.as_mut_ptr() },
};

unsafe impl Sync for DBuffer {}

impl DBuffer {
    pub async fn swap(&mut self) {
        while !IS_READING_DONE.load(Ordering::Relaxed) {
            // Timer::after_ticks(1).await;
        }
        IS_READING_DONE.store(false, Ordering::Relaxed);
        swap(&mut self.active, &mut self.background);
        IS_SWAPPED.store(true, Ordering::Relaxed);
    }

    pub fn get_active(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.active, BUFFER_SIZE) }
    }

    pub fn get_background(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.background, BUFFER_SIZE) }
    }

    pub fn reading_done(&mut self) {
        IS_READING_DONE.store(true, Ordering::Relaxed);
    }

    pub async fn wait_for_swap(&self) {
        while !IS_SWAPPED.load(Ordering::Relaxed) {
            Timer::after_ticks(1).await;
        }
        IS_SWAPPED.store(false, Ordering::Relaxed);
    }
}
