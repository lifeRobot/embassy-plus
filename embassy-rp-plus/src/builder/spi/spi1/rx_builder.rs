use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi::{Async, Blocking, Config, MisoPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::clk::Spi1Clk;
use crate::builder::spi::spi1::miso::Spi1Miso;

/// spi1 rx builder
pub struct Spi1RxBuilder<'d> {
    /// spi1 base data
    pub base: SpiBase<'d, SPI1>,
    /// clk pin
    pub clk: Spi1Clk<'d>,
    /// miso pin
    pub miso: Spi1Miso<'d>,
}

/// custom method
impl<'d> Spi1RxBuilder<'d> {
    /// crate builder
    #[inline]
    pub fn new(spi: Peri<'d, SPI1>, clk: Spi1Clk<'d>, miso: Spi1Miso<'d>) -> Self {
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
    pub fn build(self, tx_dma: Peri<'d, impl Channel>, rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI1, Async> {
        match self.miso {
            Spi1Miso::Pin8(pin_8) => Self::build_clk(self.clk, pin_8, self.base, tx_dma, rx_dma),
            Spi1Miso::Pin12(pin_12) => Self::build_clk(self.clk, pin_12, self.base, tx_dma, rx_dma),
            Spi1Miso::Pin24(pin_24) => Self::build_clk(self.clk, pin_24, self.base, tx_dma, rx_dma),
            Spi1Miso::Pin28(pin_28) => Self::build_clk(self.clk, pin_28, self.base, tx_dma, rx_dma),
        }
    }

    /// build by clk
    fn build_clk(
        clk: Spi1Clk<'d>,
        miso: Peri<'d, impl MisoPin<SPI1> + 'd>,
        base: SpiBase<'d, SPI1>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI1, Async> {
        match clk {
            Spi1Clk::Pin10(pin_10) => Spi::new_rxonly(base.spi, pin_10, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi1Clk::Pin14(pin_14) => Spi::new_rxonly(base.spi, pin_14, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi1Clk::Pin26(pin_26) => Spi::new_rxonly(base.spi, pin_26, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build spi rx by async<br />
    /// more see [Spi::<_, Blocking>::new_blocking_rxonly]
    pub fn build_blocking(self) -> Spi<'d, SPI1, Blocking> {
        match self.miso {
            Spi1Miso::Pin8(pin_8) => Self::build_clk_blocking(self.clk, pin_8, self.base),
            Spi1Miso::Pin12(pin_12) => Self::build_clk_blocking(self.clk, pin_12, self.base),
            Spi1Miso::Pin24(pin_24) => Self::build_clk_blocking(self.clk, pin_24, self.base),
            Spi1Miso::Pin28(pin_28) => Self::build_clk_blocking(self.clk, pin_28, self.base),
        }
    }

    /// build blocking by clk
    fn build_clk_blocking(
        clk: Spi1Clk<'d>,
        miso: Peri<'d, impl MisoPin<SPI1> + 'd>,
        base: SpiBase<'d, SPI1>) -> Spi<'d, SPI1, Blocking> {
        match clk {
            Spi1Clk::Pin10(pin_10) => Spi::new_blocking_rxonly(base.spi, pin_10, miso, base.config.unwrap_or_default()),
            Spi1Clk::Pin14(pin_14) => Spi::new_blocking_rxonly(base.spi, pin_14, miso, base.config.unwrap_or_default()),
            Spi1Clk::Pin26(pin_26) => Spi::new_blocking_rxonly(base.spi, pin_26, miso, base.config.unwrap_or_default()),
        }
    }
}
