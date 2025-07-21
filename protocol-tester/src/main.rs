#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Level, Output},
    spi::{Config, Spi},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Started");
    let p = embassy_rp::init(Default::default());

    let miso = p.PIN_12;
    let mosi = p.PIN_11;
    let clk = p.PIN_10;
    let mut cs = Output::new(p.PIN_13, Level::High);

    let mut cfg = Config::default();
    cfg.frequency = 10_000_000;

    let mut spi = Spi::new(p.SPI1, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, cfg);

    loop {
        cs.set_low();
        for _ in 0..1000 {
            spi.write(&[0b10000100; 1024]).await.unwrap();
        }
        cs.set_high();
        Timer::after_secs(10).await;
    }
}
