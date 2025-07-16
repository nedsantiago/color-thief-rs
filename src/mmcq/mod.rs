use std::vec::Vec;
use crate::data_models::{ ColorChannel, MinMaxBox, Histogram, DimHistograms };
use image::{ Rgba };


fn iterative_split(dim_histograms: DimHistograms, minmax_box: MinMaxBox) {
    println!("Begin Iterative Split");
    // Put MinMaxBox in a BoxQueue

    // Get highest MinMaxBox from a count-sorted vector
    // Get median
    // Split Box
    // Until max iterations reached
}

fn two_phase_split(dim_histograms: DimHistograms, minmax_boxes: Vec<MinMaxBox>) {
    println!("Begin Two-Phase Split");
    // Get highest MinMaxBox from a volume-count-sorted vector
    // Get median
    // Split Box
    // Until max iterations reached
}

fn get_median(histogram: Histogram, minmax_box: MinMaxBox) -> () {
    // Find longest dimension in MinMaxBox (biggest range)
    // Cut the perpendicular to longest dimension
    // Create a cumulative histogram (may implement in main)
    // Calculate inverse cumulative histogram
    // NOTE failure when no median is found
    // Find the median (NOTE get_rough_median func)
    // Adjust the median to a bin with a count move median
    // After you get median, split the MinMaxBox
}

fn split_box(minmax_box: MinMaxBox, split_val: u8) {
    // Create a left box
    // Create a right box
}

/// Modified Median Cut Quantization (MMCQ) encapsulates all the
/// functionality and constants for conducting the algorithm.
/// The algorithm uses binary operations. It removes smaller-
/// valued bits and leaves the larger-valued bits (i.e. 00001111 becomes
/// 00001 taking away the right-most bits) to build its palette with.
/// Uses 5 significant bits.
pub struct MMCQ;

impl MMCQ {
    // Settings for color binning, how many bits to preserve
    const SIGNIFICANT_BITS: u8 = 5;
    const BIT_SHIFT: u8 = 8 - Self::SIGNIFICANT_BITS;

    /// Creates an hashed color for each binned color
    /// combination, particularly important when implementing HashMap.
    /// The method takes RGB values and returns an unsigned integer
    /// representing the ID. Uses bit shifting to create unique ID's
    ///
    /// # Examples
    /// ```rust
    /// let result = mmcq::MMCQ::color_hash(15, 12, 10)
    /// assert_eq!(result, 15754);
    /// ```
    pub fn hash_rgb(r: u8, g: u8, b: u8) -> u32 {
        let r_lshift: u32 = (r as u32) << 2 * Self::SIGNIFICANT_BITS;
        let g_lshift: u32 = (g as u32) << Self::SIGNIFICANT_BITS;
        let b_lshift: u32 = b as u32;
        r_lshift + g_lshift + b_lshift
    }

    pub fn bin_pixel(pixel: Rgba<u8>) -> Rgba<u8> {
        let pixel = pixel.0;

        let r_rshift: u8 = pixel[0] >> Self::BIT_SHIFT;
        let g_rshift: u8 = pixel[1] >> Self::BIT_SHIFT;
        let b_rshift: u8 = pixel[2] >> Self::BIT_SHIFT;

        Rgba {
            0: [r_rshift, g_rshift, b_rshift, pixel[3]]
        }
    }
}

#[cfg(test)]
mod test_MMCQ {
    use super::*;

    #[test]
    fn test_hash_pixel() {
        let input = [15, 12, 10];
        let found = MMCQ::hash_rgb(input[0], input[1], input[2]);
        let expected = 15754;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_bin_color() {
        let input = Rgba::from([255 as u8; 4]);
        let found = MMCQ::bin_pixel(input);
        let expected = Rgba::from([31, 31, 31, 255]);
        assert_eq!(expected, found, "Logic Error:");
    }
}
