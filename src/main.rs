mod data_models;
mod img_io;
mod stats;
mod mmcq;
use crate::data_models::{ ColorChannel, MinMaxBox };
use crate::mmcq::MMCQ;
use std::error::Error;
use image::Rgba;


fn main() -> Result<(), Box<dyn Error>> {
    // Load Image Data
    // NOTE FUTURE: Take file as cli argument
    let img_path = "/home/ubuntu-admin/dotfiles/config/bg-img/wallhaven-ox6d57_1920x1080.png";
    let img: image::RgbaImage = img_io::open_img_rgba(img_path)?;
    
    // NOTE FUTURE: Add Failure mode when Image too large
    // Make an array of rgba filtering out half alphas
    // Filter pixels
    let pixels: Vec<Rgba<u8>> = img
        .pixels()
        .filter(|&pixel| {
            pixel[3] > 125
        })
        .map(|&pixel| MMCQ::bin_pixel(pixel))
        .collect();
    println!("Binned Pixels: {:?}", pixels);

    // Check validity
    let minmax_box: MinMaxBox = stats::calc_minmax_box(&pixels);

    println!("minmax: {}", minmax_box);

    // Calculate Initial MinMaxBox
    // Calculate Frequency Map
    // Calculate Histogram per dimension
    let rhistogram = stats::calc_histogram(ColorChannel::Red, &pixels);
    let ghistogram = stats::calc_histogram(ColorChannel::Green, &pixels);
    let bhistogram = stats::calc_histogram(ColorChannel::Blue, &pixels);
    println!("Red Histogram: {:?}", rhistogram.0);
    println!("Green Histogram: {:?}", ghistogram.0);
    println!("Blue Histogram: {:?}", bhistogram.0);
    // Check validity
    // Modified Median Cut Quantization
    // Calculate average color per MinMaxBox
    // Find nearest colors
    Ok(())
}
