use egui::Rgba;

use crate::cg::Float;

macro_rules! hex_color {
    ($s:literal) => {{
        let array = color_hex::color_from_hex!($s);
        if array.len() == 3 {
            egui::Rgba::from_rgba_premultiplied(
                array[0] as f32 / 255.0,
                array[1] as f32 / 255.0,
                array[2] as f32 / 255.0,
                1.0,
            )
        } else {
            #[allow(unconditional_panic)]
            egui::Rgba::from_rgba_premultiplied(
                array[0] as f32 / 255.0,
                array[1] as f32 / 255.0,
                array[2] as f32 / 255.0,
                array[3] as f32 / 255.0,
            )
        }
    }};
}

pub trait WithAlpha {
    fn with_alpha(&self, a: Float) -> Self;
}

impl WithAlpha for Rgba {
    #[inline]
    fn with_alpha(&self, a: Float) -> Self {
        Rgba::from_rgba_premultiplied(self.r(), self.g(), self.b(), a)
    }
}

pub const RED: Rgba = hex_color!("#ff0000");
pub const ORANGE: Rgba = hex_color!("#ffa160");
pub const LIGHT_BLUE: Rgba = hex_color!("#c2fffe");
pub const WHITE: Rgba = hex_color!("#ffffff");
pub const LIGHT_GRAY: Rgba = hex_color!("#b6b6b6");
pub const GRAY: Rgba = hex_color!("#747474");
pub const DARK_GRAY: Rgba = hex_color!("#474747");
pub const BLACK: Rgba = hex_color!("#000000");
pub const TRANSPARENT: Rgba = hex_color!("#00000000");
