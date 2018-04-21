extern crate image;
extern crate serde_json;

use image::{imageops, RgbaImage, ConvertBuffer};
use std::process;
use std::collections::HashMap;
use ops::filters;

pub fn apply_ops(img: RgbaImage, value: serde_json::Map<String, serde_json::Value>) -> RgbaImage {
    let mut ops_map: HashMap<String, ColorOps> = HashMap::new();
    ops_map.insert("huerotate".to_string(), ColorOps::Huerotate);
    ops_map.insert("contrast".to_string(), ColorOps::Contrast);
    ops_map.insert("saturate".to_string(), ColorOps::Saturate);
    ops_map.insert("brighten_by_percent".to_string(), ColorOps::BrightenByPercent);
    ops_map.insert("opaque".to_string(), ColorOps::Opaque);
    ops_map.insert("fill_with_channels".to_string(), ColorOps::FillWithChannels);
    ops_map.insert("sepia".to_string(), ColorOps::Sepia);
    ops_map.insert("grayscale".to_string(), ColorOps::Grayscale);

    let mut out = img.clone();
    if value.len() != 1 {
        eprintln!("len != 1");
        process::exit(1);
    }
    for (name, v) in value {
        let op: ColorOps = ops_map.get(&name).unwrap().clone();
        out = match op {
            ColorOps::Contrast => do_contrast(&out, v),
            ColorOps::Huerotate => do_huerotate(&out, v),
            ColorOps::BrightenByPercent => do_brighten_by_percent(&out, v),
            ColorOps::Saturate => do_saturate(&out, v),
            ColorOps::Opaque => do_opaque(&out),
            ColorOps::FillWithChannels => do_fill_with_channels(&out, v),
            ColorOps::Sepia => do_sepia(&out, v),
            ColorOps::Grayscale => do_grayscale(&out),
        }
    }
    out
}

#[derive(Clone)]
pub enum ColorOps {
    Contrast,
    Huerotate,
    BrightenByPercent,
    Saturate,
    Opaque,
    FillWithChannels,
    Sepia,
    Grayscale,
}

pub fn do_contrast(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    imageops::contrast(img, num)
}

pub fn do_huerotate(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_i64().unwrap() as i32;
    imageops::huerotate(img, num)
}

pub fn do_brighten_by_percent(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    filters::brighten_by_percent(img, num)
}

pub fn do_saturate(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    filters::saturate(img, num)
}

pub fn do_opaque(img: &RgbaImage) -> RgbaImage {
    filters::restore_transparency(img)
}

pub fn do_fill_with_channels(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
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

pub fn do_sepia(img: &RgbaImage, value: serde_json::Value) -> RgbaImage {
    let num = value.as_f64().unwrap() as f32;
    filters::sepia(img, num)
}

pub fn do_grayscale(img: &RgbaImage) -> RgbaImage {
    let out = imageops::grayscale(img);
    ConvertBuffer::convert(&out)
}