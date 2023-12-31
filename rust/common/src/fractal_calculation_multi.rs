use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use log::error;

use crate::color::Color;
use crate::complex::ComplexNumber;
use crate::fractal::{calc_fractal_color, calc_fractal_color2};
use crate::fractal_image::FractalImage;
use crate::palette::read_palette;
use crate::utils::{print_debug, save_png2};

pub fn calc_multi_threaded(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128, usize) {
    let complex_width = complex_width / zoom;
    let ratio = width as f64 / height as f64;
    let complex_height = complex_width / ratio;

    let palette = read_palette();

    print_debug(
        width,
        height,
        zoom,
        center,
        complex_width,
        complex_height,
        ratio,
        max_iterations,
    );

    let cores = num_cpus::get();

    let start = Instant::now();

    let re_min = center.a - complex_width / 2.0;
    let re_max = center.a + complex_width / 2.0;

    let img_min = center.b - complex_height / 2.0;
    let img_max = center.b + complex_height / 2.0;

    let x_delta = (re_max - re_min) / width as f64;
    let y_delta = (img_max - img_min) / height as f64;

    // info!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");
    //
    // info!("x_delta {},   y_delta {}   width {}  height {},  max_iterations {},  re_min {}, re_max {}, img_min {}, img_max {}" ,
    //     x_delta, y_delta, width,  height, max_iterations, re_min, re_max, img_min, img_max);

    let pixels = vec![Color::default(); width as usize * height as usize];

    let mut threads = vec![];

    let y_global = 0;

    let pixels = Arc::new(Mutex::new(pixels));
    let y_global = Arc::new(Mutex::new(y_global));

    for _ in 0..cores {
        let colors = color_palette(colors, &palette);

        let mut pixels = Arc::clone(&pixels);
        let y_global = Arc::clone(&y_global);

        let thread_join_handle = thread::spawn(move || {
            //  info!("thread  {:?} started ", thread::current().id());

            let start = Instant::now();
            let mut calculated_rows = 0;
            let mut y_thread = 0;

            while *y_global.lock().unwrap() < height {
                {
                    let mut y_global = y_global.lock().unwrap();
                    if *y_global < height {
                        y_thread = *y_global;
                        *y_global += 1;
                        // info!("thread  {:?} handles y {y_thread} ", thread::current().id());
                    }
                }
                // y_global is unlocked
                let mut pixels_thread = vec![Color::default(); width as usize];

                if y_thread < height {
                    for x in 0..width {
                        let p = calc_fractal_color(
                            x,
                            y_thread,
                            re_min,
                            img_min,
                            x_delta,
                            y_delta,
                            max_iterations,
                            &colors,
                        );
                        pixels_thread[x as usize].r = p.r;
                        pixels_thread[x as usize].g = p.g;
                        pixels_thread[x as usize].b = p.b;
                    }
                    {
                        copy_pixel_row(width, &pixels_thread, &mut pixels, y_thread);
                    }
                }
                calculated_rows += 1;
            }

            let duration = start.elapsed().as_millis();
            // let msg = format!(
            //     "thread {:?} spent {} ms working on {} rows.",
            //     thread::current().id(),
            //     duration,
            //     calculated_rows
            // );

            (duration, calculated_rows)
        });
        threads.push(thread_join_handle);
    }

    let mut joined = 0;
    for t in threads {
        let res = t.join();
        match res {
            Ok((duration, calculated_rows)) => {
                joined += 1;
                // info!(
                //     "thread successfully joined {joined}/{cores} threads finished  // thread worked for {} ms on {} rows",
                //     duration,calculated_rows
                // );
            }
            Err(e) => error!("thread returned an error {:?}", e),
        }
    }
    let duration = start.elapsed().as_millis();
    let mutex = Arc::into_inner(pixels).unwrap();
    let pixels = mutex.into_inner().unwrap();

    let tl = ComplexNumber {
        a: re_min,
        b: img_max,
    };
    let br = ComplexNumber {
        a: re_max,
        b: img_min,
    };

    save_png2(
        &pixels,
        width,
        height,
        &center,
        &tl,
        &br,
        zoom,
        max_iterations,
        name,
    );

    let fractal = FractalImage {
        width,
        height,
        pixels,
    };

    (fractal, duration, cores)
}

