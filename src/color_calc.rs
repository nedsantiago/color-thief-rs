use std::collections::HashMap;
use image::Rgba;

pub enum RGB {
    red,
    green,
    blue
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct MMCQ {}

impl MMCQ {
    // Settings for color binning
    const SIGNIFICANT_BITS: u8 = 5;
    const bit_shift: u8 = 8 - Self::SIGNIFICANT_BITS;

    pub fn get_history(pixels: &Vec<&Rgba<u8>>) -> Color {
        let mut color = Color{
            red: 0,
            green: 0,
            blue: 0,
        };

        // let history = HashMap::new();
    
        // Get color index
        // Shift colors based on bit shift
        // Check minimums and maximums
        for px in pixels {

            let [red, green, blue, alpha] = px.0;

            let red_shift = &red >> Self::bit_shift;
            let green_shift = green >> Self::bit_shift;
            let blue_shift = blue >> Self::bit_shift;
            println!("Pixels: {:?}", px);

            color.red = red_shift;
            color.green = green_shift;
            color.blue = blue_shift;
        }
        color
    }

    fn get_color_index(red: u8, green: u8, blue: u8) {
        (red << (2 * Self::SIGNIFICANT_BITS) + green << Self::SIGNIFICANT_BITS + blue);
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
// 
//     fn test_get_history() {
//         MMCQ::get_history();
//     }
// }
