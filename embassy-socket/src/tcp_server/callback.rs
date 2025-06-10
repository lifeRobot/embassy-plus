use embassy_net::IpEndpoint;
use crate::channel::write_channel::WriteChannel;
use crate::err::SocketErr;

/// tcp server callback
pub trait TcpServerCallBack {
    /// data processed independently on a single connection, ensure data atomicity for a single connection
    type T;

    /// connection success call this
    async fn conn<const CN: usize>(&self, endpoint: IpEndpoint, wch: &WriteChannel<'_, CN>, t: &mut Self::T);

    /// connection lost call this
    async fn dis_conn(&self, endpoint: IpEndpoint, t: &mut Self::T);

    /// recv tcp client data call this
    async fn recv<const CN: usize>(&self, endpoint: IpEndpoint, buf: &[u8], wch: &WriteChannel<'_, CN>, t: &mut Self::T);

    /// socket err will call this<br />
    /// only error notification will be made, no blocking and exit will be made<br />
    /// please do not use endless loops in this method
    async fn err(&self, err: SocketErr, t: &mut Self::T);
}