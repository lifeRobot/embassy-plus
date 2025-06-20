use core::net::Ipv4Addr;
use embassy_net::Stack;
use crate::channel::SocketChannel;
use crate::connection::socket_state::SocketState;
use crate::tcp_client::callback::TcpClientCallBack;
use crate::tcp_client::callback_runner::CallbackRunner;
use crate::tcp_client::read_runner::ReadRunner;

pub mod callback;
pub mod read_runner;
pub mod callback_runner;

/// build tcp client runner
#[inline]
pub fn build<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, const RC_SZ: usize, const WC_SZ: usize, CB: TcpClientCallBack>(
    stack: Stack<'d>,
    ip: Ipv4Addr,
    port: u16,
    socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
    state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
    cb: CB)
    -> (ReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ>, CallbackRunner<'d, RC_SZ, CB>) {
    (ReadRunner::new(stack, ip, port, socket_channel, state), CallbackRunner::new(&socket_channel.read_channel, cb))
}

/// just create runner<br />
/// read tcp data use `socket_channel.read_channel`<br />
/// write tcp data use `socket_channel.write_channel`<br />
/// if you use multiple tcp clients,
/// only enable runner, reduce the number of `embassy_executor::task`,
/// and then call read yourself to read data to save resources
#[inline]
pub fn build_runner<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, const RC_SZ: usize, const WC_SZ: usize>(
    stack: Stack<'d>,
    ip: Ipv4Addr,
    port: u16,
    socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
    state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>) -> ReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ> {
    ReadRunner::new(stack, ip, port, socket_channel, state)
}
