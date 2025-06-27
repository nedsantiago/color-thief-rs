use image::{ImageReader, RgbaImage, ImageError};

pub fn open_img_rgba(img_dir: &str) -> Result<RgbaImage, ImageError>{
    // Open the image directory
    let img = ImageReader::open(img_dir)?;
    // Decode data when successfully read
    let img = img.decode()?;
    // Convert data to RGBA8 when successfully decoded
    let img = img.to_rgba8();

    Ok(img)
}
