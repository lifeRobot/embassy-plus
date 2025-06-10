use core::net::Ipv4Addr;
use embassy_net::Stack;
use crate::channel::read_channel::ReadChannel;
use crate::connection::socket_state::SocketState;
use crate::tcp_client;
use crate::tcp_client::callback::TcpClientCallBack;
use crate::tcp_client::callback_runner::CallbackRunner;
use crate::tcp_client::read_runner::ReadRunner;
use crate::tcp_server::callback::TcpServerCallBack;
use crate::tcp_server::TcpServer;

/// socket builder trait, let stack build tcp client or tcp server
pub trait SocketBuilderTrait<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize> {
    /// build tcp client
    fn build_tcp_client<CB: TcpClientCallBack, const RC_SZ: usize>(self, ip: Ipv4Addr, port: u16, cb: CB, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, channel: &'d ReadChannel<'d, RC_SZ>,
    ) -> (ReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ>, CallbackRunner<'d, RC_SZ, CB>);

    /// build tcp server
    fn build_tcp_server<CB: TcpServerCallBack>(self, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, port: u16, cb: &'d CB) -> TcpServer<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB>;
}

/// support socket to build tcp client/server
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize>
SocketBuilderTrait<'d, N, TX_SZ, RX_SZ, BUF_SIZE>
for Stack<'d> {
    #[inline]
    fn build_tcp_client<CB: TcpClientCallBack, const RC_SZ: usize>(
        self, ip: Ipv4Addr, port: u16, cb: CB, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, channel: &'d ReadChannel<'d, RC_SZ>)
        -> (ReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ>, CallbackRunner<'d, RC_SZ, CB>) {
        tcp_client::build(self, ip, port, channel, state, cb)
    }

    #[inline]
    fn build_tcp_server<CB: TcpServerCallBack>(
        self, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, port: u16, cb: &'d CB)
        -> TcpServer<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB> {
        TcpServer::new(self, state, port, cb)
    }
}
