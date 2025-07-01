use std::collections::HashMap;
use image::Rgba;

struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

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
pub struct MMCQ {}

impl MMCQ {
    // Settings for color binning, how many bits to preserve
    const SIGNIFICANT_BITS: u8 = 5;
    const bit_shift: u8 = 8 - Self::SIGNIFICANT_BITS;

    pub fn get_frequency_map(pixels: &Vec<&Rgba<u8>>) -> HashMap<u32, u32> {
        let mut history: HashMap<u32, u32> = HashMap::new();
    
        // Get color index
        // Shift colors based on bit shift
        // Check minimums and maximums
        for px in pixels {

            // Get Pixel value
            let [red, green, blue, alpha] = px.0;

            // Shift binary to right
            let red_rshift = red >> Self::bit_shift;
            let green_rshift = green >> Self::bit_shift;
            let blue_rshift = blue >> Self::bit_shift;

            // Calculate a hash index
            let hash: u32 = Self::get_color_hash(red_rshift, green_rshift, blue_rshift);
            // Count how many times color combination appears in image
            match history.get(&hash) {
                Some(count) => { history.insert(hash, count + 1); }
                None => { history.insert(hash, 1); }
            }
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
