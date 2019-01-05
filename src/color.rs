use image::Rgb;

macro_rules! color {
    ($name:ident, $r:expr, $g:expr, $b:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name;
        impl From<$name> for Color {
            fn from(_: $name) -> Self {
                ($r, $g, $b).into()
            }
        }
    };
}

color!(Black, 0, 0, 0);
color!(White, 255, 255, 255);
color!(Red, 255, 0, 0);
color!(Green, 0, 255, 0);
color!(Blue, 0, 0, 255);

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub rgb: Rgb<u8>,
}

impl From<(u8, u8, u8)> for Color {
    fn from(t: (u8, u8, u8)) -> Self {
        Self {
            rgb: Rgb {
                data: [t.0, t.1, t.2],
            },
        }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        assert!(r >= 0.0 && r <= 1.0);
        assert!(g >= 0.0 && g <= 1.0);
        assert!(b >= 0.0 && b <= 1.0);
        Self {
            rgb: Rgb {
                data: [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8],
            },
        }
    }
}

impl Into<Rgb<u8>> for Color {
    fn into(self) -> Rgb<u8> {
        self.rgb
    }
}
