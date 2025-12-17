use embassy_hal_internal::Peri;
use embassy_rp::peripherals::WATCHDOG;
use embassy_rp::watchdog::Watchdog;

/// watchdog trait
pub trait WatchDogTrait {
    /// build watchdog driver
    fn build(self) -> Watchdog;
}

/// support watchdog trait
impl WatchDogTrait for Peri<'static, WATCHDOG> {
    #[inline]
    fn build(self) -> Watchdog {
        Watchdog::new(self)
    }
}
