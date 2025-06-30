use std::collections::HashMap;
use image::Rgba;

pub enum RGB {
    red,
    green,
    blue
}

pub fn get_history(pixels: &Vec<&Rgba<u8>>) -> HashMap<RGB,u8> {
    let mut color_bins = HashMap::new();
    color_bins
}
