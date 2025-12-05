use embassy_hal_internal::Peri;
use embassy_rp::dma::Channel;
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi::{Async, Blocking, Config, MosiPin, Spi};
use crate::builder::spi::base::SpiBase;
use crate::builder::spi::spi1::clk::Spi1Clk;
use crate::builder::spi::spi1::mosi::Spi1Mosi;

/// spi1 tx builder
pub struct Spi1TxBuilder<'d> {
    /// spi1 base data
    pub base: SpiBase<'d, SPI1>,
    /// clk pin
    pub clk: Option<Spi1Clk<'d>>,
    /// mosi pin
    pub mosi: Spi1Mosi<'d>,
}

/// custom method
impl<'d> Spi1TxBuilder<'d> {
    /// crate builder, default nosck
    #[inline]
    pub fn new(spi: Peri<'d, SPI1>, mosi: Spi1Mosi<'d>) -> Self {
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
    pub fn clk(mut self, clk: impl Into<Spi1Clk<'d>>) -> Self {
        self.clk = Some(clk.into());
        self
    }

    /// build spi tx by async<br />
    /// more see [Spi::<_, Async>::new_txonly] or [Spi::<_, Async>::new_txonly_nosck]
    pub fn build(self, tx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI1, Async> {
        match self.mosi {
            Spi1Mosi::Pin11(pin_11) => Self::build_clk(self.clk, pin_11, self.base, tx_dma),
            Spi1Mosi::Pin15(pin_15) => Self::build_clk(self.clk, pin_15, self.base, tx_dma),
            Spi1Mosi::Pin27(pin_27) => Self::build_clk(self.clk, pin_27, self.base, tx_dma),
        }
    }

    /// build by clk
    fn build_clk(
        clk: Option<Spi1Clk<'d>>,
        mosi: Peri<'d, impl MosiPin<SPI1> + 'd>,
        base: SpiBase<'d, SPI1>,
        tx_dma: Peri<'d, impl Channel>) -> Spi<'d, SPI1, Async> {
        let clk = match clk {
            Some(clk) => clk,
            None => return Spi::new_txonly_nosck(base.spi, mosi, tx_dma, base.config.unwrap_or_default()),
        };

        match clk {
            Spi1Clk::Pin10(pin_10) => Spi::new_txonly(base.spi, pin_10, mosi, tx_dma, base.config.unwrap_or_default()),
            Spi1Clk::Pin14(pin_14) => Spi::new_txonly(base.spi, pin_14, mosi, tx_dma, base.config.unwrap_or_default()),
            Spi1Clk::Pin26(pin_26) => Spi::new_txonly(base.spi, pin_26, mosi, tx_dma, base.config.unwrap_or_default()),
        }
    }

    /// build spi tx by blocking<br />
    /// more see [Spi::<_, Blocking>::new_blocking_txonly] or [Spi::<_, Blocking>::new_blocking_txonly_nosck]
    pub fn build_blocking(self) -> Spi<'d, SPI1, Blocking> {
        match self.mosi {
            Spi1Mosi::Pin11(pin_11) => Self::build_clk_blocking(self.clk, pin_11, self.base),
            Spi1Mosi::Pin15(pin_15) => Self::build_clk_blocking(self.clk, pin_15, self.base),
            Spi1Mosi::Pin27(pin_27) => Self::build_clk_blocking(self.clk, pin_27, self.base),
        }
    }

    /// build blocking by clk
    fn build_clk_blocking(
        clk: Option<Spi1Clk<'d>>,
        mosi: Peri<'d, impl MosiPin<SPI1> + 'd>,
        base: SpiBase<'d, SPI1>) -> Spi<'d, SPI1, Blocking> {
        let clk = match clk {
            Some(clk) => clk,
            None => return Spi::new_blocking_txonly_nosck(base.spi, mosi, base.config.unwrap_or_default()),
        };

        match clk {
            Spi1Clk::Pin10(pin_10) => Spi::new_blocking_txonly(base.spi, pin_10, mosi, base.config.unwrap_or_default()),
            Spi1Clk::Pin14(pin_14) => Spi::new_blocking_txonly(base.spi, pin_14, mosi, base.config.unwrap_or_default()),
            Spi1Clk::Pin26(pin_26) => Spi::new_blocking_txonly(base.spi, pin_26, mosi, base.config.unwrap_or_default()),
        }
    }
}
