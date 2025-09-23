use embassy_hal_internal::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_1, UART0};
use crate::builder::uart::uart0::rx::Uart0RxBuilder;
use crate::builder::uart::uart0::tx::Uart0TxBuilder;
use crate::builder::uart::uart0::Uart0Builder;

/// uart0 trait
pub trait Uart0Trait<'d> {
    /// create uart0 builder
    fn builder(self, rx: Peri<'d, PIN_1>, tx: Peri<'d, PIN_0>) -> Uart0Builder<'d>;

    /// create uart0 tx builder
    fn tx_builder(self, tx: Peri<'d, PIN_0>) -> Uart0TxBuilder<'d>;

    /// create uart1 rx builder
    fn rx_builder(self, rx: Peri<'d, PIN_1>) -> Uart0RxBuilder<'d>;
}

/// support uart0 trait
impl<'d> Uart0Trait<'d> for Peri<'d, UART0> {
    #[inline]
    fn builder(self, rx: Peri<'d, PIN_1>, tx: Peri<'d, PIN_0>) -> Uart0Builder<'d> {
        Uart0Builder::new(self, rx, tx)
    }

    #[inline]
    fn tx_builder(self, tx: Peri<'d, PIN_0>) -> Uart0TxBuilder<'d> {
        Uart0TxBuilder::new(self, tx)
    }

    #[inline]
    fn rx_builder(self, rx: Peri<'d, PIN_1>) -> Uart0RxBuilder<'d> {
        Uart0RxBuilder::new(self, rx)
    }
}
