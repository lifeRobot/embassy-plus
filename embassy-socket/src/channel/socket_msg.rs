/// socket msg
#[derive(Copy, Clone)]
pub struct SocketMsg<const N: usize> {
    /// send cache bytes
    pub(crate) bytes: [u8; N],
    /// real send bytes len
    pub(crate) len: usize,
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
        Self { bytes, len }
    }

    /// get real bytes data
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
