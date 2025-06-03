use embassy_rp::gpio::Level;
use embassy_rp::i2c::{Instance, Mode};
use crate::i2c_lock::I2cLock;
use crate::pcf8575::pcf8575_pin::Pcf8575Pin;

pub mod pcf8575_pin;

/// pcf8575
pub struct PCF8575<'a, T: Instance, M: Mode> {
    /// i2c lock
    i2c: &'a I2cLock<'a, T, M>,
    /// i2c address
    address: u8,
    /// read buf
    write_buf: [u8; 2],
    /// read buf
    pub read_buf: [u8; 2],
}

/// custom method
impl<'a, T: Instance, M: Mode> PCF8575<'a, T, M> {
    /// create pcf8575
    #[inline]
    pub const fn new(i2c: &'a I2cLock<'a, T, M>, address: u8) -> Self {
        Self { i2c, address, write_buf: [0; 2], read_buf: [0; 2] }
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
        u16::from_le_bytes(self.read_buf)
    }

    /// read pin level
    #[inline]
    pub async fn read_pin(&mut self, pin: Pcf8575Pin) -> Level {
        ((self.read_gpio().await >> pin.to_u8() & 0x1) == 1).into()
    }

    /// write all gpio
    pub async fn write_gpio(&mut self, gpio: u16) {
        self.write_buf = gpio.to_be_bytes();
        self.i2c.blocking_write(self.address, &self.write_buf).await.ok();
    }

    /// write one pin io
    pub async fn write_pin(&mut self, pin: Pcf8575Pin, val: Level) {
        let bytes = u16::from_be_bytes(self.write_buf);
        let p = 1 << pin.to_u8();
        let bytes = match val {
            Level::High => { bytes | p }
            Level::Low => { bytes & !p }
        };

        self.write_gpio(bytes).await;
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
        let mut bytes = u16::from_be_bytes(self.write_buf);
        for (pin, level) in pins {
            let p = 1 << pin.to_u8();
            match level {
                Level::High => { bytes |= p }
                Level::Low => { bytes &= !p }
            }
        }

        self.write_gpio(bytes).await;
    }

    /// write pins to low
    pub async fn write_pins_low(&mut self, pins: &[Pcf8575Pin]) {
        let mut bytes = u16::from_be_bytes(self.write_buf);
        for pin in pins {
            bytes &= !(1 << pin.to_u8());
        }

        self.write_gpio(bytes).await;
    }

    /// write pins to high
    pub async fn write_pins_high(&mut self, pins: &[Pcf8575Pin]) {
        let mut bytes = u16::from_be_bytes(self.write_buf);
        for pin in pins {
            bytes |= 1 << pin.to_u8();
        }

        self.write_gpio(bytes).await;
    }
}
