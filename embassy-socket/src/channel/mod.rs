use embassy_net::tcp;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::zerocopy_channel::Channel;
use embassy_time::Timer;
use crate::channel::socket_msg::SocketMsg;
use crate::connection::TcpConnection;

pub mod socket_msg;

/// socket write channel<br />
/// N is channel len<br />
pub struct WriteChannel<'d, const N: usize> {
    /// channel
    channel: Mutex<CriticalSectionRawMutex, Channel<'d, CriticalSectionRawMutex, SocketMsg<N>>>,
    /// can send data
    can_send: Mutex<CriticalSectionRawMutex, bool>,
}

/// custom method
impl<'d, const N: usize> WriteChannel<'d, N> {
    /// create write channel
    #[inline]
    pub fn new(buf: &'d mut [SocketMsg<N>]) -> Self {
        Self { channel: Mutex::new(Channel::new(buf)), can_send: Mutex::new(false) }
    }

    /// channel is empty
    #[inline]
    pub async fn is_empty(&self) -> bool {
        self.channel.lock().await.is_empty()
    }

    /// enable channel, allow channels to send data
    #[inline]
    pub async fn enable(&self) {
        *self.can_send.lock().await = true;
    }

    /// disable channel, disable channel from sending data
    pub async fn disable(&self) {
        *self.can_send.lock().await = false;
        self.channel.lock().await.clear();
    }

    /// try tcp write data
    pub async fn try_tcp_write<const CN: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize>(
        &self,
        conn: &mut TcpConnection<'_, CN, TX_SZ, RX_SZ, BUF_SIZE>) -> Result<(), tcp::Error> {
        let mut ch = self.channel.lock().await;
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
        if !*self.can_send.lock().await { return; }

        // split send
        let mut bytes_iter = bytes.chunks_exact(N);
        for byte in bytes_iter.by_ref() {
            loop {
                if let Ok(mut msg) = self.channel.try_lock() {
                    let mut sender = msg.split().0;
                    let socket_msg = sender.send().await;
                    socket_msg.bytes.copy_from_slice(byte);
                    socket_msg.len = N;
                    sender.send_done();
                    drop(msg);
                    break;
                }
                Timer::after_millis(100).await;
            }
        }

        // send last bytes
        let byte = bytes_iter.remainder();
        if byte.is_empty() { return; }
        let mut ch = self.channel.lock().await;
        let mut sender = ch.split().0;
        let msg = sender.send().await;
        msg.bytes[..byte.len()].copy_from_slice(byte);
        msg.len = byte.len();
        sender.send_done();
    }

    /// send str data
    #[inline]
    pub async fn send_str(&self, s: &str) {
        self.send_bytes(s.as_bytes()).await;
    }
}
