### abstract

embassy-utils is embassy plus collection of util

### support now

- RP &#10004;
- STM32 &#10006; coming soon
- FlashUtil &#10004;
- more util coming soon

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
embassy-executor = { version = "0.9.1", features = ["arch-cortex-m", "executor-thread"] }
embassy-rp-plus = { version = "0.1.1", features = ["rp2040", "usb_log"] }
embassy-utils = { version = "0.1.0", features = ["rp"] }
embassy-time = "0.5.0"

cortex-m-rt = "0.7.5"
defmt-rtt = "1.0.0"
panic-probe = "1.0.0"
```

FlashUtil example   
main.rs file:

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp_plus::{embassy_rp, log};
use embassy_rp_plus::traits::flash::FlashTrait;
use embassy_rp_plus::traits::usb::usb_log::UsbLogTrait;
use embassy_time::Timer;
use embassy_utils::flash::to_lock::ToFlashLock;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner.spawn(p.USB.log_builder_default().builder()).unwrap();

    // 2097152 = 2 * 1024 * 1024 = 2M
    let flash = p.FLASH.build::<2097152>(p.DMA_CH0).to_lock();
    let flash_utl = flash.build_flash_util_default();

    // read flash data
    Timer::after_secs(1).await;
    let mut buf = [0; 3];
    flash_utl.try_read(&mut buf).await.unwrap();
    log::info!("buf: {buf:0x?}");

    // erase write and read
    Timer::after_secs(1).await;
    flash_utl.try_erase_write(&[0x0A, 0x0B, 0x0C]).await.unwrap();
    flash_utl.try_read(&mut buf).await.unwrap();
    log::info!("buf: {buf:0x?}");

    // erase and read
    Timer::after_secs(1).await;
    flash_utl.try_erase(1).await.unwrap();
    flash_utl.try_read(&mut buf).await.unwrap();
    log::info!("buf: {buf:0x?}");
}
```