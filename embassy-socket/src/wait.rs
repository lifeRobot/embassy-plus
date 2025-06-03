use embassy_time::{Duration, Timer};
use embedded_hal::digital::{Error, ErrorKind, ErrorType};
use embedded_hal_async::digital::Wait;

/// timeout wait<br />
/// generally used when there are no interrupt pins
pub struct TimeOutWait {
    pub timeout: Duration,
}

/// timeout wait error, nothing error
#[derive(Debug)]
pub struct TimeoutWaitError;

/// out order error
impl Error for TimeoutWaitError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

/// custom error type
impl ErrorType for TimeOutWait { type Error = TimeoutWaitError; }

/// support default 100 millis
impl Default for TimeOutWait {
    #[inline]
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
}

/// custom method
impl TimeOutWait {
    /// custom timeout
    #[inline]
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

/// support wait
impl Wait for TimeOutWait {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Timer::after(self.timeout).await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Timer::after(self.timeout).await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Timer::after(self.timeout).await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Timer::after(self.timeout).await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Timer::after(self.timeout).await;
        Ok(())
    }
}
