mod data_models;
mod img_io;
mod stats;
mod mmcq;
use crate::data_models::{ ColorChannel, MinMaxBox, BoxQueue };
use crate::mmcq::MMCQ;
use std::error::Error;
use image::Rgba;


fn main() -> Result<(), Box<dyn Error>> {
    // Load Image Data
    let img_path = "./tests/data/12colors.png";
    let img: image::RgbaImage = img_io::open_img_rgba(img_path)?;
    
    // NOTE FUTURE: Add Failure mode when Image too large
    // Make an array of rgba filtering out half alphas
    // Filter pixels
    let pixels: Vec<Rgba<u8>> = img.pixels()
        // Filter out half-transparent pixels
        .filter(|&pixel| {
            pixel[3] > 125
        })
        // Bin using MMCQ bit shift
        .map(|&pixel| MMCQ::bin_pixel(pixel))
        .collect();

    // Check validity

    // Calculate Initial MinMaxBox
    let init_minmax_box: MinMaxBox = stats::calc_minmax_box(&pixels);
    println!("minmax: {}", init_minmax_box);

    // Calculate Frequency Map

    // Calculate Histogram per dimension
    let dim_histograms = stats::calc_dim_histograms(&pixels);
    println!("Red Histogram: {:?}", dim_histograms.0[0].0);
    println!("Green Histogram: {:?}", dim_histograms.0[1].0);
    println!("Blue Histogram: {:?}", dim_histograms.0[2].0);

    // Check validity

    // Modified Median Cut Quantization
    let box_queue_itersplit: BoxQueue = mmcq::iterative_split(dim_histograms, init_minmax_box);
    println!("After Iterative Split: {}", box_queue_itersplit);

    // Calculate average color per MinMaxBox

    // Find nearest colors
    Ok(())
}
