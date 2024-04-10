// Blatanly copied from macroquad

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::new(r, g, b, a)
    }

    pub fn from_hex(hex: u32) -> Color {
        let bytes: [u8; 4] = hex.to_be_bytes();
        Self::from_rgba(bytes[1], bytes[2], bytes[3], 255)
    }

    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const NOT_BLACK: Color = Color { r: 18, g: 33, b: 43, a: 255 };
    pub const LIGHTGRAY: Color = Color { r: 199, g: 199, b: 199, a: 255 };
    pub const GRAY: Color = Color { r: 130, g: 130, b: 130, a: 255 };
    pub const DARKGRAY: Color = Color { r: 79, g: 79, b: 79, a: 255 };
    pub const YELLOW: Color = Color { r: 252, g: 250, b: 0, a: 255 };
    pub const GOLD: Color = Color { r: 255, g: 204, b: 0, a: 255 };
    pub const ORANGE: Color = Color { r: 255, g: 161, b: 0, a: 255 };
    pub const PINK: Color = Color { r: 255, g: 110, b: 194, a: 255 };
    pub const RED: Color = Color { r: 230, g: 41, b: 56, a: 255 };
    pub const MAROON: Color = Color { r: 191, g: 33, b: 56, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 227, b: 48, a: 255 };
    pub const LIME: Color = Color { r: 0, g: 158, b: 46, a: 255 };
    pub const DARKGREEN: Color = Color { r: 0, g: 117, b: 43, a: 255 };
    pub const SKYBLUE: Color = Color { r: 102, g: 191, b: 255, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 120, b: 242, a: 255 };
    pub const DARKBLUE: Color = Color { r: 0, g: 82, b: 171, a: 255 };
    pub const PURPLE: Color = Color { r: 199, g: 122, b: 255, a: 255 };
    pub const VIOLET: Color = Color { r: 135, g: 61, b: 191, a: 255 };
    pub const DARKPURPLE: Color = Color { r: 112, g: 31, b: 125, a: 255 };
    pub const BEIGE: Color = Color { r: 212, g: 176, b: 130, a: 255 };
    pub const BROWN: Color = Color { r: 128, g: 107, b: 79, a: 255 };
    pub const DARKBROWN: Color = Color { r: 77, g: 64, b: 46, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const BLANK: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> [u8; 4] {
        [color.r, color.g, color.b, color.a]
    }
}