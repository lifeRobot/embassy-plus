use embassy_hal_internal::Peri;
use embassy_rp::peripherals::SPI0;
use crate::builder::spi::spi0::clk::Spi0Clk;
use crate::builder::spi::spi0::miso::Spi0Miso;
use crate::builder::spi::spi0::mosi::Spi0Mosi;
use crate::builder::spi::spi0::rx_builder::Spi0RxBuilder;
use crate::builder::spi::spi0::Spi0Builder;
use crate::builder::spi::spi0::tx_builder::Spi0TxBuilder;

/// spi0 trait
pub trait Spi0Trait<'d> {
    /// create spi0 builder
    fn builder(self, clk: impl Into<Spi0Clk<'d>>, mosi: impl Into<Spi0Mosi<'d>>, miso: impl Into<Spi0Miso<'d>>) -> Spi0Builder<'d>;

    /// create spi0 tx builder
    fn tx_builder(self, mosi: impl Into<Spi0Mosi<'d>>) -> Spi0TxBuilder<'d>;

    /// create spi0 rx builder
    fn rx_builder(self, clk: impl Into<Spi0Clk<'d>>, miso: impl Into<Spi0Miso<'d>>) -> Spi0RxBuilder<'d>;
}

/// support spi0 trait
impl<'d> Spi0Trait<'d> for Peri<'d, SPI0> {
    #[inline]
    fn builder(self, clk: impl Into<Spi0Clk<'d>>, mosi: impl Into<Spi0Mosi<'d>>, miso: impl Into<Spi0Miso<'d>>) -> Spi0Builder<'d> {
        Spi0Builder::new(self, clk.into(), mosi.into(), miso.into())
    }

    #[inline]
    fn tx_builder(self, mosi: impl Into<Spi0Mosi<'d>>) -> Spi0TxBuilder<'d> {
        Spi0TxBuilder::new(self, mosi.into())
    }

    #[inline]
    fn rx_builder(self, clk: impl Into<Spi0Clk<'d>>, miso: impl Into<Spi0Miso<'d>>) -> Spi0RxBuilder<'d> {
        Spi0RxBuilder::new(self, clk.into(), miso.into())
    }
}
