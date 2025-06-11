use embassy_net::{IpEndpoint, Stack};
use embassy_net::tcp::State;
use embassy_time::{Duration, Timer};
use crate::channel::SocketChannel;
use crate::connection::socket_state::SocketState;
use crate::connection::TcpConnection;
use crate::err::{SocketErr, SocketResult};

/// tcp server read runner
pub struct ReadRunner<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, const RC_SZ: usize, const WC_SZ: usize> {
    /// tcp stack
    stack: Stack<'d>,
    /// socket state, memory pool
    state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>,
    /// listener port
    port: u16,
    /// socket timeout
    socket_timeout: Option<Duration>,
    /// read data timeout, default is 100 milliseconds
    read_timeout: Duration,
    /// socket channel
    socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>,
}

/// custom method
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, const RC_SZ: usize, const WC_SZ: usize> ReadRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, RC_SZ, WC_SZ> {
    /// create one read runner
    #[inline]
    pub fn new(stack: Stack<'d>, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, port: u16, socket_channel: &'d SocketChannel<'d, RC_SZ, WC_SZ>) -> Self {
        Self { stack, state, port, socket_timeout: None, read_timeout: Duration::from_millis(100), socket_channel }
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

    /// run tcp server
    pub async fn run(&self) {
        loop {
            if let Err(e) = self.run_logic().await {
                self.socket_channel.read_channel.err(e).await;
            }
        }
    }

    /// run logic
    async fn run_logic(&self) -> SocketResult<()> {
        let mut conn = self.try_accept().await?;
        let endpoint = conn.socket.remote_endpoint().ok_or_else(SocketErr::no_route)?;

        self.socket_channel.write_channel.enable().await;
        self.socket_channel.read_channel.conn_addr(endpoint).await;
        while !self.read_logic(&mut conn, endpoint).await {}
        self.socket_channel.write_channel.disable().await;
        self.socket_channel.read_channel.dis_conn_addr(endpoint).await;
        Ok(())
    }

    /// read logic
    async fn read_logic(&self, conn: &mut TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>, endpoint: IpEndpoint) -> bool {
        if !conn.socket.can_recv() {
            if let Err(e) = self.write_logic(conn).await { self.socket_channel.read_channel.err(e).await; }
            return matches!(conn.socket.state(), State::CloseWait|State::Closed);
        }

        match conn.try_read().await {
            Ok(bytes) => {
                self.socket_channel.read_channel.recv_addr(bytes, endpoint).await;
                false
            }
            Err(e) => {
                self.socket_channel.read_channel.err(e.into()).await;
                true
            }
        }
    }

    /// write logic
    async fn write_logic(&self, conn: &mut TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>) -> SocketResult<()> {
        if self.socket_channel.write_channel.is_empty().await {
            Timer::after(self.read_timeout).await;
            return Ok(());
        }

        self.socket_channel.write_channel.try_tcp_write(conn).await?;
        Ok(())
    }

    /// try accept connection
    async fn try_accept(&self) -> SocketResult<TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>> {
        self.stack.wait_link_up().await;
        self.stack.wait_config_up().await;

        let mut conn = TcpConnection::new(self.stack, self.state)?;
        conn.socket.set_timeout(self.socket_timeout);
        conn.socket.accept(self.port).await?;
        Ok(conn)
    }
}
