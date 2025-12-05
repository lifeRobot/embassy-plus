/// pcf 8574 pin
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pcf8574Pin { PIN0, PIN1, PIN2, PIN3, PIN4, PIN5, PIN6, PIN7 }

/// custom method
impl Pcf8574Pin {
    /// convert pin data to bytes
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::PIN0 => 0,
            Self::PIN1 => 1,
            Self::PIN2 => 2,
            Self::PIN3 => 3,
            Self::PIN4 => 4,
            Self::PIN5 => 5,
            Self::PIN6 => 6,
            Self::PIN7 => 7,
        }
    }

    /// generate pin data in bytes
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            1 => Self::PIN1,
            2 => Self::PIN2,
            3 => Self::PIN3,
            4 => Self::PIN4,
            5 => Self::PIN5,
            6 => Self::PIN6,
            7 => Self::PIN7,
            _ => Self::PIN0,
        }
    }
}
