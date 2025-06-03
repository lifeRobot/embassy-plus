### abstract

embassy-pcf857x is pcf8574 and pcf8575 by embassy-plus

### support now

- pcf8575 &#10004;
- pcf8574 &#10006; comming soon
- RP2040 &#10004;
- STM32 and more &#10006; comming soon
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
embassy-rp-plus = { version = "0.1.0", features = ["rp2040", "usb_log"] }
embassy-pcf857x = { version = "0.1.0", features = ["rp"] }
embassy-time = { version = "0.4.0" }

cortex-m-rt = "0.7.5"
defmt-rtt = "1.0.0"
panic-probe = "1.0.0"
```

main.rs file:
```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_pcf857x::i2c_lock::to_lock::ToLock;
use embassy_pcf857x::pcf8575::pcf8575_pin::Pcf8575Pin;
use embassy_rp_plus::{embassy_rp, log};
use embassy_rp_plus::traits::i2c::i2c0::I2c0Trait;
use embassy_rp_plus::traits::usb::usb_log::UsbLogTrait;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // start usb log, y can use log::info! and etc to write log to usb serial
    spawner.spawn(p.USB.log_builder_default().builder()).unwrap();

    let i2c = p.I2C0.builder(p.PIN_21, p.PIN_20).build().to_lock();
    // default address is 0x20
    let mut pcf_8575 = i2c.build_pcf8575_default();
    // use the same i2c and customize the address, such as 0x27
    let mut pcf_8575_27 = i2c.build_pcf8575(0x27);

    // write all pins to high/low
    pcf_8575.write_all_high().await;
    pcf_8575_27.write_all_low().await;

    loop {
        pcf_8575.read_to_buf().await;
        log::info!("all bin is {:08b}{:08b}",pcf_8575.read_buf[0],pcf_8575.read_buf[1]);
        Timer::after_secs(1).await;

        // read all gpio to buf and trans to u16, use byte array in little endian
        let gpio = pcf_8575.read_gpio().await;
        // this log eq `log::info!("all bin is {:08b}{:08b}",pcf_8575.read_buf[1],pcf_8575.read_buf[0]);`
        log::info!("all bin is {gpio:016b}");
        Timer::after_secs(1).await;

        // read one pin
        let p0 = pcf_8575.read_pin(Pcf8575Pin::PIN0).await;
        log::info!("pin 0 is {p0:?}");
        Timer::after_secs(1).await;
    }
}
```
