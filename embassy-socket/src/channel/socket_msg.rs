use embassy_net::{IpAddress, IpEndpoint};
use crate::channel::callback_enum::CallbackEnum;

/// socket msg
#[derive(Copy, Clone)]
pub struct SocketMsg<const N: usize> {
    /// send cache bytes
    pub(crate) bytes: [u8; N],
    /// real send bytes len
    pub(crate) len: usize,
    /// read channel logic enum
    pub callback_enum: CallbackEnum,
    /// ip addr, tcp server need this attribute<br />
    /// default is 0.0.0.0:0, if this happens, please consider it invalid ip
    pub endpoint: IpEndpoint,
}

/// support default
impl<const N: usize> Default for SocketMsg<N> {
    #[inline]
    fn default() -> Self {
        Self::new([0; N], 0)
    }
}

/// custom method
impl<const N: usize> SocketMsg<N> {
    /// create socket msg
    #[inline]
    pub const fn new(bytes: [u8; N], len: usize) -> Self {
        Self { bytes, len, callback_enum: CallbackEnum::Disconnect, endpoint: IpEndpoint::new(IpAddress::v4(0, 0, 0, 0), 0) }
    }

    /// get real bytes data
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
