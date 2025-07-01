use std::collections::HashMap;
use image::Rgba;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct MMCQ {}

/// Modified Median Cut Quantization (MMCQ) encapsulates all the
/// functionality and constants for conducting the algorithm.
/// The algorithm uses binary operations. It removes smaller-
/// valued bits and leaves the larger-valued bits (i.e. 00001111 becomes
/// 00001 taking away the right-most bits) to build its palette with.
/// Defaults to 5 significant bits.
///
/// # Examples
/// ```rust
/// let result = color_calc::MMCQ::get_color_hash(15, 12, 10)
/// assert_eq!(result, 15754);
/// ```
impl MMCQ {
    // Settings for color binning, how many bits to preserve
    const SIGNIFICANT_BITS: u8 = 5;
    const bit_shift: u8 = 8 - Self::SIGNIFICANT_BITS;

    pub fn get_history(pixels: &Vec<&Rgba<u8>>) -> HashMap<u32, u32> {
        let mut color = Color{
            red: 0,
            green: 0,
            blue: 0,
        };

        let mut history: HashMap<u32, u32> = HashMap::new();
    
        // Get color index
        // Shift colors based on bit shift
        // Check minimums and maximums
        for px in pixels {

            // Get Pixel value
            let [red, green, blue, alpha] = px.0;

            let red_shift = red >> Self::bit_shift;
            let green_shift = green >> Self::bit_shift;
            let blue_shift = blue >> Self::bit_shift;
            println!("Pixels: {:?}", px);

            color.red = red_shift;
            color.green = green_shift;
            color.blue = blue_shift;

            let hash: u32 = Self::get_color_hash(color.red, color.green, color.blue);
            history.insert(
                hash, 0
            );
        }
        history
    }

    fn get_color_hash(red: u8, green: u8, blue: u8) -> u32 {
        let red_lshift: u32 = (red as u32) << 2 * Self::SIGNIFICANT_BITS;
        let green_lshift: u32 = (green as u32) << Self::SIGNIFICANT_BITS;
        let blue_lshift: u32 = blue as u32;
        red_lshift + green_lshift + blue_lshift
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color_hash() {
        let input = [15, 12, 10];
        let resulted = MMCQ::get_color_hash(input[0], input[1], input[2]);
        let expected = 15754;
        assert_eq!(resulted, expected, "\nEXPECTED\n{}\nRESULTED\n{}", expected, resulted);
    }
}