pub fn calc_multi_threaded_opt1(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128, usize) {
    let complex_width = complex_width / zoom;
    let ratio = width as f64 / height as f64;
    let complex_height = complex_width / ratio;

    let palette = read_palette();

    print_debug(
        width,
        height,
        zoom,
        center,
        complex_width,
        complex_height,
        ratio,
        max_iterations,
    );

    let cores = num_cpus::get();

    let start = Instant::now();

    let re_min = center.a - complex_width / 2.0;
    let re_max = center.a + complex_width / 2.0;

    let img_min = center.b - complex_height / 2.0;
    let img_max = center.b + complex_height / 2.0;

    let x_delta = (re_max - re_min) / width as f64;
    let y_delta = (img_max - img_min) / height as f64;

    // info!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");
    //
    // info!("x_delta {},   y_delta {}   width {}  height {},  max_iterations {},  re_min {}, re_max {}, img_min {}, img_max {}" ,
    //     x_delta, y_delta, width,  height, max_iterations, re_min, re_max, img_min, img_max);

    let pixels = vec![Color::default(); width as usize * height as usize];

    let mut threads = vec![];

    let y_global = 0;

    let pixels = Arc::new(Mutex::new(pixels));
    let y_global = Arc::new(Mutex::new(y_global));

    for _ in 0..cores {
        let colors = color_palette(colors, &palette);

        // let z1 = z1.clone();
        let mut pixels_thread = vec![Color::default(); width as usize];

        let mut pixels = Arc::clone(&pixels);
        let y_global = Arc::clone(&y_global);

        let thread_join_handle = thread::spawn(move || {
            // info!("thread  {:?} started ", thread::current().id());

            let start = Instant::now();
            let mut calculated_rows = 0;
            let mut y_thread = 0;

            while *y_global.lock().unwrap() < height {
                {
                    let mut y_global = y_global.lock().unwrap();
                    if *y_global < height {
                        y_thread = *y_global;
                        *y_global += 1;
                        // info!("thread  {:?} handles y {y_thread} ", thread::current().id());
                    }
                }
                // y_global is unlocked

                if y_thread < height {
                    for x in 0..width {
                        let p = calc_fractal_color(
                            x,
                            y_thread,
                            re_min,
                            img_min,
                            x_delta,
                            y_delta,
                            max_iterations,
                            &colors,
                        );
                        pixels_thread[x as usize].r = p.r;
                        pixels_thread[x as usize].g = p.g;
                        pixels_thread[x as usize].b = p.b;
                    }
                    {
                        copy_pixel_row(width, &pixels_thread, &mut pixels, y_thread);
                    }
                }
                calculated_rows += 1;
            }

            let duration = start.elapsed().as_millis();
            // let msg = format!(
            //     "thread {:?} spent {} ms working on {} rows.",
            //     thread::current().id(),
            //     duration,
            //     calculated_rows
            // );

            (duration, calculated_rows)
        });
        threads.push(thread_join_handle);
    }

    let mut joined = 0;
    for t in threads {
        let res = t.join();
        match res {
            Ok((duration, calculated_rows)) => {
                joined += 1;
                // info!(
                //     "thread successfully joined {joined}/{cores} threads finished  // thread worked for {} ms on {} rows",
                //     duration,calculated_rows
                // );
            }
            Err(e) => error!("thread returned an error {:?}", e),
        }
    }
    let duration = start.elapsed().as_millis();
    let mutex = Arc::into_inner(pixels).unwrap();
    let pixels = mutex.into_inner().unwrap();

    let tl = ComplexNumber {
        a: re_min,
        b: img_max,
    };
    let br = ComplexNumber {
        a: re_max,
        b: img_min,
    };

    save_png2(
        &pixels,
        width,
        height,
        &center,
        &tl,
        &br,
        zoom,
        max_iterations,
        name,
    );

    let fractal = FractalImage {
        width,
        height,
        pixels,
    };

    (fractal, duration, cores)
}

