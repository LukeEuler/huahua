extern crate image;

use image::RgbaImage;
use ops::filters;
use std::collections::HashMap;

pub fn apply_blend(name: String, foreground: RgbaImage, background: RgbaImage) -> RgbaImage {
    let mut blend_map: HashMap<String, Blend> = HashMap::new();
    blend_map.insert("color_dodge".to_string(), Blend::ColorDodge);
    blend_map.insert("darken".to_string(), Blend::Darken);
    blend_map.insert("exclusion".to_string(), Blend::Exclusion);
    blend_map.insert("lighten".to_string(), Blend::Lighten);
    blend_map.insert("multiply".to_string(), Blend::Multiply);
    blend_map.insert("over".to_string(), Blend::Over);
    blend_map.insert("overlay".to_string(), Blend::Overlay);
    blend_map.insert("screen".to_string(), Blend::Screen);
    blend_map.insert("soft_light".to_string(), Blend::SoftLight);

    let bb: Blend = blend_map.get(&name).unwrap().clone();
    match bb {
        Blend::ColorDodge => filters::blend_color_dodge(&foreground, &background),
        Blend::Darken => filters::blend_darken(&foreground, &background),
        Blend::Exclusion => filters::blend_exclusion(&foreground, &background),
        Blend::Lighten => filters::blend_lighten(&foreground, &background),
        Blend::Multiply => filters::blend_multiply(&foreground, &background),
        Blend::Over => filters::blend_over(&foreground, &background),
        Blend::Overlay => filters::blend_overlay(&foreground, &background),
        Blend::Screen => filters::blend_screen(&foreground, &background),
        Blend::SoftLight => filters::blend_soft_light(&foreground, &background),
    }
}

#[derive(Clone)]
pub enum Blend {
    ColorDodge,
    Darken,
    Exclusion,
    Lighten,
    Multiply,
    Over,
    Overlay,
    Screen,
    SoftLight,
}
