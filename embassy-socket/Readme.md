### abstract

embassy-socket is tcp client/server callback by embassy-plus and embassy-ent

### support now

- tcp client &#10004;
- tcp server &#10004;
- more support comming soon

### example

build.rs file (applicable to rp2040):   
(build.rs is necessary, otherwise it may result in inability to burn)

```rust
//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
```

Cargo.toml file :

```toml
embassy-executor = { version = "0.7.0", features = ["arch-cortex-m", "executor-thread"] }
embassy-rp-plus = { version = "0.1.1", features = ["rp2040", "usb_log"] }
embassy-socket = { version = "0.1.2" }
static_cell = "2.1.0"
portable-atomic = { version = "1.11.0", features = ["critical-section"] }
rand_core = { version = "0.6.4", default-features = false }
embassy-net-wiznet = "0.2.0"
embedded-hal-bus = { version = "0.3.0", features = ["async"] }

cortex-m-rt = "0.7.5"
defmt-rtt = "1.0.0"
panic-probe = "1.0.0"
```

<details open>
<summary>tcp client example</summary>

main.rs file:

```rust
#![no_std]
#![no_main]

use core::net::Ipv4Addr;
use embassy_executor::Spawner;
use embassy_net_wiznet::chip::W5500;
use embassy_net_wiznet::{Device, Runner, State};
use embassy_rp_plus::{embassy_rp, log};
use embassy_rp_plus::embassy_rp::clocks::RoscRng;
use embassy_rp_plus::embassy_rp::gpio::Output;
use embassy_rp_plus::embassy_rp::peripherals::SPI0;
use embassy_rp_plus::embassy_rp::spi;
use embassy_rp_plus::embassy_rp::spi::{Async, Spi};
use embassy_rp_plus::traits::gpio::GpioTrait;
use embassy_rp_plus::traits::usb::usb_log::UsbLogTrait;
use embassy_socket::channel::socket_msg::SocketMsg;
use embassy_socket::channel::WriteChannel;
use embassy_socket::connection::socket_state::SocketState;
use embassy_socket::embassy_net;
use embassy_socket::embassy_net::{Ipv4Cidr, StackResources, StaticConfigV4};
use embassy_socket::embassy_time::{Delay, Timer};
use embassy_socket::err::SocketErr;
use embassy_socket::tcp_client::callback::TcpClientCallBack;
use embassy_socket::tcp_client::TcpClient;
use embassy_socket::wait::TimeOutWait;
use embedded_hal_bus::spi::ExclusiveDevice;
use rand_core::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/// re type
type W5500Runner = Runner<'static, W5500, ExclusiveDevice<Spi<'static, SPI0, Async>, Output<'static>, Delay>, TimeOutWait, Output<'static>>;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // start usb log, y can use log::info! and etc to write log to usb serial
    spawner.spawn(p.USB.log_builder_default().builder()).unwrap();

    // define global attributes
    static STATE: StaticCell<State<8, 8>> = StaticCell::new();
    static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    static SOCKET_STATE: StaticCell<SocketState<1, 1024, 1024, 1024>> = StaticCell::new();
    static MSG_STATIC: StaticCell<[SocketMsg<1024>; 10]> = StaticCell::new();
    static WRITE_CHANNEL: StaticCell<WriteChannel<'static, 1024>> = StaticCell::new();
    let state = STATE.init(State::new());
    let stack_resources = STACK_RESOURCES.init(StackResources::new());
    let socket_state = SOCKET_STATE.init(SocketState::new());
    let msg_static = MSG_STATIC.init([SocketMsg::new([0; 1024], 0); 10]);
    let write_channel = WRITE_CHANNEL.init(WriteChannel::new(msg_static));

    // create device
    let spi = Spi::new(p.SPI0, p.PIN_2, p.PIN_3, p.PIN_4, p.DMA_CH0, p.DMA_CH1, spi::Config::default());
    let cs = p.PIN_5.output();
    // int pin, if there is no interrupt pin, temporarily use TimeOutWait instead
    let int = TimeOutWait::default();
    let reset = p.PIN_10.output();
    let mac_addr = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
    let ip = Ipv4Addr::new(192, 168, 0, 119);

    // create stack
    let device = ExclusiveDevice::new(spi, cs, Delay).unwrap();
    let (device, runner) = embassy_net_wiznet::new(mac_addr, state, device, int, reset).await.unwrap();
    spawner.spawn(ethernet_task(runner)).unwrap();
    let address = Ipv4Cidr::from_netmask(ip, Ipv4Addr::new(255, 255, 255, 0)).unwrap();
    let ip = embassy_net::Config::ipv4_static(StaticConfigV4 { address, gateway: None, dns_servers: Default::default() });
    let (stack, runner) = embassy_net::new(device, ip, stack_resources, RoscRng.next_u64());
    spawner.spawn(net_task(runner)).unwrap();

    // create tcp client
    let cb = CB { wch: write_channel };
    let tcp_client = TcpClient::new(stack, Ipv4Addr::new(192, 168, 0, 44), 1234, cb, socket_state);
    spawner.spawn(tcp_client_run(tcp_client, write_channel)).unwrap();

    loop {
        write_channel.send_str("hello world").await;
        log::info!("hello world");
        Timer::after_secs(3).await;
    }
}

#[embassy_executor::task]
async fn tcp_client_run(tcp_client: TcpClient<'static, 1, 1024, 1024, 1024, CB>, wch: &'static WriteChannel<'static, 1024>) {
    tcp_client.run(wch).await;
}

/// run eth task
#[embassy_executor::task]
async fn ethernet_task(runner: W5500Runner) -> ! {
    runner.run().await
}

/// run net task
#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device<'static>>) -> ! {
    runner.run().await
}


/// tcp client callback
struct CB {
    pub wch: &'static WriteChannel<'static, 1024>,
}

/// tcp client callback business
impl TcpClientCallBack for CB {
    async fn conn(&self) {
        log::info!("conn");
        self.wch.send_str("conn").await;
    }

    async fn dis_conn(&self) {
        log::info!("dis conn");
    }

    async fn recv(&self, buf: &[u8]) {
        log::info!("conn buf is {buf:?}");
        self.wch.send_bytes(buf).await;
    }

    async fn err(&self, err: SocketErr) {
        log::info!("socket error: {err:?}");
    }
}
```

