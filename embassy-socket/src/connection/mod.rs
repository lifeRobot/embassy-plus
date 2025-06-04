use core::ptr::NonNull;
use embassy_net::Stack;
use embassy_net::tcp::{Error, TcpSocket};
use crate::connection::socket_state::SocketState;

pub mod socket_state;
pub(crate) mod pool;

/// tcp connection
pub struct TcpConnection<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize> {
    /// tcp socket
    pub socket: TcpSocket<'d>,
    /// socket state, memory pool
    state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
    /// memory buf
    bufs: NonNull<([u8; TX_SZ], [u8; RX_SZ], [u8; BUF_SIZE])>,
}

/// custom method
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize> TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE> {
    /// create tcp connection
    pub fn new(stack: Stack<'d>, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>) -> Result<Self, Error> {
        let mut bufs = state.pool.alloc().ok_or(Error::ConnectionReset)?;
        Ok(Self {
            socket: unsafe { TcpSocket::new(stack, &mut bufs.as_mut().1, &mut bufs.as_mut().0) },
            state,
            bufs,
        })
    }

    /// try read to buf
    pub async fn try_read(&mut self) -> Result<&[u8], Error> {
        let bytes = unsafe { &mut self.bufs.as_mut().2 };
        let len = self.socket.read(bytes).await?;
        if len == 0 { return Err(Error::ConnectionReset); }

        Ok(&bytes[0..len])
    }
}

/// support drop
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize> Drop for TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE> {
    fn drop(&mut self) {
        self.socket.close();
        self.state.pool.free(self.bufs);
    }
}
