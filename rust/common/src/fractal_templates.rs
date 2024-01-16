use crate::complex::ComplexNumber;
use crate::models::FractalRequest;

pub fn basic(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber { a: -0.8, b: 0.0 };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    if debug {
        zoom = 0.7;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 800;
        height = 600;
    }

    let complex_width = 3.1;

    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,
        name: "basic".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn flower(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.999985881222,
        b: 0.0,
    };

    let mut zoom = 16969081.0;
    let mut max_iterations: u32 = 50_000_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.5;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
    }

    let complex_width = 4.1;

    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,
        name: "flower".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn tendrils(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.22626671100753,
        b: 1.116174442537,
    };

    // 6407226.562
    let center = ComplexNumber {
        a: -0.22626671100758155,
        b: 1.11617444253613305,
    };

    // 6407226.562
    let center = ComplexNumber {
        a: -0.2262667110076049,
        b: 1.116174442536128,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;

    let colors = 256;

    if debug {
        zoom = 6_407_226.0 * 2.0;
        max_iterations = 5_000_000;
        zoom_factor = 2.0;
        max_zoom_factor = 145_000_000.0;

        width = 4096 / 4;
        height = 2160 / 4;
    }

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "tendrils".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn julia_island(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.768778833,
        b: -0.001738996,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 100000000.0;
        max_iterations = 10_000;
        zoom_factor = 1000000.2;
        max_zoom_factor = 50_000.0;

        width = 4096;
        height = 2160;
    }

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,
        name: "julia_island".to_string(),
    };
    (req, zoom_factor, max_zoom_factor)
}

pub fn seahorse_valley(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.74351784,
        b: -0.127094578,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.5;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
    }

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "seahorse_valley".to_string(),
    };
    (req, zoom_factor, max_zoom_factor)
}

pub fn starfish(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.3740041393,
        b: 0.659792175,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
    }

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "starfish".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn sun(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.3740041393,
        b: 0.659792175,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
    }

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,
        name: "sun".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn tree(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.940157343,
        b: -1. / 1250000.0,
    };
    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;
    let complex_width = 4.1;

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
    }

    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "tree".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}
