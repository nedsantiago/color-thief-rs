//mod data_models;
mod img_io;
mod data_models;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // Open an image file
    // Convert the image to RGBA
    // Make an array of rgba filtering out half alphas
    let img = "/home/nix-admin/.config/bg-img/wallhaven-ox6d57_1920x1080.png";
    let img: image::RgbaImage = img_io::open_img_rgba(img)?;
    // Get Width image
    // Get Height of image
    // Get number of pixels h x w
    let img_dimensions: data_models::Dimensions = img.dimensions();

    for pixel in img.pixels() {
        println!("{:?}", pixel);
    }

    Ok(())
}
