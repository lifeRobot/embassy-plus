use embassy_hal_internal::Peri;
use embassy_rp::peripherals::SPI1;
use crate::builder::spi::spi1::clk::Spi1Clk;
use crate::builder::spi::spi1::miso::Spi1Miso;
use crate::builder::spi::spi1::mosi::Spi1Mosi;
use crate::builder::spi::spi1::rx_builder::Spi1RxBuilder;
use crate::builder::spi::spi1::Spi1Builder;
use crate::builder::spi::spi1::tx_builder::Spi1TxBuilder;

/// spi1 trait
pub trait Spi1Trait<'d> {
    /// create spi1 builder
    fn builder(self, clk: impl Into<Spi1Clk<'d>>, mosi: impl Into<Spi1Mosi<'d>>, miso: impl Into<Spi1Miso<'d>>) -> Spi1Builder<'d>;

    /// create spi1 tx builder
    fn tx_builder(self, mosi: impl Into<Spi1Mosi<'d>>) -> Spi1TxBuilder<'d>;

    /// create spi0 rx builder
    fn rx_builder(self, clk: impl Into<Spi1Clk<'d>>, miso: impl Into<Spi1Miso<'d>>) -> Spi1RxBuilder<'d>;
}

/// support spi1 trait
impl<'d> Spi1Trait<'d> for Peri<'d, SPI1> {
    #[inline]
    fn builder(self, clk: impl Into<Spi1Clk<'d>>, mosi: impl Into<Spi1Mosi<'d>>, miso: impl Into<Spi1Miso<'d>>) -> Spi1Builder<'d> {
        Spi1Builder::new(self, clk.into(), mosi.into(), miso.into())
    }

    #[inline]
    fn tx_builder(self, mosi: impl Into<Spi1Mosi<'d>>) -> Spi1TxBuilder<'d> {
        Spi1TxBuilder::new(self, mosi.into())
    }

    #[inline]
    fn rx_builder(self, clk: impl Into<Spi1Clk<'d>>, miso: impl Into<Spi1Miso<'d>>) -> Spi1RxBuilder<'d> {
        Spi1RxBuilder::new(self, clk.into(), miso.into())
    }
}
