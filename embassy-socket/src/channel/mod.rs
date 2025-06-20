use crate::channel::read_channel::ReadChannel;
use crate::channel::socket_msg::SocketMsg;
use crate::channel::write_channel::WriteChannel;

pub(crate) mod lock_channel;
pub mod write_channel;
pub mod read_channel;
pub mod socket_msg;
pub mod callback_enum;

/// socket channel
pub struct SocketChannel<'d, const RC_SZ: usize, const WC_SZ: usize> {
    /// read channel
    pub read_channel: ReadChannel<'d, RC_SZ>,
    /// write channel
    pub write_channel: WriteChannel<'d, WC_SZ>,
}

/// custom method
impl<'d, const RC_SZ: usize, const WC_SZ: usize> SocketChannel<'d, RC_SZ, WC_SZ> {
    /// create socket channel
    #[inline]
    pub fn new(read_buf: &'d mut [SocketMsg<RC_SZ>], write_buf: &'d mut [SocketMsg<WC_SZ>]) -> Self {
        Self { read_channel: ReadChannel::new(read_buf), write_channel: WriteChannel::new(write_buf) }
    }
}
