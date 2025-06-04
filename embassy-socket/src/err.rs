use embassy_net::tcp;
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
    TryReceiveError(TryReceiveError),
    /// tcp accept error
    AcceptError(tcp::AcceptError),
}

/// custom method
impl SocketErr {
    /// not route<br />
    /// more see [tcp::ConnectError::NoRoute]
    #[inline]
    pub fn no_route() -> Self {
        Self::ConnectError(tcp::ConnectError::NoRoute)
    }
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
    #[inline]
    fn from(value: tcp::ConnectError) -> Self {
        Self::ConnectError(value)
    }
}

/// support TryReceiveError to socket err
impl From<TryReceiveError> for SocketErr {
    #[inline]
    fn from(value: TryReceiveError) -> Self {
        Self::TryReceiveError(value)
    }
}

/// support accept error to socket err
impl From<tcp::AcceptError> for SocketErr {
    #[inline]
    fn from(value: tcp::AcceptError) -> Self {
        Self::AcceptError(value)
    }
}
