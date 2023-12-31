use serde_derive::{Deserialize, Serialize};

use crate::complex::ComplexNumber;
use crate::fractal_image::FractalImage;
use crate::image_tile::TileData;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct FractalRequest {
    pub center: ComplexNumber,
    pub width: u32,
    pub height: u32,
    pub complex_width: f64,
    pub max_iterations: u32,
    pub colors: u32,
    pub x_tiles: u32,
    pub y_tiles: u32,
    pub zoom: f64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct FractalResponse {
    pub duration_calculation: String,
    pub duration_ms: u128,
    pub fractal: FractalImage,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum WebSocketCommand {
    RENDERFRACTAL(FractalRequest),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WebSocketRequest {
    pub command: WebSocketCommand,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WebSocketResponse {
    pub tile: Option<TileData>,
}
