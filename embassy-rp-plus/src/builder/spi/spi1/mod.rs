use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi::{Async, Blocking, Config, MisoPin, MosiPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::clk::Spi1Clk;
use crate::builder::spi::spi1::miso::Spi1Miso;
use crate::builder::spi::spi1::mosi::Spi1Mosi;
use crate::builder::spi::spi1::rx_builder::Spi1RxBuilder;

pub mod clk;
pub mod mosi;
pub mod miso;
pub mod rx_builder;
pub mod tx_builder;

/// spi1 builder
pub struct Spi1Builder<'d> {
    /// spi1 base data
    pub base: SpiBase<'d, SPI1>,
    /// clk pin
    pub clk: Spi1Clk<'d>,
    /// mosi pin
    pub mosi: Spi1Mosi<'d>,
    /// miso pin
    pub miso: Spi1Miso<'d>,
}

/// custom method
impl<'d> Spi1Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(spi1: Peri<'d, SPI1>, clk: Spi1Clk<'d>, mosi: Spi1Mosi<'d>, miso: Spi1Miso<'d>) -> Self {
        Self { base: SpiBase::new(spi1), clk, mosi, miso }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build spi1 by async<br />
    /// more see [Spi::<_, Async>::new]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>, rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI1, Async> {
        let rx = Spi1RxBuilder { base: self.base, clk: self.clk, miso: self.miso };
        match self.mosi {
            Spi1Mosi::Pin11(pin_11) => Self::build_rx(rx, pin_11, tx_dma, rx_dma),
            Spi1Mosi::Pin15(pin_15) => Self::build_rx(rx, pin_15, tx_dma, rx_dma),
            Spi1Mosi::Pin27(pin_27) => Self::build_rx(rx, pin_27, tx_dma, rx_dma),
        }
    }

    /// build by rx
    fn build_rx(
        rx: Spi1RxBuilder<'d>,
        mosi: Peri<'d, impl MosiPin<SPI1> + 'd>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI1, Async> {
        match rx.miso {
            Spi1Miso::Pin8(pin_8) => Self::build_clk(rx.clk, mosi, pin_8, rx.base, tx_dma, rx_dma),
            Spi1Miso::Pin12(pin_12) => Self::build_clk(rx.clk, mosi, pin_12, rx.base, tx_dma, rx_dma),
            Spi1Miso::Pin24(pin_24) => Self::build_clk(rx.clk, mosi, pin_24, rx.base, tx_dma, rx_dma),
            Spi1Miso::Pin28(pin_28) => Self::build_clk(rx.clk, mosi, pin_28, rx.base, tx_dma, rx_dma),
        }
    }

    /// build by clk
    fn build_clk(
        clk: Spi1Clk<'d>,
        mosi: Peri<'d, impl MosiPin<SPI1> + 'd>,
        miso: Peri<'d, impl MisoPin<SPI1> + 'd>,
        base: SpiBase<'d, SPI1>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI1, Async> {
        match clk {
            Spi1Clk::Pin10(pin_10) => Spi::new(base.spi, pin_10, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi1Clk::Pin14(pin_14) => Spi::new(base.spi, pin_14, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi1Clk::Pin26(pin_26) => Spi::new(base.spi, pin_26, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build spi1 by blocking<br />
    /// more see [Spi::<_, Blocking>::new]
    pub fn build_blocking(self) -> Spi<'d, SPI1, Blocking> {
        let rx = Spi1RxBuilder { base: self.base, clk: self.clk, miso: self.miso };
        match self.mosi {
            Spi1Mosi::Pin11(pin_11) => Self::build_rx_blocking(rx, pin_11),
            Spi1Mosi::Pin15(pin_15) => Self::build_rx_blocking(rx, pin_15),
            Spi1Mosi::Pin27(pin_27) => Self::build_rx_blocking(rx, pin_27),
        }
    }

    /// build blocking by rx
    fn build_rx_blocking(
        rx: Spi1RxBuilder<'d>,
        mosi: Peri<'d, impl MosiPin<SPI1> + 'd>) -> Spi<'d, SPI1, Blocking> {
        match rx.miso {
            Spi1Miso::Pin8(pin_8) => Self::build_clk_blocking(rx.clk, mosi, pin_8, rx.base),
            Spi1Miso::Pin12(pin_12) => Self::build_clk_blocking(rx.clk, mosi, pin_12, rx.base),
            Spi1Miso::Pin24(pin_24) => Self::build_clk_blocking(rx.clk, mosi, pin_24, rx.base),
            Spi1Miso::Pin28(pin_28) => Self::build_clk_blocking(rx.clk, mosi, pin_28, rx.base),
        }
    }

    /// build blocking by clk
    fn build_clk_blocking(
        clk: Spi1Clk<'d>,
        mosi: Peri<'d, impl MosiPin<SPI1> + 'd>,
        miso: Peri<'d, impl MisoPin<SPI1> + 'd>,
        base: SpiBase<'d, SPI1>) -> Spi<'d, SPI1, Blocking> {
        match clk {
            Spi1Clk::Pin10(pin_10) => Spi::new_blocking(base.spi, pin_10, mosi, miso, base.config.unwrap_or_default()),
            Spi1Clk::Pin14(pin_14) => Spi::new_blocking(base.spi, pin_14, mosi, miso, base.config.unwrap_or_default()),
            Spi1Clk::Pin26(pin_26) => Spi::new_blocking(base.spi, pin_26, mosi, miso, base.config.unwrap_or_default()),
        }
    }
}
