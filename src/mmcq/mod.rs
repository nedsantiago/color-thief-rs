use std::collections::HashMap;
use crate::data_models::{ MinMaxBox };
use std::vec::Vec;
use image::{ Rgba };


fn iterative_split() {
    println!("Begin Iterative Split");
}

fn two_phase_split() {
    println!("Begin Two-Phase Split");
}

/// Modified Median Cut Quantization (MMCQ) encapsulates all the
/// functionality and constants for conducting the algorithm.
/// The algorithm uses binary operations. It removes smaller-
/// valued bits and leaves the larger-valued bits (i.e. 00001111 becomes
/// 00001 taking away the right-most bits) to build its palette with.
/// Defaults to 5 significant bits.
pub struct MMCQ;

impl MMCQ {
    // Settings for color binning, how many bits to preserve
    const SIGNIFICANT_BITS: u8 = 5;
    const BIT_SHIFT: u8 = 8 - Self::SIGNIFICANT_BITS;

    /// Creates an ID / index number / hash for each binned color
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
    fn test_hash_rgb() {
        let input = [15, 12, 10];
        let resulted = MMCQ::hash_rgb(input[0], input[1], input[2]);
        let expected = 15754;
        assert_eq!(resulted, expected, "\nEXPECTED\n{}\nRESULTED\n{}", expected, resulted);
    }
}
