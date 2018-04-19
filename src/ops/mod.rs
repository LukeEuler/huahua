extern crate image;
extern crate config;
extern crate serde_json;

use image::{RgbaImage, ConvertBuffer};
use image::imageops;
use std::process;
use std::collections::HashMap;

mod filters;
mod blend;

pub fn apply(mut img: RgbaImage, value: serde_json::Value) -> RgbaImage {
    match value.as_array() {
        Some(array) => {
            for item in array {
                img = apply_object(img, item.clone())
            }
        }
        None => {
            println!("not array! {:?}", value.to_string());
            process::exit(1);
        }
    }
    img
}

pub fn apply_object(img: RgbaImage, value: serde_json::Value) -> RgbaImage {
    let map = match value.as_object() {
        Some(m) => m.clone(),
        None => {
            println!("not object! {:?}", value.to_string());
            process::exit(1);
        }
    };
    match map.get("blend") {
        Some(item) => {
            let foreground = map.get("foreground").unwrap().clone();
            let ff = apply(img.clone(), foreground).clone();
            let background = value.get("background").unwrap().clone();
            let bb = apply(img.clone(), background).clone();
            let name = item.as_str().unwrap().to_string();
            return apply_blend(name, ff, bb);
        }
        None => {}
    }

    apply_filter(img, map)
}

pub fn apply_blend(name: String, foreground: RgbaImage, background: RgbaImage) -> RgbaImage {
    let mut blend_map: HashMap<String, Blend> = HashMap::new();
    blend_map.insert("screen".to_string(), Blend::Screen);
    blend_map.insert("lighten".to_string(), Blend::Lighten);
    blend_map.insert("overlay".to_string(), Blend::Overlay);
    blend_map.insert("soft_light".to_string(), Blend::SoftLight);
    blend_map.insert("multiply".to_string(), Blend::Multiply);
    blend_map.insert("color_dodge".to_string(), Blend::ColorDodge);
    blend_map.insert("darken".to_string(), Blend::Darken);
    blend_map.insert("over".to_string(), Blend::Over);
    blend_map.insert("exclusion".to_string(), Blend::Exclusion);

    let bb: Blend = blend_map.get(&name).unwrap().clone();
    match bb {
        Blend::Screen => filters::blend_screen(&foreground, &background),
        Blend::Lighten => filters::blend_lighten(&foreground, &background),
        Blend::Overlay => filters::blend_overlay(&foreground, &background),
        Blend::SoftLight => filters::blend_soft_light(&foreground, &background),
        Blend::Multiply => filters::blend_multiply(&foreground, &background),
        Blend::ColorDodge => filters::blend_color_dodge(&foreground, &background),
        Blend::Darken => filters::blend_darken(&foreground, &background),
        Blend::Over => filters::over(&foreground, &background),
        Blend::Exclusion => filters::blend_exclusion(&foreground, &background),
    }
}

pub fn apply_filter(img: RgbaImage, value: serde_json::Map<String, serde_json::Value>) -> RgbaImage {
    let mut filter_map: HashMap<String, Filter> = HashMap::new();
    filter_map.insert("huerotate".to_string(), Filter::Huerotate);
    filter_map.insert("contrast".to_string(), Filter::Contrast);
    filter_map.insert("saturate".to_string(), Filter::Saturate);
    filter_map.insert("brighten_by_percent".to_string(), Filter::BrightenByPercent);
    filter_map.insert("opaque".to_string(), Filter::Opaque);
    filter_map.insert("fill_with_channels".to_string(), Filter::FillWithChannels);
    filter_map.insert("sepia".to_string(), Filter::Sepia);
    filter_map.insert("grayscale".to_string(), Filter::Grayscale);

    let mut out = img.clone();
    if value.len() != 1 {
        println!("len != 1");
        process::exit(1);
    }
    for (name, v) in value {
        let filter: Filter = filter_map.get(&name).unwrap().clone();
        out = judge_filter(out, filter, v.clone());
    }
    out
}

fn judge_filter(img: RgbaImage, filter: Filter, value: serde_json::Value) -> RgbaImage {
    match filter {
        Filter::Contrast => do_contrast(&img, value),
        Filter::Huerotate => do_huerotate(&img, value),
        Filter::BrightenByPercent => do_brighten_by_percent(&img, value),
        Filter::Saturate => do_saturate(&img, value),
        Filter::Opaque => do_opaque(&img),
        Filter::FillWithChannels => do_fill_with_channels(&img, value),
        Filter::Sepia => do_sepia(&img, value),
        Filter::Grayscale => do_grayscale(&img),
    }
}

#[derive(Clone)]
pub enum Filter {
    Contrast,
    Huerotate,
    BrightenByPercent,
    Saturate,
    Opaque,
    FillWithChannels,
    Sepia,
    Grayscale,
}

#[derive(Clone)]
pub enum Blend {
    Screen,
    Lighten,
    Overlay,
    SoftLight,
    Multiply,
    ColorDodge,
    Darken,
    Over,
    Exclusion,
}

fn do_contrast(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    imageops::contrast(img, num)
}

fn do_huerotate(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_i64().unwrap() as i32;
    imageops::huerotate(img, num)
}

fn do_brighten_by_percent(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    filters::brighten_by_percent(img, num)
}

fn do_saturate(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    filters::saturate(img, num)
}

fn do_opaque(img: &RgbaImage) -> RgbaImage {
    filters::restore_transparency(img)
}

fn do_fill_with_channels(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let (width, height) = img.dimensions();
    let channels_vec = value.as_array().unwrap().clone();
    let mut channels_u8_vec: Vec<u8> = Vec::new();
    for item in channels_vec {
        let zz = item.as_u64().unwrap() as u8;
        channels_u8_vec.push(zz);
    }
    let mut channels: [u8; 4] = [1, 2, 3, 4];
    channels.clone_from_slice(&channels_u8_vec);
    filters::fill_with_channels(width, height, &channels)
}

fn do_sepia(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    filters::sepia(img, num)
}

fn do_grayscale(img: &RgbaImage) -> RgbaImage {
    let out = imageops::grayscale(img);
    ConvertBuffer::convert(&out)
}
