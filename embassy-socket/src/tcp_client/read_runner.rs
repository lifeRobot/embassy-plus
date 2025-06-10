use core::net::Ipv4Addr;
use embassy_net::Stack;
use embassy_net::tcp::State;
use embassy_time::{Duration, Timer};
use crate::channel::read_channel::ReadChannel;
use crate::channel::write_channel::WriteChannel;
use crate::connection::socket_state::SocketState;
use crate::connection::TcpConnection;
use crate::err::SocketResult;

/// tcp client read runner
pub struct ReadRunner<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, const RC_SZ: usize> {
    /// tcp stack
    stack: Stack<'d>,
    /// socket state, memory pool
    state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
    /// socket timeout
    socket_timeout: Option<Duration>,
    /// read data timeout, default is 100 milliseconds
    read_timeout: Duration,
    /// tcp client connection ip
    ip: Ipv4Addr,
    /// tcp client connection port
    port: u16,
    /// channel
    channel: &'d ReadChannel<'d, RC_SZ>,
}

/// custom method
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, const RC_SZ: usize>
ReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ> {
    /// create tcp client read runner
    #[inline]
    pub fn new(
        stack: Stack<'d>,
        ip: Ipv4Addr,
        port: u16,
        channel: &'d ReadChannel<'d, RC_SZ>,
        state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>) -> Self {
        Self { stack, state, socket_timeout: None, read_timeout: Duration::from_millis(100), ip, port, channel }
    }

    /// set socket timeout<br />
    /// connection timeout and etc. <br />
    /// tt is recommended not to set or set to None
    #[inline]
    pub fn socket_timeout(&mut self, timeout: Option<Duration>) {
        self.socket_timeout = timeout;
    }

    /// set read timeout
    #[inline]
    pub fn read_timeout(&mut self, timeout: Duration) {
        self.read_timeout = timeout;
    }

    /// run tcp client<br />
    /// calling this method causes tcp to maintain a long connection and send data asynchronously over WriteChannel
    pub async fn run<const CH_N: usize>(&self, wch: &WriteChannel<'_, CH_N>) {
        loop {
            self.run_logic(wch).await;
            self.channel.dis_conn().await;
        }
    }

    /// run logic
    async fn run_logic<const CH_N: usize>(&self, wch: &WriteChannel<'_, CH_N>) {
        // wait stack link and config up
        self.stack.wait_link_up().await;
        self.stack.wait_config_up().await;

        let mut conn = match self.try_conn().await {
            Ok(conn) => conn,
            Err(e) => {
                self.channel.err(e).await;
                return;
            }
        };

        wch.enable().await;
        self.channel.conn().await;
        while !self.read_logic(&mut conn, wch).await {}
        wch.disable().await;
    }

    /// read tcp data logic
    async fn read_logic<const CH_N: usize>(
        &self,
        conn: &mut TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>,
        wch: &WriteChannel<'_, CH_N>) -> bool {
        if !conn.socket.can_recv() {
            if let Err(e) = self.write_logic(conn, wch).await { self.channel.err(e).await }
            return matches!(conn.socket.state(), State::CloseWait|State::Closed);
        }

        match conn.try_read().await {
            Ok(bytes) => {
                // if channel is full, ignore data
                if self.channel.is_full().await { return false; }
                self.channel.recv(bytes).await;
                false
            }
            Err(e) => {
                self.channel.err(e.into()).await;
                true
            }
        }
    }

    /// write logic
    async fn write_logic<const CH_N: usize>(
        &self,
        conn: &mut TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>,
        wch: &WriteChannel<'_, CH_N>) -> SocketResult<()> {
        // if channel is empty, just sleep and return let continue
        if wch.is_empty().await {
            Timer::after(self.read_timeout).await;
            return Ok(());
        }

        wch.try_tcp_write(conn).await?;
        Ok(())
    }

    /// try connection
    async fn try_conn(&self) -> SocketResult<TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>> {
        let mut socket = TcpConnection::new(self.stack, self.state)?;
        socket.socket.set_timeout(self.socket_timeout);

        socket.socket.connect((self.ip, self.port)).await?;
        Ok(socket)
    }
}
