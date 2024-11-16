pub struct Color(u32);

impl Color {
    pub const RED: Self = Self(0xFFFF0000);
    pub const GREEN: Self = Self(0xFF00FF00);
    pub const BLUE: Self = Self(0xFF0000FF);
    pub const BLACK: Self = Self(0xFF000000);
    pub const WHITE: Self = Self(0xFFFFFFFF);

    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Color {
        Self((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32))
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Self::from_argb(0xFF, r, g, b)
    }

    pub const fn from_gray(g: u8) -> Color {
        Self::from_argb(0xFF, g, g, g)
    }

    pub const fn inner(&self) -> u32 {
        self.0
    }
}
