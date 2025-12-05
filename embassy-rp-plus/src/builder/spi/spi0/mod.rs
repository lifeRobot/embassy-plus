use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Blocking, Config, MisoPin, MosiPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi0::clk::Spi0Clk;
use crate::builder::spi::spi0::miso::Spi0Miso;
use crate::builder::spi::spi0::mosi::Spi0Mosi;
use crate::builder::spi::spi0::rx_builder::Spi0RxBuilder;

pub mod clk;
pub mod mosi;
pub mod miso;
pub mod rx_builder;
pub mod tx_builder;

/// spi0 builder
pub struct Spi0Builder<'d> {
    /// spi0 base data
    pub base: SpiBase<'d, SPI0>,
    /// clk pin
    pub clk: Spi0Clk<'d>,
    /// mosi pin
    pub mosi: Spi0Mosi<'d>,
    /// miso pin
    pub miso: Spi0Miso<'d>,
}

/// custom method
impl<'d> Spi0Builder<'d> {
    /// crate builder
    #[inline]
    pub fn new(spi: Peri<'d, SPI0>, clk: Spi0Clk<'d>, mosi: Spi0Mosi<'d>, miso: Spi0Miso<'d>) -> Self {
        Self { base: SpiBase::new(spi), clk, mosi, miso }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// build spi0 by async<br />
    /// more see [Spi::<_, Async>::new]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>, rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI0, Async> {
        let rx = Spi0RxBuilder { base: self.base, clk: self.clk, miso: self.miso };
        match self.mosi {
            Spi0Mosi::Pin3(pin_3) => Self::build_rx(rx, pin_3, tx_dma, rx_dma),
            Spi0Mosi::Pin7(pin_7) => Self::build_rx(rx, pin_7, tx_dma, rx_dma),
            Spi0Mosi::Pin19(pin_19) => Self::build_rx(rx, pin_19, tx_dma, rx_dma),
            Spi0Mosi::Pin23(pin_23) => Self::build_rx(rx, pin_23, tx_dma, rx_dma),
        }
    }

    /// build by rx
    fn build_rx(
        rx: Spi0RxBuilder<'d>,
        mosi: Peri<'d, impl MosiPin<SPI0> + 'd>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI0, Async> {
        match rx.miso {
            Spi0Miso::Pin0(pin_0) => Self::build_clk(rx.clk, mosi, pin_0, rx.base, tx_dma, rx_dma),
            Spi0Miso::Pin4(pin_4) => Self::build_clk(rx.clk, mosi, pin_4, rx.base, tx_dma, rx_dma),
            Spi0Miso::Pin16(pin_16) => Self::build_clk(rx.clk, mosi, pin_16, rx.base, tx_dma, rx_dma),
            Spi0Miso::Pin20(pin_20) => Self::build_clk(rx.clk, mosi, pin_20, rx.base, tx_dma, rx_dma),
        }
    }

    /// build by clk
    fn build_clk(
        clk: Spi0Clk<'d>,
        mosi: Peri<'d, impl MosiPin<SPI0> + 'd>,
        miso: Peri<'d, impl MisoPin<SPI0> + 'd>,
        base: SpiBase<'d, SPI0>,
        tx_dma: Peri<'d, impl Channel>,
        rx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI0, Async> {
        match clk {
            Spi0Clk::Pin2(pin_2) => Spi::new(base.spi, pin_2, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin6(pin_6) => Spi::new(base.spi, pin_6, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin18(pin_18) => Spi::new(base.spi, pin_18, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin22(pin_22) => Spi::new(base.spi, pin_22, mosi, miso, tx_dma, rx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build spi0 by blocking<br />
    /// more see [Spi::<_, Blocking>::new_blocking]
    pub fn build_blocking(self) -> Spi<'d, SPI0, Blocking> {
        let rx = Spi0RxBuilder { base: self.base, clk: self.clk, miso: self.miso };
        match self.mosi {
            Spi0Mosi::Pin3(pin_3) => Self::build_rx_blocking(rx, pin_3),
            Spi0Mosi::Pin7(pin_7) => Self::build_rx_blocking(rx, pin_7),
            Spi0Mosi::Pin19(pin_19) => Self::build_rx_blocking(rx, pin_19),
            Spi0Mosi::Pin23(pin_23) => Self::build_rx_blocking(rx, pin_23),
        }
    }

    /// build blocking by rx
    fn build_rx_blocking(
        rx: Spi0RxBuilder<'d>,
        mosi: Peri<'d, impl MosiPin<SPI0> + 'd>) -> Spi<'d, SPI0, Blocking> {
        match rx.miso {
            Spi0Miso::Pin0(pin_0) => Self::build_clk_blocking(rx.clk, mosi, pin_0, rx.base),
            Spi0Miso::Pin4(pin_4) => Self::build_clk_blocking(rx.clk, mosi, pin_4, rx.base),
            Spi0Miso::Pin16(pin_16) => Self::build_clk_blocking(rx.clk, mosi, pin_16, rx.base),
            Spi0Miso::Pin20(pin_20) => Self::build_clk_blocking(rx.clk, mosi, pin_20, rx.base),
        }
    }

    /// build blocking by clk
    fn build_clk_blocking(
        clk: Spi0Clk<'d>,
        mosi: Peri<'d, impl MosiPin<SPI0> + 'd>,
        miso: Peri<'d, impl MisoPin<SPI0> + 'd>,
        base: SpiBase<'d, SPI0>) -> Spi<'d, SPI0, Blocking> {
        match clk {
            Spi0Clk::Pin2(pin_2) => Spi::new_blocking(base.spi, pin_2, mosi, miso, base.config.unwrap_or_default()),
            Spi0Clk::Pin6(pin_6) => Spi::new_blocking(base.spi, pin_6, mosi, miso, base.config.unwrap_or_default()),
            Spi0Clk::Pin18(pin_18) => Spi::new_blocking(base.spi, pin_18, mosi, miso, base.config.unwrap_or_default()),
            Spi0Clk::Pin22(pin_22) => Spi::new_blocking(base.spi, pin_22, mosi, miso, base.config.unwrap_or_default()),
        }
    }
}
