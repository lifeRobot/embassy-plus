use embassy_hal_internal::Peri;
use embassy_rp::peripherals::UART0;
use crate::builder::uart::uart0::rx::Uart0Rx;
use crate::builder::uart::uart0::rx_builder::Uart0RxBuilder;
use crate::builder::uart::uart0::tx::Uart0Tx;
use crate::builder::uart::uart0::tx_builder::Uart0TxBuilder;
use crate::builder::uart::uart0::Uart0Builder;

/// uart0 trait
pub trait Uart0Trait<'d> {
    /// create uart0 builder
    fn builder(self, rx: impl Into<Uart0Rx<'d>>, tx: impl Into<Uart0Tx<'d>>) -> Uart0Builder<'d>;

    /// create uart0 tx builder
    fn tx_builder(self, tx: impl Into<Uart0Tx<'d>>) -> Uart0TxBuilder<'d>;

    /// create uart1 rx builder
    fn rx_builder(self, rx: impl Into<Uart0Rx<'d>>) -> Uart0RxBuilder<'d>;
}

/// support uart0 trait
impl<'d> Uart0Trait<'d> for Peri<'d, UART0> {
    #[inline]
    fn builder(self, rx: impl Into<Uart0Rx<'d>>, tx: impl Into<Uart0Tx<'d>>) -> Uart0Builder<'d> {
        Uart0Builder::new(self, rx.into(), tx.into())
    }

    #[inline]
    fn tx_builder(self, tx: impl Into<Uart0Tx<'d>>) -> Uart0TxBuilder<'d> {
        Uart0TxBuilder::new(self, tx.into())
    }

    #[inline]
    fn rx_builder(self, rx: impl Into<Uart0Rx<'d>>) -> Uart0RxBuilder<'d> {
        Uart0RxBuilder::new(self, rx.into())
    }
}
