extern crate image;
extern crate serde_json;

use image::RgbaImage;
use std::process;

mod filters;
mod blend_points;
mod color_ops;
mod blend_ops;

pub fn apply(mut img: RgbaImage, value: serde_json::Value) -> RgbaImage {
    match value.as_array() {
        Some(array) => {
            for item in array {
                img = apply_object(img, item.clone())
            }
        }
        None => {
            eprintln!("not array! {:?}", value.to_string());
            process::exit(1);
        }
    }
    img
}

pub fn apply_object(img: RgbaImage, value: serde_json::Value) -> RgbaImage {
    let map = match value.as_object() {
        Some(m) => m.clone(),
        None => {
            eprintln!("not object! {:?}", value.to_string());
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
            return blend_ops::apply_blend(name, ff, bb);
        }
        None => {}
    }

    color_ops::apply_ops(img, map)
}
