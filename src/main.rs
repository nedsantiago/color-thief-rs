//mod data_models;
mod img_io;


fn main() {
    // Open an image file
    // Convert the image to RGBA
    let img = "/home/nix-admin/.config/bg-img/wallhaven-ox6d57_1920x1080.png";
    match img_io::open_img_rgba(img) {
        Ok(img) => {
            println!("Dimensions: {:?}", img.dimensions());
        },
        Err(err) => {
            println!("Error {err}")
        }
    }
    // Get Width image
    // Get Height of image
    // Get number of pixels h x w
    // Make an aray of rgba filtering out half alphas
}
