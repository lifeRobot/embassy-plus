pub enum Pcf8575Pin { PIN0, PIN1, PIN2, PIN3, PIN4, PIN5, PIN6, PIN7, PIN8, PIN9, PIN10, PIN11, PIN12, PIN13, PIN14, PIN15 }

/// custom method
impl Pcf8575Pin {
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
}
