use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Blocking, Config, MisoPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi0::clk::Spi0Clk;
use crate::builder::spi::spi0::miso::Spi0Miso;

/// spi0 rx builder
pub struct Spi0RxBuilder<'d> {
    /// spi0 base data
    pub base: SpiBase<'d, SPI0>,
    /// clk pin
    pub clk: Spi0Clk<'d>,
    /// miso pin
    pub miso: Spi0Miso<'d>,
}

/// custom method
impl<'d> Spi0RxBuilder<'d> {
    /// crate builder
    #[inline]
    pub fn new(spi: Peri<'d, SPI0>, clk: Spi0Clk<'d>, miso: Spi0Miso<'d>) -> Self {
        Self { base: SpiBase::new(spi), clk, miso }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build spi rx by async<br />
    /// more see [Spi::<_, Async>::new_rxonly]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>, rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI0, Async> {
        match self.miso {
            Spi0Miso::Pin0(pin_0) => Self::build_clk(self.clk, pin_0, self.base, tx_dma, rx_dma),
            Spi0Miso::Pin4(pin_4) => Self::build_clk(self.clk, pin_4, self.base, tx_dma, rx_dma),
            Spi0Miso::Pin16(pin_16) => Self::build_clk(self.clk, pin_16, self.base, tx_dma, rx_dma),
            Spi0Miso::Pin20(pin_20) => Self::build_clk(self.clk, pin_20, self.base, tx_dma, rx_dma),
        }
    }

    /// build by clk
    fn build_clk(
        clk: Spi0Clk<'d>,
        miso: Peri<'d, impl MisoPin<SPI0> + 'd>,
        base: SpiBase<'d, SPI0>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI0, Async> {
        match clk {
            Spi0Clk::Pin2(pin_2) => Spi::new_rxonly(base.spi, pin_2, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin6(pin_6) => Spi::new_rxonly(base.spi, pin_6, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin18(pin_18) => Spi::new_rxonly(base.spi, pin_18, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin22(pin_22) => Spi::new_rxonly(base.spi, pin_22, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build spi rx by async<br />
    /// more see [Spi::<_, Blocking>::new_blocking_rxonly]
    pub fn build_blocking(self) -> Spi<'d, SPI0, Blocking> {
        match self.miso {
            Spi0Miso::Pin0(pin_0) => Self::build_clk_blocking(self.clk, pin_0, self.base),
            Spi0Miso::Pin4(pin_4) => Self::build_clk_blocking(self.clk, pin_4, self.base),
            Spi0Miso::Pin16(pin_16) => Self::build_clk_blocking(self.clk, pin_16, self.base),
            Spi0Miso::Pin20(pin_20) => Self::build_clk_blocking(self.clk, pin_20, self.base),
        }
    }

    /// build blocking by clk
    fn build_clk_blocking(
        clk: Spi0Clk<'d>,
        miso: Peri<'d, impl MisoPin<SPI0> + 'd>,
        base: SpiBase<'d, SPI0>) -> Spi<'d, SPI0, Blocking> {
        match clk {
            Spi0Clk::Pin2(pin_2) => Spi::new_blocking_rxonly(base.spi, pin_2, miso, base.config.unwrap_or_default()),
            Spi0Clk::Pin6(pin_6) => Spi::new_blocking_rxonly(base.spi, pin_6, miso, base.config.unwrap_or_default()),
            Spi0Clk::Pin18(pin_18) => Spi::new_blocking_rxonly(base.spi, pin_18, miso, base.config.unwrap_or_default()),
            Spi0Clk::Pin22(pin_22) => Spi::new_blocking_rxonly(base.spi, pin_22, miso, base.config.unwrap_or_default()),
        }
    }
}
