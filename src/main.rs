extern crate clap;
extern crate huahua;
extern crate image;
extern crate serde_json;

use std::process;
use std::fs::File;
use std::path::Path;
use std::error::Error;

use clap::{Arg, App};

use huahua::ops::apply;


fn main() {
    let (img, output, value) = args_check();
    let out = apply(img.to_rgba(), value);
    out.save(output).unwrap();
}

fn args_check() -> (image::DynamicImage, String, serde_json::Value) {
    let matches = App::new("huahua")
        .version("0.1.0")
        .author("Luke Euler <luke16times@gmail.com>")
        .about("Apply custom filters to you photos")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Path to the input image file")
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Output file name")
            .default_value("output.jpg"))
        .arg(Arg::with_name("filter_config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Set the filter file")
            .default_value("filter.json"))
        .arg(Arg::with_name("filter")
            .short("f")
            .long("filter")
            .value_name("name")
            .help("Choose the filter in you filter file")
            .required(true))
        .get_matches();
    let output = matches.value_of("output").unwrap();
    let input = matches.value_of("input").unwrap();
    let filter = matches.value_of("filter").unwrap();
    let config_file = matches.value_of("filter_config").unwrap();
    println!("input: {:?}, output: {:?}, filter: {:?}, config: {:?}", input, output, filter, config_file);

    let img = match image::open(input) {
        Ok(t) => t,
        Err(_) => {
            println!("can not find image file {:?}", input);
            process::exit(1)
        }
    };

    let config = match read_config_from_file(config_file) {
        Ok(v) => v,
        Err(_) => {
            println!("something error for config file {:?}", config_file);
            process::exit(1)
        }
    };

    let value = match config.get(filter) {
        Some(v) => v,
        None => {
            println!("can not find {:?} from {:?}", filter, config_file);
            process::exit(1)
        }
    };

    (img, output.to_string(), value.clone())
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<serde_json::Value, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    let v: serde_json::Value = serde_json::from_reader(file)?;
    Ok(v)
}
