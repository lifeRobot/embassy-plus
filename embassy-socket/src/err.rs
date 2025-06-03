use embassy_net::tcp;
use embassy_net::tcp::ConnectError;
use embassy_sync::channel::TryReceiveError;

/// socket result
pub type SocketResult<T> = Result<T, SocketErr>;

/// socket error
#[derive(Debug)]
pub enum SocketErr {
    /// tcp error
    TcpError(tcp::Error),
    /// tcp connect error
    ConnectError(tcp::ConnectError),
    /// try receive error
    TryReceiveError(TryReceiveError)
}

/// support tcp::error to socket err
impl From<tcp::Error> for SocketErr {
    #[inline]
    fn from(value: tcp::Error) -> Self {
        Self::TcpError(value)
    }
}

/// support tcp::connectError to socket err
impl From<tcp::ConnectError> for SocketErr {
    fn from(value: ConnectError) -> Self {
        Self::ConnectError(value)
    }
}

/// support TryReceiveError to socket err
impl From<TryReceiveError> for SocketErr {
    fn from(value: TryReceiveError) -> Self {
        Self::TryReceiveError(value)
    }
}