</details>

<details>
<summary>tcp server example</summary>

```rust
#![no_std]
#![no_main]

use core::net::Ipv4Addr;
use embassy_executor::Spawner;
use embassy_net_wiznet::chip::W5500;
use embassy_net_wiznet::{Device, Runner, State};
use embassy_rp_plus::{embassy_rp, log};
use embassy_rp_plus::embassy_rp::clocks::RoscRng;
use embassy_rp_plus::embassy_rp::gpio::Output;
use embassy_rp_plus::embassy_rp::peripherals::SPI0;
use embassy_rp_plus::embassy_rp::spi;
use embassy_rp_plus::embassy_rp::spi::{Async, Spi};
use embassy_rp_plus::traits::gpio::GpioTrait;
use embassy_rp_plus::traits::usb::usb_log::UsbLogTrait;
use embassy_socket::channel::socket_msg::SocketMsg;
use embassy_socket::channel::WriteChannel;
use embassy_socket::connection::socket_state::SocketState;
use embassy_socket::embassy_net;
use embassy_socket::embassy_net::{IpEndpoint, Ipv4Cidr, StackResources, StaticConfigV4};
use embassy_socket::embassy_time::{Delay, Timer};
use embassy_socket::err::SocketErr;
use embassy_socket::tcp_server::callback::TcpServerCallBack;
use embassy_socket::tcp_server::runner::TcpServerRunner;
use embassy_socket::tcp_server::TcpServer;
use embassy_socket::wait::TimeOutWait;
use embedded_hal_bus::spi::ExclusiveDevice;
use rand_core::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/// re type
type W5500Runner = Runner<'static, W5500, ExclusiveDevice<Spi<'static, SPI0, Async>, Output<'static>, Delay>, TimeOutWait, Output<'static>>;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // start usb log, y can use log::info! and etc to write log to usb serial
    spawner.spawn(p.USB.log_builder_default().builder()).unwrap();

    // define global attributes
    static STATE: StaticCell<State<8, 8>> = StaticCell::new();
    static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    static SOCKET_STATE: StaticCell<SocketState<3, 1024, 1024, 1024>> = StaticCell::new();
    // each connection is an independent channel, and here is the difference from tcp client
    // be careful not to listen on too many connections, pay attention to memory usage
    static MSG_STATIC: StaticCell<[[SocketMsg<1024>; 10]; 3]> = StaticCell::new();
    static WRITE_CHANNEL: StaticCell<[WriteChannel<'static, 1024>; 3]> = StaticCell::new();
    let state = STATE.init(State::new());
    let stack_resources = STACK_RESOURCES.init(StackResources::new());
    let socket_state = SOCKET_STATE.init(SocketState::new());
    // each connection is an independent channel, and here is the difference from tcp client
    // be careful not to listen on too many connections, pay attention to memory usage
    let [msg_static_1, msg_static_2, msg_static_3] = MSG_STATIC.init([[SocketMsg::new([0; 1024], 0); 10]; 3]);
    let write_channel = WRITE_CHANNEL.init([WriteChannel::new(msg_static_1), WriteChannel::new(msg_static_2), WriteChannel::new(msg_static_3)]);

    // create device
    let spi = Spi::new(p.SPI0, p.PIN_2, p.PIN_3, p.PIN_4, p.DMA_CH0, p.DMA_CH1, spi::Config::default());
    let cs = p.PIN_5.output();
    // int pin, if there is no interrupt pin, temporarily use TimeOutWait instead
    let int = TimeOutWait::default();
    let reset = p.PIN_10.output();
    let mac_addr = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
    let ip = Ipv4Addr::new(192, 168, 0, 119);

    // create stack
    let device = ExclusiveDevice::new(spi, cs, Delay).unwrap();
    let (device, runner) = embassy_net_wiznet::new(mac_addr, state, device, int, reset).await.unwrap();
    spawner.spawn(ethernet_task(runner)).unwrap();
    let address = Ipv4Cidr::from_netmask(ip, Ipv4Addr::new(255, 255, 255, 0)).unwrap();
    let ip = embassy_net::Config::ipv4_static(StaticConfigV4 { address, gateway: None, dns_servers: Default::default() });
    let (stack, runner) = embassy_net::new(device, ip, stack_resources, RoscRng.next_u64());
    spawner.spawn(net_task(runner)).unwrap();

    // create tcp server
    let tcp_server = TcpServer::new(stack, socket_state, 8080, &CB);
    for wch in write_channel.iter() {
        spawner.spawn(tcp_server_run(tcp_server.create(), wch)).unwrap();
    }

    loop {
        for wch in write_channel.iter() {
            wch.send_str("hello world").await;
        }
        log::info!("hello world");
        Timer::after_secs(3).await;
    }
}

/// listen on up to 3 connections
#[embassy_executor::task(pool_size = 3)]
async fn tcp_server_run(tcp_server: TcpServerRunner<'static, 3, 1024, 1024, 1024, CB>, wch: &'static WriteChannel<'static, 1024>) {
    tcp_server.run(wch, &mut ()).await;
}

/// run eth task
#[embassy_executor::task]
async fn ethernet_task(runner: W5500Runner) -> ! {
    runner.run().await
}

/// run net task
#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device<'static>>) -> ! {
    runner.run().await
}


/// tcp server callback
#[derive(Copy, Clone)]
struct CB;

/// tcp client callback business
impl TcpServerCallBack for CB {
    type T = ();

    async fn conn<const CN: usize>(&self, endpoint: IpEndpoint, wch: &WriteChannel<'_, CN>, _t: &mut ()) {
        log::info!("conn, endpoint: {endpoint:?}");
        wch.send_str("conn").await;
    }

    async fn dis_conn(&self, endpoint: IpEndpoint, _t: &mut ()) {
        log::info!("dis conn, endpoint: {endpoint:?}");
    }

    async fn recv<const CN: usize>(&self, endpoint: IpEndpoint, buf: &[u8], wch: &WriteChannel<'_, CN>, _t: &mut ()) {
        log::info!("endpoint[{endpoint:?}] recv buf is {buf:?}");
        wch.send_bytes(buf).await;
    }

    async fn err(&self, err: SocketErr, _t: &mut ()) {
        log::info!("socket error: {err:?}");
    }
}
```

</details>