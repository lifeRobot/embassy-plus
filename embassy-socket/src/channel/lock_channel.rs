use embassy_net::IpEndpoint;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::rwlock::RwLock;
use embassy_sync::zerocopy_channel::Channel;
use crate::channel::callback_enum::CallbackEnum;
use crate::channel::socket_msg::SocketMsg;

/// socket lock channel<br />
/// N is channel bytes len
pub struct LockChannel<'d, const N: usize> {
    /// channel
    pub channel: RwLock<CriticalSectionRawMutex, Channel<'d, CriticalSectionRawMutex, SocketMsg<N>>>,
}

/// custom method
impl<'d, const N: usize> LockChannel<'d, N> {
    /// create read channel
    #[inline]
    pub fn new(buf: &'d mut [SocketMsg<N>]) -> Self {
        Self { channel: RwLock::new(Channel::new(buf)) }
    }

    /// channel is empty
    #[inline]
    pub async fn is_empty(&self) -> bool {
        self.channel.read().await.is_empty()
    }

    /// channel is full
    #[inline]
    pub async fn is_full(&self) -> bool {
        self.channel.read().await.is_full()
    }

    /// clear channel
    #[inline]
    pub async fn clear(&self) {
        self.channel.write().await.clear();
    }

    /// send bytes data and set callback logic
    pub async fn send_bytes(&self, bytes: &[u8], endpoint: Option<IpEndpoint>) {
        // split send
        let mut bytes_iter = bytes.chunks_exact(N);
        for byte in bytes_iter.by_ref() {
            // if full stop send, if send will be stop task
            let mut ch = self.channel.write().await;
            if ch.is_full() { return; }

            let mut sender = ch.split().0;
            let socket_msg = sender.send().await;
            socket_msg.bytes.copy_from_slice(byte);
            socket_msg.len = N;
            socket_msg.callback_enum = CallbackEnum::Recv;
            if let Some(endpoint) = endpoint {
                socket_msg.endpoint = endpoint;
            }
            sender.send_done();
            drop(ch);
        }

        // send last bytes
        let byte = bytes_iter.remainder();
        if byte.is_empty() { return; }
        // if full stop send, if send will be stop task
        let mut ch = self.channel.write().await;
        if ch.is_full() { return; }
        let mut sender = ch.split().0;
        let msg = sender.send().await;
        msg.bytes[..byte.len()].copy_from_slice(byte);
        msg.len = byte.len();
        msg.callback_enum = CallbackEnum::Recv;
        if let Some(endpoint) = endpoint {
            msg.endpoint = endpoint;
        }
        sender.send_done();
    }
}
