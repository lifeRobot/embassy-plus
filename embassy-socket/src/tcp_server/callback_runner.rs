use embassy_time::{Duration, Timer};
use crate::channel::callback_enum::CallbackEnum;
use crate::channel::socket_msg::SocketMsg;
use crate::channel::SocketChannel;
use crate::tcp_server::callback::TcpServerCallBack;

/// tcp server callback runner
pub struct CallbackRunner<'d, const RC_SZ: usize, const WC_SZ: usize, CB: TcpServerCallBack> {
    /// socket channel
    socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
    /// channel empty timeout, default is 100 millis
    timeout: Duration,
    /// msg cache
    socket_msg: SocketMsg<RC_SZ>,
    /// tcp client callback
    cb: &'d CB,
}

/// custom method
impl<'d, const RC_SZ: usize, const WC_SZ: usize, CB: TcpServerCallBack> CallbackRunner<'d, RC_SZ, WC_SZ, CB> {
    /// create tcp server callback runner
    #[inline]
    pub fn new(socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>, cb: &'d CB) -> Self {
        Self { socket_channel, cb, timeout: Duration::from_millis(100), socket_msg: SocketMsg::default() }
    }

    /// set channel empty timeout
    #[inline]
    pub fn timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// run tcp client callback runner<br />
    /// this method is used for async callbacks
    #[inline]
    pub async fn run(&mut self, t: &mut CB::T) {
        loop { self.run_logic(t).await; }
    }

    /// run logic
    async fn run_logic(&mut self, t: &mut CB::T) {
        if self.socket_channel.read_channel.is_empty().await {
            Timer::after(self.timeout).await;
            return;
        }

        self.socket_channel.read_channel.read(&mut self.socket_msg).await;
        match self.socket_msg.callback_enum {
            CallbackEnum::Conn => self.cb.conn(self.socket_msg.endpoint, &self.socket_channel.write_channel, t).await,
            CallbackEnum::Disconnect => self.cb.dis_conn(self.socket_msg.endpoint, t).await,
            CallbackEnum::Recv => self.cb.recv(self.socket_msg.endpoint, self.socket_msg.as_bytes(), &self.socket_channel.write_channel, t).await,
            CallbackEnum::Err(e) => self.cb.err(e, t).await,
        }
    }
}
