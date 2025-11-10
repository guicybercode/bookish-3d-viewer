#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_u32(color: u32) -> Self {
        Self {
            r: ((color >> 16) & 0xFF) as u8,
            g: ((color >> 8) & 0xFF) as u8,
            b: (color & 0xFF) as u8,
            a: 255,
        }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn to_rgba_f32(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

pub struct ColorPicker {
    pub wireframe_color: Color,
    pub flat_color: Color,
    pub background_color: Color,
}

impl ColorPicker {
    pub fn new() -> Self {
        Self {
            wireframe_color: Color::from_u32(0x00FF00),
            flat_color: Color::from_u32(0xFFBF00),
            background_color: Color::from_u32(0x000000),
        }
    }

    pub fn set_wireframe_color(&mut self, color: Color) {
        self.wireframe_color = color;
    }

    pub fn set_flat_color(&mut self, color: Color) {
        self.flat_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn reset_to_defaults(&mut self) {
        self.wireframe_color = Color::from_u32(0x00FF00);
        self.flat_color = Color::from_u32(0xFFBF00);
        self.background_color = Color::from_u32(0x000000);
    }
}

