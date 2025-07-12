#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use core::mem::swap;
use core::slice::from_raw_parts_mut;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::clocks::ClockConfig;
use embassy_rp::config::Config;
use embassy_rp::{
    bind_interrupts,
    peripherals::PIO0,
    pio::{InterruptHandler, Pio},
};
use embassy_time::Timer;
use picoscope_rs::pio_converter::PioConverter;
use portable_atomic::{AtomicBool, Ordering};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const BUFFER_SIZE: usize = 256;

struct DBuffer {
    active: *mut u8,
    background: *mut u8,
}

static mut BUFFER1: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut BUFFER2: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static IS_READING_DONE: AtomicBool = AtomicBool::new(true);
static IS_SWAPPED: AtomicBool = AtomicBool::new(false);
static mut BUFFER: DBuffer = DBuffer {
    active: unsafe { BUFFER1.as_mut_ptr() },
    background: unsafe { BUFFER2.as_mut_ptr() },
};

unsafe impl Sync for DBuffer {}

impl DBuffer {
    async fn swap(&mut self) {
        while !IS_READING_DONE.load(Ordering::Relaxed) {
            Timer::after_ticks(1).await;
        }
        IS_READING_DONE.store(false, Ordering::Relaxed);
        swap(&mut self.active, &mut self.background);
        IS_SWAPPED.store(true, Ordering::Relaxed);
    }

    fn get_active(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.active, BUFFER_SIZE) }
    }

    fn get_background(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.background, BUFFER_SIZE) }
    }

    fn reading_done(&mut self) {
        IS_READING_DONE.store(true, Ordering::Relaxed);
    }

    async fn wait_for_swap(&self) {
        while !IS_SWAPPED.load(Ordering::Relaxed) {
            Timer::after_ticks(1).await;
        }
        IS_SWAPPED.store(false, Ordering::Relaxed);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    info!("Started");
    let config = ClockConfig::system_freq(200_000_000).unwrap();
    let cfg = Config::new(config);
    let p = embassy_rp::init(cfg);

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let mut pio_converter = PioConverter::new(
        &mut common,
        sm0,
        p.DMA_CH0,
        p.PIN_2, // out 0
        p.PIN_3, // out 1
        p.PIN_4, // out 2
        p.PIN_5, // out 3
        p.PIN_6, // out 4
        p.PIN_7, // out 5
        p.PIN_8, // out 6
        p.PIN_9, // out 7
    );

    info!("Begin");

    unwrap!(spawner.spawn(tmp()));

    let mut counter = 0;
    loop {
        let buffer = unsafe { BUFFER.get_active() };
        // pio_converter.work(buffer).await;
        buffer[0] = counter;
        unsafe { BUFFER.swap().await };
        Timer::after_millis(1).await;
        counter += 1;
    }
}

#[embassy_executor::task]
async fn tmp() {
    loop {
        unsafe { BUFFER.wait_for_swap().await };
        let data = unsafe { BUFFER.get_background() };
        info!("zbs {}", data[0]);
        unsafe { BUFFER.reading_done() };
    }
}
