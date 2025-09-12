#[derive(Debug, Copy, Clone)]
pub enum Pcf8575Pin { PIN0, PIN1, PIN2, PIN3, PIN4, PIN5, PIN6, PIN7, PIN8, PIN9, PIN10, PIN11, PIN12, PIN13, PIN14, PIN15 }

/// custom method
impl Pcf8575Pin {
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
            Self::PIN8 => 8,
            Self::PIN9 => 9,
            Self::PIN10 => 10,
            Self::PIN11 => 11,
            Self::PIN12 => 12,
            Self::PIN13 => 13,
            Self::PIN14 => 14,
            Self::PIN15 => 15,
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
            8 => Self::PIN8,
            9 => Self::PIN9,
            10 => Self::PIN10,
            11 => Self::PIN11,
            12 => Self::PIN12,
            13 => Self::PIN13,
            14 => Self::PIN14,
            15 => Self::PIN15,
            _ => Self::PIN0,
        }
    }

    /// convert pin data to index and bytes
    pub fn to_be_index_u8(&self) -> (usize, u8) {
        match self {
            Self::PIN0 => (1, 0),
            Self::PIN1 => (1, 1),
            Self::PIN2 => (1, 2),
            Self::PIN3 => (1, 3),
            Self::PIN4 => (1, 4),
            Self::PIN5 => (1, 5),
            Self::PIN6 => (1, 6),
            Self::PIN7 => (1, 7),
            Self::PIN8 => (0, 0),
            Self::PIN9 => (0, 1),
            Self::PIN10 => (0, 2),
            Self::PIN11 => (0, 3),
            Self::PIN12 => (0, 4),
            Self::PIN13 => (0, 5),
            Self::PIN14 => (0, 6),
            Self::PIN15 => (0, 7),
        }
    }

    /// convert pin data to index and bytes
    #[inline]
    pub fn to_le_index_u8(&self) -> (usize, u8) {
        let (i, p) = self.to_be_index_u8();
        (if i == 1 { 0 } else { 1 }, p)
    }

    /// convert pin data to index and bytes
    pub fn to_index_u8(&self, big_endian: bool) -> (usize, u8) {
        let (i, p) = self.to_be_index_u8();
        if big_endian {
            return (i, p);
        }
        (if i == 1 { 0 } else { 1 }, p)
    }
}
