use std::{fs, time::Instant};

use chrono::Utc;
use image::{ImageBuffer, RgbImage};
use log::{error, info};

use crate::color::Color;
use crate::complex::ComplexNumber;

pub fn save_png(pixels: &[Color], width: u32, height: u32) {
    let start = Instant::now();
    let mut x = 0;
    let mut y = 0;
    let mut image: RgbImage = ImageBuffer::new(width, height);

    for p in pixels.iter() {
        let pixel = image::Rgb([p.r, p.g, p.b]);
        // info!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
        image.put_pixel(x, y as u32, pixel);
        x += 1;
        if x % width == 0 {
            y += 1;
            x = 0;
        }
    }
    let now = Utc::now();
    let filename = format!(
        "fractal_{}_{}_{}.png",
        now.timestamp_millis(),
        width,
        height,
    );
    let res = image.save(filename);
    let duration = start.elapsed().as_millis();
    match res {
        Ok(_) => info!("save ok. took {} ms", duration),
        Err(e) => error!("error saving file {}. took {} ms", e, duration),
    }
}

pub fn save_png2(
    pixels: &[Color],
    width: u32,
    height: u32,
    center: &ComplexNumber,
    tl: &ComplexNumber,
    br: &ComplexNumber,
    zoom: f64,
    max_iterations: u32,
    name: String,
) {
    let start = Instant::now();
    let mut x = 0;
    let mut y = 0;
    let mut image: RgbImage = ImageBuffer::new(width, height);

    for p in pixels.iter() {
        let pixel = image::Rgb([p.r, p.g, p.b]);
        // info!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
        image.put_pixel(x, y as u32, pixel);
        x += 1;
        if x % width == 0 {
            y += 1;
            x = 0;
        }
    }
    let now = Utc::now();
    let c = format!("center_a_{}_b_{}", center.a, center.b);

    let path = env!("CARGO_MANIFEST_DIR");
    // println!("CARGO_MANIFEST_DIR   {path}");
    let path = format!("{}/../../images/{}", path, name);
    fs::create_dir_all(&path).expect("create dir should work");

    let filename = format!(
        "{}/{}_{}___{}x{}_zoom_{}_max_iter_{}.png",
        path,
        now.timestamp_millis(),
        name,
        width,
        height,
        zoom,
        max_iterations
    );
    let res = image.save(filename);
    let duration = start.elapsed().as_millis();
    match res {
        Ok(_) => info!("save ok. took {} ms", duration),
        Err(e) => error!("error saving file {}. took {} ms", e, duration),
    }
}

pub fn print_debug(
    width: u32,
    height: u32,
    zoom: f64,
    center: &ComplexNumber,
    complex_width: f64,
    complex_height: f64,
    ratio: f64,
    max_iterations: u32,
) {
    info!("width {width}, height: {height}, zoom  {zoom},  complex_width {complex_width},  complex_height {complex_height}   ratio {ratio},  center {center},  max_iterations {max_iterations}");
}