pub fn calc_multi_threaded_opt2(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128, usize) {
    let complex_width = complex_width / zoom;
    let ratio = width as f64 / height as f64;
    let complex_height = complex_width / ratio;

    let palette = read_palette();

    print_debug(
        width,
        height,
        zoom,
        center,
        complex_width,
        complex_height,
        ratio,
        max_iterations,
    );
    let cores = num_cpus::get();
    let start = Instant::now();
    let re_min = center.a - complex_width / 2.0;
    let re_max = center.a + complex_width / 2.0;
    let img_min = center.b - complex_height / 2.0;
    let img_max = center.b + complex_height / 2.0;

    let x_delta = (re_max - re_min) / width as f64;
    let y_delta = (img_max - img_min) / height as f64;

    // info!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");
    //
    // info!("x_delta {},   y_delta {}   width {}  height {},  max_iterations {},  re_min {}, re_max {}, img_min {}, img_max {}" ,
    //     x_delta, y_delta, width,  height, max_iterations, re_min, re_max, img_min, img_max);

    let pixels = vec![Color::default(); width as usize * height as usize];

    let mut threads = vec![];

    let y_global = 0;

    let pixels = Arc::new(Mutex::new(pixels));
    let y_global = Arc::new(Mutex::new(y_global));

    for _ in 0..cores {
        let colors = color_palette(colors, &palette);

        // let z1 = z1.clone();
        let mut pixels_thread = vec![Color::default(); width as usize];

        let mut pixels = Arc::clone(&pixels);
        let y_global = Arc::clone(&y_global);

        let thread_join_handle = thread::spawn(move || {
            // info!("thread  {:?} started ", thread::current().id());

            let start = Instant::now();
            let mut calculated_rows = 0;
            let mut y_thread = 0;

            while *y_global.lock().unwrap() < height {
                {
                    let mut y_global = y_global.lock().unwrap();
                    if *y_global < height {
                        y_thread = *y_global;
                        *y_global += 1;
                        // info!("thread  {:?} handles y {y_thread} ", thread::current().id());
                    }
                }
                // y_global is unlocked

                if y_thread < height {
                    for x in 0..width {
                        let p = calc_fractal_color2(
                            x,
                            y_thread,
                            re_min,
                            img_min,
                            x_delta,
                            y_delta,
                            max_iterations,
                            &colors,
                            &mut pixels_thread[x as usize],
                        );
                    }
                    {
                        copy_pixel_row(width, &pixels_thread, &mut pixels, y_thread);
                    }
                }
                calculated_rows += 1;
            }

            let duration = start.elapsed().as_millis();

            (duration, calculated_rows)
        });
        threads.push(thread_join_handle);
    }

    let mut joined = 0;
    for t in threads {
        let res = t.join();
        match res {
            Ok((duration, calculated_rows)) => {
                joined += 1;
                // info!(
                //     "thread successfully joined {joined}/{cores} threads finished  // thread worked for {} ms on {} rows",
                //     duration,calculated_rows
                // );
            }
            Err(e) => error!("thread returned an error {:?}", e),
        }
    }

    let duration = start.elapsed().as_millis();
    let mutex = Arc::into_inner(pixels).unwrap();
    let pixels = mutex.into_inner().unwrap();

    let tl = ComplexNumber {
        a: re_min,
        b: img_max,
    };
    let br = ComplexNumber {
        a: re_max,
        b: img_min,
    };

    save_png2(
        &pixels,
        width,
        height,
        &center,
        &tl,
        &br,
        zoom,
        max_iterations,
        name,
    );

    let fractal = FractalImage {
        width,
        height,
        pixels,
    };

    (fractal, duration, cores)
}

fn color_palette(colors: u32, palette: &HashMap<String, Vec<Color>>) -> Vec<Color> {
    let colors: Vec<Color> = match colors {
        16 => palette.get("wild.map").unwrap().clone(),
        256 => palette.get("basic.map").unwrap().clone(),
        _ => panic!("number of colors not supported {}", colors),
    };
    colors
}

fn copy_pixel_row(
    width: u32,
    mut pixels_thread: &Vec<Color>,
    pixels: &Arc<Mutex<Vec<Color>>>,
    mut y_thread: u32,
) {
    let mut p = pixels.lock().unwrap();
    for i in 0..width {
        let idx = y_thread * width + i;
        let p = &mut *p;
        let pixel = &mut p[idx as usize];
        pixel.r = pixels_thread[i as usize].r;
        pixel.g = pixels_thread[i as usize].g;
        pixel.b = pixels_thread[i as usize].b;
    }
}
