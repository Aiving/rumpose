#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color(u32);

impl Color {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn from_argb(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self(
            (0xFF & u32::from(alpha)) << 24
                | (0xFF & u32::from(red)) << 16
                | (0xFF & u32::from(green)) << 8
                | (0xFF & u32::from(blue)),
        )
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::from_argb(255, red, green, blue)
    }

    pub fn alpha(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    pub fn red(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    pub fn green(&self) -> u8 {
        ((self.0 >> 6) & 0xFF) as u8
    }

    pub fn blue(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value.0
    }
}
