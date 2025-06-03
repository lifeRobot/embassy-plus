use crate::connection::pool::Pool;

/// socket connection<br />
/// N is socket number<br />
/// TX_SZ is socket tx size<br />
/// RX_SZ is socket rx size<br />
/// BUF_SIZE is read data buf size
pub struct SocketState<const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize> {
    /// memory pool
    pub pool: Pool<([u8; TX_SZ], [u8; RX_SZ], [u8; BUF_SIZE]), N>,
}

/// custom method
impl<const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize> SocketState<N, TX_SZ, RX_SZ, BUF_SIZE> {
    /// create socket connection
    pub const fn new() -> Self {
        Self { pool: Pool::new() }
    }
}
