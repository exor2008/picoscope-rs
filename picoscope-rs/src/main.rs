#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![allow(static_mut_refs)]

use core::mem::swap;
use core::slice::from_raw_parts_mut;
use defmt::*;
use embassy_executor::Executor;
use embassy_rp::clocks::ClockConfig;
use embassy_rp::config::Config;
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::{
    bind_interrupts,
    peripherals::PIO0,
    pio::{InterruptHandler, Pio},
};
use embassy_time::Timer;
use picoscope_rs::pio_pins_listener::PioPinsListener;
use picoscope_rs::states::State;
use portable_atomic::{AtomicBool, Ordering};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const BUFFER_SIZE: usize = 256;

struct DBuffer {
    active: *mut u8,
    background: *mut u8,
}
static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

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
            // Timer::after_ticks(1).await;
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

#[cortex_m_rt::entry]
fn main() -> ! {
    let config = ClockConfig::system_freq(200_000_000).unwrap();
    let cfg = Config::new(config);
    let p = embassy_rp::init(cfg);

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let mut pins_listener = PioPinsListener::new(
        &mut common,
        sm0,
        p.DMA_CH0,
        p.PIN_2, // in 0
        p.PIN_3, // in 1
        p.PIN_4, // in 2
        p.PIN_5, // in 3
        p.PIN_6, // in 4
        p.PIN_7, // in 5
        p.PIN_8, // in 6
        p.PIN_9, // in 7
    );

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(pins_listener))));
        },
    );

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| unwrap!(spawner.spawn(core0_task())));
}

#[embassy_executor::task]
async fn core1_task(pins_listener: PioPinsListener<'static, PIO0, 0>) -> ! {
    info!("Started");

    let mut counter = 0;
    loop {
        let buffer = unsafe { BUFFER.get_active() };
        // pins_listener.work(buffer).await;
        buffer[0] = counter;
        unsafe { BUFFER.swap().await };
        counter = counter.wrapping_add(1);
    }
}

#[embassy_executor::task]
async fn core0_task() {
    loop {
        // Timer::after_millis(1).await;
        unsafe { BUFFER.wait_for_swap().await };
        let data = unsafe { BUFFER.get_background() };
        info!("zbs {}", data[0]);
        unsafe { BUFFER.reading_done() };

        let mut state = State::init();
        state = State::init();
    }
}
