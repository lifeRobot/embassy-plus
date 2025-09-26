use embassy_net::tcp;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::rwlock::RwLock;
use crate::channel::lock_channel::LockChannel;
use crate::channel::socket_msg::SocketMsg;
use crate::connection::TcpConnection;

/// socket write channel<br />
/// N is channel len
pub struct WriteChannel<'d, const N: usize> {
    /// channel
    channel: LockChannel<'d, N>,
    /// can send data
    can_send: RwLock<CriticalSectionRawMutex, bool>,
}

/// custom method
impl<'d, const N: usize> WriteChannel<'d, N> {
    /// create write channel
    #[inline]
    pub fn new(buf: &'d mut [SocketMsg<N>]) -> Self {
        Self { channel: LockChannel::new(buf), can_send: RwLock::new(false) }
    }

    /// enable channel, allow channels to send data
    #[inline]
    pub async fn enable(&self) {
        *self.can_send.write().await = true;
    }

    /// disable channel, disable channel from sending data
    pub async fn disable(&self) {
        *self.can_send.write().await = false;
        self.channel.clear().await;
    }

    /// channel is empty
    #[inline]
    pub async fn is_empty(&self) -> bool {
        self.channel.is_empty().await
    }

    /// channel is full
    #[inline]
    pub async fn is_full(&self) -> bool {
        self.channel.is_full().await
    }

    /// try tcp write data
    pub async fn try_tcp_write<const CN: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize>(
        &self,
        conn: &mut TcpConnection<'_, CN, TX_SZ, RX_SZ, BUF_SIZE>) -> Result<(), tcp::Error> {
        let mut ch = self.channel.channel.write().await;
        let mut recv = ch.split().1;
        let msg = recv.receive().await;

        if let Err(e) = Self::try_conn_write(conn, msg.as_bytes()).await {
            recv.receive_done();
            return Err(e);
        }
        recv.receive_done();
        Ok(())
    }

    /// try conn write data
    async fn try_conn_write<const CN: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize>(
        conn: &mut TcpConnection<'_, CN, TX_SZ, RX_SZ, BUF_SIZE>, buf: &[u8]) -> Result<(), tcp::Error> {
        conn.socket.write(buf).await?;
        conn.socket.flush().await
    }

    #[inline]
    pub async fn tcp_write<const CN: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize>(
        &self, conn: &mut TcpConnection<'_, CN, TX_SZ, RX_SZ, BUF_SIZE>) {
        self.try_tcp_write(conn).await.ok();
    }

    /// send bytes data
    pub async fn send_bytes(&self, bytes: &[u8]) {
        if !*self.can_send.read().await { return; }

        self.channel.send_bytes(bytes, None).await;
    }

    /// send str data
    #[inline]
    pub async fn send_str(&self, s: &str) {
        self.send_bytes(s.as_bytes()).await;
    }
}
