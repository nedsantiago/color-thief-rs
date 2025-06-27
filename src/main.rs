//mod data_models;
mod img_io;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // Open an image file
    // Convert the image to RGBA
    let img = "/home/nix-admin/.config/bg-img/wallhaven-ox6d57_1920x1080.png";
    let img = img_io::open_img_rgba(img)?;
    println!("Dimensions: {:?}", img.dimensions());

    Ok(())
    // Get Width image
    // Get Height of image
    // Get number of pixels h x w
    // Make an aray of rgba filtering out half alphas
}
