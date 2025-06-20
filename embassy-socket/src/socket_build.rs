use core::net::Ipv4Addr;
use embassy_net::Stack;
use crate::channel::SocketChannel;
use crate::connection::socket_state::SocketState;
use crate::tcp_client;
use crate::tcp_client::callback::TcpClientCallBack;
use crate::tcp_client::callback_runner::CallbackRunner;
use crate::tcp_client::read_runner::ReadRunner as TcpClientReadRunner;
use crate::tcp_server::callback::TcpServerCallBack;
use crate::tcp_server::read_runner::ReadRunner as TcpServerReadRunner;
use crate::tcp_server::TcpServer;

/// socket builder trait, let stack build tcp client or tcp server
pub trait SocketBuilderTrait<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize> {
    /// build tcp client
    fn build_tcp_client<CB: TcpClientCallBack, const RC_SZ: usize, const WC_SZ: usize>(
        self, ip: Ipv4Addr,
        port: u16,
        cb: CB,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
        socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>)
        -> (TcpClientReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ>, CallbackRunner<'d, RC_SZ, CB>);

    /// build tcp server
    fn build_tcp_server<CB: TcpServerCallBack>(self, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, port: u16, cb: &'d CB,
    ) -> TcpServer<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB>;

    /// build tcp client runner<br />
    /// more see [tcp_client::build_runner]
    fn build_tcp_client_runner<const RC_SZ: usize, const WC_SZ: usize>(
        self, ip: Ipv4Addr,
        port: u16,
        socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>)
        -> TcpClientReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ>;

    /// build tcp server runner<br />
    /// more see [crate::tcp_server::build_runner]
    fn build_tcp_server_runner<const RC_SZ: usize, const WC_SZ: usize>(
        self,
        port: u16,
        socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>)
        -> TcpServerReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ>;
}

/// support socket to build tcp client/server
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize>
SocketBuilderTrait<'d, N, TX_SZ, RX_SZ, BUF_SIZE>
for Stack<'d> {
    #[inline]
    fn build_tcp_client<CB: TcpClientCallBack, const RC_SZ: usize, const WC_SZ: usize>(
        self,
        ip: Ipv4Addr,
        port: u16,
        cb: CB,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
        socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>)
        -> (TcpClientReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ>, CallbackRunner<'d, RC_SZ, CB>) {
        tcp_client::build(self, ip, port, socket_channel, state, cb)
    }

    #[inline]
    fn build_tcp_server<CB: TcpServerCallBack>(
        self,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
        port: u16,
        cb: &'d CB) -> TcpServer<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB> {
        TcpServer::new(self, state, port, cb)
    }

    #[inline]
    fn build_tcp_client_runner<const RC_SZ: usize, const WC_SZ: usize>(
        self,
        ip: Ipv4Addr,
        port: u16,
        socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>) -> TcpClientReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ> {
        TcpClientReadRunner::new(self, ip, port, socket_channel, state)
    }

    #[inline]
    fn build_tcp_server_runner<const RC_SZ: usize, const WC_SZ: usize>(
        self,
        port: u16,
        socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>) -> TcpServerReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ> {
        TcpServerReadRunner::new(self, state, port, socket_channel)
    }
}
