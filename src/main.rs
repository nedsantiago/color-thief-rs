//mod data_models;
mod img_io;
mod data_models;
use std::error::Error;
use std::vec;
use image::{ Pixel, Rgba };


fn main() -> Result<(), Box<dyn Error>> {
    // Open an image file as RGBA
    // Convert the image to RGBA
    let img_path = "/home/nix-admin/.config/bg-img/wallhaven-ox6d57_1920x1080.png";
    let img: image::RgbaImage = img_io::open_img_rgba(img_path)?;

    // Get number of pixels h x w
    let total_pixel_count: u32 = img.width() * img.height();

    // May Add Failure mode: Image too large

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


    Ok(())
}
