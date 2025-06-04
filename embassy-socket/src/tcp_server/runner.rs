use embassy_net::{IpEndpoint, Stack};
use embassy_net::tcp::State;
use embassy_time::{Duration, Timer};
use crate::channel::WriteChannel;
use crate::connection::socket_state::SocketState;
use crate::connection::TcpConnection;
use crate::err::{SocketErr, SocketResult};
use crate::tcp_server::callback::TcpServerCallBack;

/// single tcp server runner
#[derive(Copy, Clone)]
pub struct TcpServerRunner<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, CB: TcpServerCallBack> {
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
    /// tcp callback
    pub cb: &'d CB,
}

/// custom method
impl<'d, const N: usize, const TX_SZ: usize, const RX_SZ: usize, const BUF_SIZE: usize, CB: TcpServerCallBack> TcpServerRunner<'d, N, TX_SZ, RX_SZ, BUF_SIZE, CB> {
    /// create one runner
    #[inline]
    pub fn new(stack: Stack<'d>, state: &'d SocketState<N, TX_SZ, RX_SZ, BUF_SIZE>, port: u16, cb: &'d CB) -> Self {
        Self { stack, state, port, socket_timeout: None, read_timeout: Duration::from_millis(100), cb }
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
    pub async fn run<const CN: usize>(&self, wch: &WriteChannel<'_, CN>) {
        loop {
            if let Err(e) = self.run_logic(wch).await {
                self.cb.err(e).await;
            }
        }
    }

    /// run logic
    async fn run_logic<const CN: usize>(&self, wch: &WriteChannel<'_, CN>) -> SocketResult<()> {
        let mut conn = self.try_accept().await?;
        let endpoint = conn.socket.remote_endpoint().ok_or_else(SocketErr::no_route)?;

        wch.enable().await;
        self.cb.conn(endpoint, wch).await;
        while !self.read_logic(&mut conn, endpoint, wch).await {}
        wch.disable().await;
        self.cb.dis_conn(endpoint).await;
        Ok(())
    }

    /// read logic
    async fn read_logic<const CN: usize>(&self, conn: &mut TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>, endpoint: IpEndpoint, wch: &WriteChannel<'_, CN>) -> bool {
        if !conn.socket.can_recv() {
            if let Err(e) = self.write_logic(conn, wch).await { self.cb.err(e).await; }
            return matches!(conn.socket.state(), State::CloseWait|State::Closed);
        }

        match conn.try_read().await {
            Ok(bytes) => {
                self.cb.recv(endpoint, bytes, wch).await;
                false
            }
            Err(e) => {
                self.cb.err(e.into()).await;
                true
            }
        }
    }

    /// write logic
    async fn write_logic<const CN: usize>(
        &self,
        conn: &mut TcpConnection<'d, N, TX_SZ, RX_SZ, BUF_SIZE>,
        wch: &WriteChannel<'_, CN>) -> SocketResult<()> {
        if wch.is_empty().await {
            Timer::after(self.read_timeout).await;
            return Ok(());
        }

        wch.try_tcp_write(conn).await?;
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
