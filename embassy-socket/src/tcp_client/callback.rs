use crate::err::SocketErr;

/// tcp client callback
pub trait TcpClientCallBack {
    /// connection success call this
    async fn conn(&mut self);

    /// connection lost call this
    async fn dis_conn(&mut self);

    /// recv tcp client data call this
    async fn recv(&mut self, buf: &[u8]);

    /// socket err will call this<br />
    /// only error notification will be made, no blocking and exit will be made<br />
    /// please do not use endless loops in this method
    async fn err(&mut self, err: SocketErr);
}