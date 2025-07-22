#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![allow(static_mut_refs)]

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
use picoscope_rs::double_buffer::DBUFFER;
use picoscope_rs::pio_pins_listener::PioPinsListener;
use picoscope_rs::ring_buffer::RingBuffer;
use picoscope_rs::states::State;
use picoscope_rs::trigger::{Trigger, TriggerKind};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    let config = ClockConfig::system_freq(250_000_000).unwrap();
    let cfg = Config::new(config);
    let p = embassy_rp::init(cfg);

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let pins_listener = PioPinsListener::new(
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
async fn core1_task(mut pins_listener: PioPinsListener<'static, PIO0, 0>) -> ! {
    info!("Started");

    loop {
        let buffer = unsafe { DBUFFER.get_active() };
        pins_listener.work(buffer).await;
        unsafe { DBUFFER.swap().await };
    }
}

#[embassy_executor::task]
async fn core0_task() {
    let mut state = State::idle();
    let mut rbuffer = RingBuffer::new();

    let trigger = Trigger {
        mask: 0b0000_0001,
        kind: TriggerKind::Falling,
    };

    loop {
        unsafe { DBUFFER.wait_for_swap().await };
        let chunk = unsafe { DBUFFER.get_background() };

        state = match state {
            State::Idle(ref mut st) => st.tick(&mut rbuffer, chunk, &trigger).await,
            State::Record(ref mut st) => st.tick(&mut rbuffer, chunk),
            State::Analysys(st) => st.tick(),
        };

        unsafe { DBUFFER.reading_done() };
    }
}
