use embassy_net::IpEndpoint;
use crate::channel::callback_enum::CallbackEnum;
use crate::channel::lock_channel::LockChannel;
use crate::channel::socket_msg::SocketMsg;
use crate::err::SocketErr;

/// socket read channel <br />
/// N is channel bytes len
pub struct ReadChannel<'d, const N: usize> {
    /// channel
    channel: LockChannel<'d, N>,
}

/// custom method
impl<'d, const N: usize> ReadChannel<'d, N> {
    /// create read channel
    #[inline]
    pub fn new(buf: &'d mut [SocketMsg<N>]) -> Self {
        Self { channel: LockChannel::new(buf) }
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

    /// change callback logic and addr
    pub(crate) async fn callback_logic_addr(&self, callback_enum: CallbackEnum, endpoint: IpEndpoint) {
        let mut ch = self.channel.channel.lock().await;
        let mut send = ch.split().0;
        let socket_msg = send.send().await;
        socket_msg.callback_enum = callback_enum;
        socket_msg.endpoint = endpoint;
        send.send_done();
    }

    /// change callback logic
    pub(crate) async fn callback_logic(&self, callback_enum: CallbackEnum) {
        let mut ch = self.channel.channel.lock().await;
        let mut send = ch.split().0;
        let socket_msg = send.send().await;
        socket_msg.callback_enum = callback_enum;
        send.send_done();
    }

    /// socket connection
    #[inline]
    pub async fn conn(&self) {
        self.callback_logic(CallbackEnum::Conn).await
    }

    /// socket connection
    #[inline]
    pub async fn conn_addr(&self, endpoint: IpEndpoint) {
        self.callback_logic_addr(CallbackEnum::Conn, endpoint).await
    }

    /// socket dis connection
    #[inline]
    pub async fn dis_conn(&self) {
        self.callback_logic(CallbackEnum::Disconnect).await
    }

    /// socket dis connection
    #[inline]
    pub async fn dis_conn_addr(&self, endpoint: IpEndpoint) {
        self.callback_logic_addr(CallbackEnum::Disconnect, endpoint).await
    }

    /// socket error
    #[inline]
    pub async fn err(&self, socket_err: SocketErr) {
        self.callback_logic(CallbackEnum::Err(socket_err)).await
    }

    /// socket recv
    #[inline]
    pub async fn recv(&self, bytes: &[u8]) {
        self.channel.send_bytes(bytes, None).await
    }

    /// socket recv
    #[inline]
    pub async fn recv_addr(&self, bytes: &[u8], endpoint: IpEndpoint) {
        self.channel.send_bytes(bytes, Some(endpoint)).await
    }

    /// read data
    pub async fn read(&self, socket_msg: &mut SocketMsg<N>) {
        let mut ch = self.channel.channel.lock().await;
        let mut recv = ch.split().1;
        let msg = recv.receive().await;
        socket_msg.bytes.copy_from_slice(&msg.bytes);
        socket_msg.len = msg.len;
        socket_msg.callback_enum = msg.callback_enum;
        recv.receive_done();
    }

    /// read data and addr
    pub async fn read_addr(&self, socket_msg: &mut SocketMsg<N>) {
        let mut ch = self.channel.channel.lock().await;
        let mut recv = ch.split().1;
        let msg = recv.receive().await;
        socket_msg.bytes.copy_from_slice(&msg.bytes);
        socket_msg.len = msg.len;
        socket_msg.callback_enum = msg.callback_enum;
        socket_msg.endpoint = msg.endpoint;
        recv.receive_done();
    }
}
