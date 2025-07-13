use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::gpio::Pull;
use embassy_rp::pio::{
    Common, Config, Direction, FifoJoin, Instance, PioPin, ShiftConfig, ShiftDirection,
    StateMachine,
};
use embassy_rp::Peri;
use fixed::types::U24F8;

pub struct PioPinsListener<'a, P: Instance, const N: usize> {
    dma: Peri<'a, AnyChannel>,
    sm: StateMachine<'a, P, N>,
}

#[allow(clippy::too_many_arguments)]
impl<'a, P: Instance, const N: usize> PioPinsListener<'a, P, N> {
    pub fn new(
        pio: &mut Common<'a, P>,
        mut sm: StateMachine<'a, P, N>,
        dma: Peri<'a, impl Channel>,

        db0: Peri<'a, impl PioPin>,
        db1: Peri<'a, impl PioPin>,
        db2: Peri<'a, impl PioPin>,
        db3: Peri<'a, impl PioPin>,
        db4: Peri<'a, impl PioPin>,
        db5: Peri<'a, impl PioPin>,
        db6: Peri<'a, impl PioPin>,
        db7: Peri<'a, impl PioPin>,
    ) -> PioPinsListener<'a, P, N> {
        let mut db0 = pio.make_pio_pin(db0);
        let mut db1 = pio.make_pio_pin(db1);
        let mut db2 = pio.make_pio_pin(db2);
        let mut db3 = pio.make_pio_pin(db3);
        let mut db4 = pio.make_pio_pin(db4);
        let mut db5 = pio.make_pio_pin(db5);
        let mut db6 = pio.make_pio_pin(db6);
        let mut db7 = pio.make_pio_pin(db7);

        db0.set_pull(Pull::Down);
        db1.set_pull(Pull::Down);
        db2.set_pull(Pull::Down);
        db3.set_pull(Pull::Down);
        db4.set_pull(Pull::Down);
        db5.set_pull(Pull::Down);
        db6.set_pull(Pull::Down);
        db7.set_pull(Pull::Down);

        let prg_command = pio::pio_asm!(
            r#"
            .wrap_target
                in pins 8
            .wrap
            "#,
        );

        sm.set_pin_dirs(
            Direction::In,
            &[&db0, &db1, &db2, &db3, &db4, &db5, &db6, &db7],
        );

        let mut cfg = Config::default();
        cfg.use_program(&pio.load_program(&prg_command.program), &[]);
        cfg.set_in_pins(&[&db0, &db1, &db2, &db3, &db4, &db5, &db6, &db7]);

        cfg.shift_in = ShiftConfig {
            auto_fill: true,
            direction: ShiftDirection::Left,
            threshold: 32,
        };

        cfg.fifo_join = FifoJoin::RxOnly;
        cfg.clock_divider = U24F8::from_num(1.0);

        sm.set_config(&cfg);
        sm.set_enable(true);

        PioPinsListener {
            dma: dma.into(),
            sm,
        }
    }
}

impl<'a, P: Instance, const N: usize> PioPinsListener<'a, P, N> {
    pub async fn work(&mut self, input: &mut [u8]) {
        self.sm
            .rx()
            .dma_pull(self.dma.reborrow(), input, false)
            .await;
    }
}
