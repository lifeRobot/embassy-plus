use embassy_hal_internal::Peri;
use embassy_rp::peripherals::UART1;
use crate::builder::uart::uart1::rx::Uart1Rx;
use crate::builder::uart::uart1::rx_builder::Uart1RxBuilder;
use crate::builder::uart::uart1::tx::Uart1Tx;
use crate::builder::uart::uart1::tx_builder::Uart1TxBuilder;
use crate::builder::uart::uart1::Uart1Builder;

/// uart1 trait
pub trait Uart1Trait<'d> {
    /// create uart1 builder
    fn builder(self, rx: impl Into<Uart1Rx<'d>>, tx: impl Into<Uart1Tx<'d>>) -> Uart1Builder<'d>;

    /// create uart1 tx builder
    fn tx_builder(self, tx: impl Into<Uart1Tx<'d>>) -> Uart1TxBuilder<'d>;

    /// create uart1 rx builder
    fn rx_builder(self, rx: impl Into<Uart1Rx<'d>>) -> Uart1RxBuilder<'d>;
}

/// support uart1 trait
impl<'d> Uart1Trait<'d> for Peri<'d, UART1> {
    #[inline]
    fn builder(self, rx: impl Into<Uart1Rx<'d>>, tx: impl Into<Uart1Tx<'d>>) -> Uart1Builder<'d> {
        Uart1Builder::new(self, rx.into(), tx.into())
    }

    #[inline]
    fn tx_builder(self, tx: impl Into<Uart1Tx<'d>>) -> Uart1TxBuilder<'d> {
        Uart1TxBuilder::new(self, tx.into())
    }

    #[inline]
    fn rx_builder(self, rx: impl Into<Uart1Rx<'d>>) -> Uart1RxBuilder<'d> {
        Uart1RxBuilder::new(self, rx.into())
    }
}
