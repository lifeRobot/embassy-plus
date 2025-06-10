use crate::err::SocketErr;

/// callback enum
#[derive(Copy, Clone)]
pub enum CallbackEnum {
    /// socket connection
    Conn,
    /// socket dis connection
    Disconnect,
    /// socket recv data
    Recv,
    /// socket error
    Err(SocketErr),
}

/// support default
impl Default for CallbackEnum {
    #[inline]
    fn default() -> Self {
        Self::Disconnect
    }
}
