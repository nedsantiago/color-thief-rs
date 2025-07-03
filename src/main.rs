//mod data_models;
mod img_io;
mod color_calc;
mod queue;
use std::error::Error;
use std::collections::HashMap;


fn main() -> Result<(), Box<dyn Error>> {
    // NOTE FUTURE: Take file as cli argument
    let img_path = "/home/nix-admin/.config/bg-img/wallhaven-ox6d57_1920x1080.png";
    let max_colors: u8 = 10;

    // Open and Convert the image to RGBA
    let img: image::RgbaImage = img_io::open_img_rgba(img_path)?;

    // NOTE FUTURE: Add Failure mode when Image too large

    // Make an array of rgba filtering out half alphas
    let mut valid_pixels = Vec::new();
    let pixels = img.pixels();
    for pixel in pixels {
        // If alpha is greater than half alpha
        if pixel.0[3] >= 125 {
            // Add as valid pixel
            valid_pixels.push(pixel)
        }
    }

    // Calculate colors
    let histogram = color_calc::MMCQ::get_frequency_map(&valid_pixels);
    println!("histogram: {:?}", histogram);
    
    let histogram_len: usize = histogram.len();
    // If colors in histogram aren't enough, run a failure mode
    if histogram_len as u32 <= max_colors as u32 {
        // Ask for a different image and exit
        println!("Image has a small color range. Try another one.");
        return Ok(());
    }

    // Calculate the color space
    let colorspace = color_calc::MMCQ::get_colorspace(&valid_pixels, &histogram);
    println!("Colorspace:{}", colorspace);

    fn get_hashmap_count(h: HashMap<u32, u32>) -> u32 {
        let mut sum_count: u32 = 0;
        for count in h.into_values() {
            sum_count += count;
        }
        sum_count
    }
    // Create a queue for sorting data
    let mut count_sorter = queue::Queue::new(
        &get_hashmap_count,
        histogram
    );

    let count = count_sorter.count();
    println!("Histogram count: {}", count);

    Ok(())
}
