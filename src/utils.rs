pub struct Vec2<T: Sized> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex(color: u32) -> Self {
        let color = color.to_be_bytes();
        Color {
            a: color[0],
            r: color[1],
            g: color[2],
            b: color[3],
        }
    }

    pub fn from(a: u8, r: u8, g: u8, b: u8) -> Self {
        Color { a, r, g, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { a: 255, r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        u32::from_be_bytes([self.a, self.r, self.g, self.b])
    }
}
