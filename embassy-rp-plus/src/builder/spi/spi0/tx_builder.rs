use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Blocking, Config, MosiPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi0::clk::Spi0Clk;
use crate::builder::spi::spi0::mosi::Spi0Mosi;

/// spi0 tx builder
pub struct Spi0TxBuilder<'d> {
    /// spi0 base data
    pub base: SpiBase<'d, SPI0>,
    /// clk pin
    pub clk: Option<Spi0Clk<'d>>,
    /// mosi pin
    pub mosi: Spi0Mosi<'d>,
}

/// custom method
impl<'d> Spi0TxBuilder<'d> {
    /// crate builder, default nosck
    #[inline]
    pub fn new(spi: Peri<'d, SPI0>, mosi: Spi0Mosi<'d>) -> Self {
        Self { base: SpiBase::new(spi), clk: None, mosi }
    }

    /// set spi config
    #[inline]
    pub fn config(mut self, config: Config) -> Self {
        self.base.set_config(config);
        self
    }

    /// set clk, use sck<br />
    /// more see [Spi::<_, Async>::new_txonly_nosck] or [Spi::<_, Async>]
    #[inline]
    pub fn clk(mut self, clk: impl Into<Spi0Clk<'d>>) -> Self {
        self.clk = Some(clk.into());
        self
    }

    /// build spi tx by async<br />
    /// more see [Spi::<_, Async>::new_txonly] or [Spi::<_, Async>::new_txonly_nosck]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI0, Async> {
        match self.mosi {
            Spi0Mosi::Pin3(pin_3) => Self::build_clk(self.clk, pin_3, self.base, tx_dma),
            Spi0Mosi::Pin7(pin_7) => Self::build_clk(self.clk, pin_7, self.base, tx_dma),
            Spi0Mosi::Pin19(pin_19) => Self::build_clk(self.clk, pin_19, self.base, tx_dma),
            Spi0Mosi::Pin23(pin_23) => Self::build_clk(self.clk, pin_23, self.base, tx_dma),
        }
    }

    /// build by clk
    fn build_clk(
        clk: Option<Spi0Clk<'d>>,
        mosi: Peri<'d, impl MosiPin<SPI0> + 'd>,
        base: SpiBase<'d, SPI0>,
        tx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI0, Async> {
        let clk = match clk {
            Some(clk) => clk,
            None => return Spi::new_txonly_nosck(base.spi, mosi, tx_dma, base.config.unwrap_or_default()),
        };

        match clk {
            Spi0Clk::Pin2(pin_2) => Spi::new_txonly(base.spi, pin_2, mosi, tx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin6(pin_6) => Spi::new_txonly(base.spi, pin_6, mosi, tx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin18(pin_18) => Spi::new_txonly(base.spi, pin_18, mosi, tx_dma, base.config.unwrap_or_default()),
            Spi0Clk::Pin22(pin_22) => Spi::new_txonly(base.spi, pin_22, mosi, tx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build spi tx by blocking<br />
    /// more see [Spi::<_, Blocking>::new_blocking_txonly] or [Spi::<_, Blocking>::new_blocking_txonly_nosck]
    pub fn build_blocking(self) -> Spi<'d, SPI0, Blocking> {
        match self.mosi {
            Spi0Mosi::Pin3(pin_3) => Self::build_clk_blocking(self.clk, pin_3, self.base),
            Spi0Mosi::Pin7(pin_7) => Self::build_clk_blocking(self.clk, pin_7, self.base),
            Spi0Mosi::Pin19(pin_19) => Self::build_clk_blocking(self.clk, pin_19, self.base),
            Spi0Mosi::Pin23(pin_23) => Self::build_clk_blocking(self.clk, pin_23, self.base),
        }
    }

    /// build blocking by clk
    fn build_clk_blocking(
        clk: Option<Spi0Clk<'d>>,
        mosi: Peri<'d, impl MosiPin<SPI0> + 'd>,
        base: SpiBase<'d, SPI0>) -> Spi<'d, SPI0, Blocking> {
        let clk = match clk {
            Some(clk) => clk,
            None => return Spi::new_blocking_txonly_nosck(base.spi, mosi, base.config.unwrap_or_default()),
        };

        match clk {
            Spi0Clk::Pin2(pin_2) => Spi::new_blocking_txonly(base.spi, pin_2, mosi, base.config.unwrap_or_default()),
            Spi0Clk::Pin6(pin_6) => Spi::new_blocking_txonly(base.spi, pin_6, mosi, base.config.unwrap_or_default()),
            Spi0Clk::Pin18(pin_18) => Spi::new_blocking_txonly(base.spi, pin_18, mosi, base.config.unwrap_or_default()),
            Spi0Clk::Pin22(pin_22) => Spi::new_blocking_txonly(base.spi, pin_22, mosi, base.config.unwrap_or_default()),
        }
    }
}
