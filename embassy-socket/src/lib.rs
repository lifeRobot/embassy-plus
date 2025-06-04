#![no_std]
#![allow(async_fn_in_trait)]

// export dependency library
pub use embassy_net;
pub use embassy_time;
pub use embassy_sync;
pub use embedded_hal;
pub use embedded_hal_async;

pub mod tcp_client;
pub mod tcp_server;
pub mod connection;
pub mod channel;
pub mod err;
pub mod wait;
pub mod socket_build;