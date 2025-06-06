use core::num::TryFromIntError;
use embassy_rp_plus::embassy_rp::flash;

/// re type
pub type FlashResult<T> = Result<T, FlashError>;

/// flash error
#[derive(Debug)]
pub enum FlashError {
    /// flash error
    FlashError(flash::Error),
    /// try from error
    TryFromIntError(TryFromIntError),
}

/// support flash error into flash error
impl From<flash::Error> for FlashError {
    #[inline]
    fn from(value: flash::Error) -> Self {
        Self::FlashError(value)
    }
}

/// support try from int error to flash error
impl From<TryFromIntError> for FlashError {
    #[inline]
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}
