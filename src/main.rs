//mod data_models;
mod img_io;
mod color_calc;
use std::error::Error;
use std::vec;


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
    let history = color_calc::MMCQ::get_history(&valid_pixels);
    println!("(r,g,b): {}, {}, {}", history.red, history.green, history.blue); 

    Ok(())
}
