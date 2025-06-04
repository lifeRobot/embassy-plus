use embassy_net::IpEndpoint;
use crate::channel::WriteChannel;
use crate::err::SocketErr;

/// tcp server回调
pub trait TcpServerCallBack: Copy {
    /// 有连接进来时回调此
    async fn conn<const CN: usize>(&self, endpoint: IpEndpoint, wch: &WriteChannel<'_, CN>);

    /// 连接断开后的回调
    async fn dis_conn(&self, endpoint: IpEndpoint);

    /// 数据读取
    async fn recv<const CN: usize>(&self, endpoint: IpEndpoint, buf: &[u8], wch: &WriteChannel<'_, CN>);

    /// socket err will call this<br />
    /// only error notification will be made, no blocking and exit will be made<br />
    /// please do not use endless loops in this method
    async fn err(&self, err: SocketErr);
}