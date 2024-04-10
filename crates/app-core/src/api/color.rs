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
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
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
    pub const LIGHTGRAY: Color = Color::new(199, 199, 199, 255);
    pub const GRAY: Color = Color::new(130, 130, 130, 255);
    pub const DARKGRAY: Color = Color::new(79, 79, 79, 255);
    pub const YELLOW: Color = Color::new(252, 250, 0, 255);
    pub const GOLD: Color = Color::new(255, 204, 0, 255);
    pub const ORANGE: Color = Color::new(255, 161, 0, 255);
    pub const PINK: Color = Color::new(255, 110, 194, 255);
    pub const RED: Color = Color::new(230, 41, 56, 255);
    pub const MAROON: Color = Color::new(191, 33, 56, 255);
    pub const GREEN: Color = Color::new(0, 227, 48, 255);
    pub const LIME: Color = Color::new(0, 158, 46, 255);
    pub const DARKGREEN: Color = Color::new(0, 117, 43, 255);
    pub const SKYBLUE: Color = Color::new(102, 191, 255, 255);
    pub const BLUE: Color = Color::new(0, 120, 242, 255);
    pub const DARKBLUE: Color = Color::new(0, 82, 171, 255);
    pub const PURPLE: Color = Color::new(199, 122, 255, 255);
    pub const VIOLET: Color = Color::new(135, 61, 191, 255);
    pub const DARKPURPLE: Color = Color::new(112, 31, 125, 255);
    pub const BEIGE: Color = Color::new(212, 176, 130, 255);
    pub const BROWN: Color = Color::new(128, 107, 79, 255);
    pub const DARKBROWN: Color = Color::new(77, 64, 46, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const BLANK: Color = Color::new(0, 0, 0, 0);
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255);
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> [u8; 4] {
        [color.r, color.g, color.b, color.a]
    }
}