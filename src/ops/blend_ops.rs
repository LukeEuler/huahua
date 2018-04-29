extern crate image;

use image::RgbaImage;
use ops::filters;
use std::collections::HashMap;

pub fn apply_blend(name: String, foreground: RgbaImage, background: RgbaImage) -> RgbaImage {
    let mut blend_map: HashMap<String, Blend> = HashMap::new();
    blend_map.insert("color_burn".to_string(), Blend::ColorBurn);
    blend_map.insert("color_dodge".to_string(), Blend::ColorDodge);
    blend_map.insert("darken".to_string(), Blend::Darken);
    blend_map.insert("difference".to_string(), Blend::Difference);
    blend_map.insert("exclusion".to_string(), Blend::Exclusion);
    blend_map.insert("lighten".to_string(), Blend::Lighten);
    blend_map.insert("linear_burn".to_string(), Blend::LinearBurn);
    blend_map.insert("linear_dodge".to_string(), Blend::LinearDodge);
    blend_map.insert("linear_light".to_string(), Blend::LinearLight);
    blend_map.insert("multiply".to_string(), Blend::Multiply);
    blend_map.insert("over".to_string(), Blend::Over);
    blend_map.insert("overlay".to_string(), Blend::Overlay);
    blend_map.insert("pin_light".to_string(), Blend::PinLight);
    blend_map.insert("screen".to_string(), Blend::Screen);
    blend_map.insert("soft_light".to_string(), Blend::SoftLight);
    blend_map.insert("vivid_light".to_string(), Blend::VividLight);

    let bb: Blend = blend_map.get(&name).unwrap().clone();
    match bb {
        Blend::ColorBurn => filters::blend_color_burn(&foreground, &background),
        Blend::ColorDodge => filters::blend_color_dodge(&foreground, &background),
        Blend::Darken => filters::blend_darken(&foreground, &background),
        Blend::Difference => filters::blend_difference(&foreground, &background),
        Blend::Exclusion => filters::blend_exclusion(&foreground, &background),
        Blend::Lighten => filters::blend_lighten(&foreground, &background),
        Blend::LinearBurn => filters::blend_linear_burn(&foreground, &background),
        Blend::LinearDodge => filters::blend_linear_dodge(&foreground, &background),
        Blend::LinearLight => filters::blend_linear_light(&foreground, &background),
        Blend::Multiply => filters::blend_multiply(&foreground, &background),
        Blend::Over => filters::blend_over(&foreground, &background),
        Blend::Overlay => filters::blend_overlay(&foreground, &background),
        Blend::PinLight => filters::blend_pin_light(&foreground, &background),
        Blend::Screen => filters::blend_screen(&foreground, &background),
        Blend::SoftLight => filters::blend_soft_light(&foreground, &background),
        Blend::VividLight => filters::blend_vivid_light(&foreground, &background),
    }
}

#[derive(Clone)]
pub enum Blend {
    ColorBurn,
    ColorDodge,
    Darken,
    Difference,
    Exclusion,
    Lighten,
    LinearBurn,
    LinearDodge,
    LinearLight,
    Multiply,
    Over,
    Overlay,
    PinLight,
    Screen,
    SoftLight,
    VividLight,
}
