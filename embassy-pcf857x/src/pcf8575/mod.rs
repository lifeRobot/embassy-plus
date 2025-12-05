use embassy_rp::gpio::Level;
use embassy_rp::i2c::{Async, Instance, Mode};
use crate::i2c_lock::I2cLock;
use crate::pcf8575::pcf8575_pin::Pcf8575Pin;

pub mod pcf8575_pin;

/// pcf8575
pub struct PCF8575<'a, T: Instance, M: Mode> {
    /// i2c lock
    i2c: &'a I2cLock<'a, T, M>,
    /// i2c address
    address: u8,
    /// if true, use big endian<br />
    /// if false, use little endian<br />
    /// default is big endian
    pub big_endian: bool,
    /// write buf
    write_buf: [u8; 2],
    /// read buf
    pub read_buf: [u8; 2],
}

/// custom method
impl<'a, T: Instance, M: Mode> PCF8575<'a, T, M> {
    /// create pcf8575
    #[inline]
    pub const fn new(i2c: &'a I2cLock<'a, T, M>, address: u8) -> Self {
        Self { i2c, address, big_endian: true, write_buf: [0; 2], read_buf: [0; 2] }
    }

    /// use big endian
    #[inline]
    pub const fn set_big_endian(&mut self) {
        self.big_endian = true;
    }

    /// use little endian
    #[inline]
    pub const fn set_little_endian(&mut self) {
        self.big_endian = false;
    }

    /// change big endian<br />
    /// if true, use big endian
    /// if false, use little endian
    #[inline]
    pub const fn big_endian(mut self, big_endian: bool) -> Self {
        self.big_endian = big_endian;
        self
    }

    /// create default address pcf8575, default address is 0x20
    #[inline]
    pub const fn new_default(i2c: &'a I2cLock<'a, T, M>) -> Self {
        Self::new(i2c, 0x20)
    }

    /// read to buf, see `self.read_buf`
    #[inline]
    pub async fn read_to_buf(&mut self) {
        self.i2c.blocking_read(self.address, &mut self.read_buf).await.ok();
    }

    /// read all gpio
    pub async fn read_gpio(&mut self) -> u16 {
        self.read_to_buf().await;
        if self.big_endian {
            u16::from_be_bytes(self.read_buf)
        } else {
            u16::from_le_bytes(self.read_buf)
        }
    }

    /// read pin level
    #[inline]
    pub async fn read_pin(&mut self, pin: Pcf8575Pin) -> Level {
        ((self.read_gpio().await >> pin.to_u8() & 0x1) == 1).into()
    }

    /// write all gpio
    #[inline]
    pub async fn write_gpio(&mut self, gpio: u16) {
        let buf = if self.big_endian { gpio.to_be_bytes() } else { gpio.to_le_bytes() };
        self.write_buf(buf).await;
    }

    /// write all buf<br />
    /// more see [Self::write_gpio]
    pub async fn write_buf(&mut self, buf: [u8; 2]) {
        self.write_buf = buf;
        self.re_write().await;
    }

    /// re-write all buf to i2c
    #[inline]
    pub async fn re_write(&self) {
        self.i2c.blocking_write(self.address, &self.write_buf).await.ok();
    }

    /// just set up the cache, no data will be written to i2c<br />
    /// if you need to write cached data to i2c, call[Self::re_write] or [Self::re_write_async]
    pub fn set_pin(&mut self, pin: Pcf8575Pin, level: Level) {
        let (i, p) = pin.to_index_u8(self.big_endian);
        match level {
            Level::High => { self.write_buf[i] |= 1 << p; }
            Level::Low => { self.write_buf[i] &= !(1 << p); }
        }
    }

    /// write one pin io
    #[inline]
    pub async fn write_pin(&mut self, pin: Pcf8575Pin, val: Level) {
        self.set_pin(pin, val);
        self.re_write().await;
    }

    /// write all pin to low
    pub async fn write_all_low(&mut self) {
        self.write_buf = [0x00; 2];
        self.i2c.blocking_write(self.address, &self.write_buf).await.ok();
    }

