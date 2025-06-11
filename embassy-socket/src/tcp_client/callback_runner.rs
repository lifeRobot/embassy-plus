use embassy_time::{Duration, Timer};
use crate::channel::callback_enum::CallbackEnum;
use crate::channel::read_channel::ReadChannel;
use crate::channel::socket_msg::SocketMsg;
use crate::tcp_client::callback::TcpClientCallBack;

/// tcp client callback runner
pub struct CallbackRunner<'d, const RC_SZ: usize, CB: TcpClientCallBack> {
    /// channel
    channel: &'d ReadChannel<'d, RC_SZ>,
    /// channel empty timeout, default is 100 millis
    timeout: Duration,
    /// msg cache
    socket_msg: SocketMsg<RC_SZ>,
    /// tcp client callback
    cb: CB,
}

/// custom method
impl<'d, const RC_SZ: usize, CB: TcpClientCallBack> CallbackRunner<'d, RC_SZ, CB> {
    /// create tcp client callback runner
    #[inline]
    pub fn new(channel: &'d ReadChannel<'d, RC_SZ>, cb: CB) -> Self {
        Self { channel, cb, timeout: Duration::from_millis(100), socket_msg: SocketMsg::default() }
    }

    /// set channel empty timeout
    #[inline]
    pub fn timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// run tcp client callback runner<br />
    /// this method is used for async callbacks
    #[inline]
    pub async fn run(&mut self) {
        loop { self.run_logic().await; }
    }

    /// run logic
    async fn run_logic(&mut self) {
        if self.channel.is_empty().await {
            Timer::after(self.timeout).await;
            return;
        }

        self.channel.read(&mut self.socket_msg).await;
        match self.socket_msg.callback_enum {
            CallbackEnum::Conn => self.cb.conn().await,
            CallbackEnum::Disconnect => self.cb.dis_conn().await,
            CallbackEnum::Recv => self.cb.recv(self.socket_msg.as_bytes()).await,
            CallbackEnum::Err(e) => self.cb.err(e).await,
        }
    }
}
