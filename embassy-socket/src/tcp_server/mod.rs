use embassy_net::Stack;
use crate::connection::socket_state::SocketState;
use crate::tcp_server::callback::TcpServerCallBack;
use crate::tcp_server::runner::TcpServerRunner;

pub mod callback;
pub mod runner;
pub mod read_runner;
pub mod callback_runner;

/// tcp server
pub struct TcpServer<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, CB: TcpServerCallBack> {
    /// tcp server runner
    runner: TcpServerRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB>,
}

/// custom method
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, CB: TcpServerCallBack>
TcpServer<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB> {
    /// create tcp server
    #[inline]
    pub fn new(stack: Stack<'d>, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, port: u16, cb: &'d CB) -> Self {
        Self { runner: TcpServerRunner::new(stack, state, port, cb) }
    }

    /// create runner<br />
    /// it is recommended to only create N at most, and each representative you create can listen to one connection
    #[inline]
    pub fn create(&self) -> TcpServerRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB> {
        self.runner
    }
}