    /// write all pin to high
    pub async fn write_all_high(&mut self) {
        self.write_buf = [0xFF; 2];
        self.i2c.blocking_write(self.address, &self.write_buf).await.ok();
    }

    /// write pins level
    pub async fn write_pins(&mut self, pins: &[(Pcf8575Pin, Level)]) {
        for (pin, level) in pins {
            self.set_pin(*pin, *level);
        }

        self.re_write().await;
    }

    /// write pins to low
    pub async fn write_pins_low(&mut self, pins: &[Pcf8575Pin]) {
        for pin in pins {
            let (i, p) = pin.to_index_u8(self.big_endian);
            self.write_buf[i] &= !(1 << p);
        }

        self.re_write().await;
    }

    /// write pins to high
    pub async fn write_pins_high(&mut self, pins: &[Pcf8575Pin]) {
        for pin in pins {
            let (i, p) = pin.to_index_u8(self.big_endian);
            self.write_buf[i] |= 1 << p;
        }

        self.re_write().await;
    }
}

/// custom method
impl<'a, T: Instance> PCF8575<'a, T, Async> {
    /// read to buf, see `self.read_buf`
    #[inline]
    pub async fn read_to_buf_async(&mut self) {
        self.i2c.read_async(self.address, &mut self.read_buf).await.ok();
    }

    /// read all gpio
    pub async fn read_gpio_async(&mut self) -> u16 {
        self.read_to_buf_async().await;
        if self.big_endian {
            u16::from_be_bytes(self.read_buf)
        } else {
            u16::from_le_bytes(self.read_buf)
        }
    }

    /// read pin level
    #[inline]
    pub async fn read_pin_async(&mut self, pin: Pcf8575Pin) -> Level {
        ((self.read_gpio_async().await >> pin.to_u8() & 0x1) == 1).into()
    }

    /// write all gpio
    #[inline]
    pub async fn write_gpio_async(&mut self, gpio: u16) {
        let buf = if self.big_endian { gpio.to_be_bytes() } else { gpio.to_le_bytes() };
        self.write_buf_async(buf).await;
    }

    /// write all buf<br />
    /// more see [Self::write_gpio_async]
    pub async fn write_buf_async(&mut self, buf: [u8; 2]) {
        self.write_buf = buf;
        self.re_write_async().await;
    }

    /// re-write all buf to i2c
    #[inline]
    pub async fn re_write_async(&self) {
        self.i2c.write_async(self.address, self.write_buf).await.ok();
    }


    /// write one pin io
    #[inline]
    pub async fn write_pin_async(&mut self, pin: Pcf8575Pin, val: Level) {
        self.set_pin(pin, val);
        self.re_write_async().await;
    }

    /// write all pin to low
    pub async fn write_all_low_async(&mut self) {
        self.write_buf = [0x00; 2];
        self.i2c.write_async(self.address, self.write_buf).await.ok();
    }

    /// write all pin to high
    pub async fn write_all_high_async(&mut self) {
        self.write_buf = [0xFF; 2];
        self.i2c.write_async(self.address, self.write_buf).await.ok();
    }

    /// write pins level
    pub async fn write_pins_async(&mut self, pins: &[(Pcf8575Pin, Level)]) {
        for (pin, level) in pins {
            self.set_pin(*pin, *level);
        }

        self.re_write_async().await;
    }

    /// write pins to low
    pub async fn write_pins_low_async(&mut self, pins: &[Pcf8575Pin]) {
        for pin in pins {
            let (i, p) = pin.to_index_u8(self.big_endian);
            self.write_buf[i] &= !(1 << p);
        }

        self.re_write_async().await;
    }

    /// write pins to high
    pub async fn write_pins_high_async(&mut self, pins: &[Pcf8575Pin]) {
        for pin in pins {
            let (i, p) = pin.to_index_u8(self.big_endian);
            self.write_buf[i] |= 1 << p;
        }

        self.re_write_async().await;
    }
}
