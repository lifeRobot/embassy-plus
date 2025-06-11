use embassy_net::Stack;
use crate::channel::SocketChannel;
use crate::connection::socket_state::SocketState;
use crate::tcp_server::callback::TcpServerCallBack;
use crate::tcp_server::callback_runner::CallbackRunner;
use crate::tcp_server::read_runner::ReadRunner;

pub mod callback;
pub mod read_runner;
pub mod callback_runner;

/// tcp server
pub struct TcpServer<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, CB: TcpServerCallBack> {
    /// tcp stack
    stack: Stack<'d>,
    /// socket state, memory pool
    state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
    /// listener port
    port: u16,
    /// socket callback
    cb: &'d CB,
}

/// custom method
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, CB: TcpServerCallBack>
TcpServer<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB> {
    /// create tcp server
    #[inline]
    pub fn new(stack: Stack<'d>, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, port: u16, cb: &'d CB) -> Self {
        Self { stack, state, port, cb }
    }

    /// create runner<br />
    /// it is recommended to only create N at most, and each representative you create can listen to one connection
    #[inline]
    pub fn create<const RC_SZ: usize, const WC_SZ: usize>(&self, socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
    ) -> (ReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ>, CallbackRunner<'d, RC_SZ, WC_SZ, CB>) {
        (ReadRunner::new(self.stack, self.state, self.port, socket_channel),
         CallbackRunner::new(socket_channel, self.cb))
    }
}
