use embassy_hal_internal::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{PIO0, PIO1};
use embassy_rp::pio::{Instance, InterruptHandler, Pio};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    PIO1_IRQ_0 => InterruptHandler<PIO1>;
});

/// pio trait
pub trait PioTrait<'d, PIO: Instance> {
    /// create pio driver<br />
    /// more see [Pio::new]
    fn build(self) -> Pio<'d, PIO>;
}

/// pio0 support pio trait
impl<'d> PioTrait<'d, PIO0> for Peri<'d, PIO0> {
    #[inline]
    fn build(self) -> Pio<'d, PIO0> {
        Pio::new(self, Irqs)
    }
}

/// pio1 support pio trait
impl<'d> PioTrait<'d, PIO1> for Peri<'d, PIO1> {
    #[inline]
    fn build(self) -> Pio<'d, PIO1> {
        Pio::new(self, Irqs)
    }
}
