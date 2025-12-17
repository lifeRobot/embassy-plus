use embassy_hal_internal::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::RTC;
use embassy_rp::rtc::Rtc;

// Bind the RTC interrupt to the handler
bind_interrupts!(struct Irqs {
    RTC_IRQ => embassy_rp::rtc::InterruptHandler;
});

/// rtc trait
pub trait RtcTrait<'d> {
    /// build rtc driver
    fn build(self) -> Rtc<'d, RTC>;
}

/// support rtc trait
impl<'d> RtcTrait<'d> for Peri<'d, RTC> {
    #[inline]
    fn build(self) -> Rtc<'d, RTC> {
        Rtc::new(self, Irqs)
    }
}
