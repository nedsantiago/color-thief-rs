use std::collections::HashMap;
use std::vec::Vec;
use image::Rgba;


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
    pub fn color_hash(r: u8, g: u8, b: u8) -> u32 {
        let r_lshift: u32 = (r as u32) << 2 * Self::SIGNIFICANT_BITS;
        let g_lshift: u32 = (g as u32) << Self::SIGNIFICANT_BITS;
        let b_lshift: u32 = b as u32;
        r_lshift + g_lshift + b_lshift
    }

    /// Median Cut Apply
    /// Given a 3D Colorspace / box, splits the Colorspace perpendicular to
    /// one color dimension. The split will either be based on median (frequency/count)
    /// or volume * count
    pub fn median_cut_apply(colorspace: ColorSpace, histogram: &HashMap<u32, u32>) -> () {

        // Get Largest ColorSpace
        let r_range: u8 = colorspace.r_max - colorspace.r_min;
        let g_range: u8 = colorspace.g_max - colorspace.g_min;
        let b_range: u8 = colorspace.b_max - colorspace.b_min;
        println!("r_range:{}, g_range:{}, b_range:{}", r_range, g_range, b_range);
        // Check for largest range if-else statements are efficient
        let mut total = 0;
        let mut cumulative_sum: Vec<u32> = Vec::new();
        let mut color_channel: ColorChannel = ColorChannel::Red;
        if (r_range >= g_range) && (r_range >= b_range) {
            // Create a cumulative histogram
            for i in colorspace.r_min..(colorspace.r_max + 1) {
                let mut sum = 0;
                for j in colorspace.g_min..(colorspace.g_max + 1) {
                    for k in colorspace.b_min..(colorspace.b_max + 1) {
                        let hash = MMCQ::get_color_hash(i, j, k);
                        if histogram.contains_key(&hash) {
                            sum += histogram[&hash];
                        }
                    }
                }
                total += sum;
                cumulative_sum.push(total);
                // println!("total={}, sum={}", total, sum);
            }
            // println!("cumulative_sum={:?}", cumulative_sum);
        } else if (g_range > r_range) && (g_range > b_range) {
            color_channel = ColorChannel::Green;
            // Create a cumulative histogram
            let mut total = 0;
            let mut cumulative_sum: Vec<u32> = Vec::new();
            for i in colorspace.g_min..(colorspace.g_max + 1) {
                let mut sum = 0;
                for j in colorspace.r_min..(colorspace.r_max + 1) {
                    for k in colorspace.b_min..(colorspace.b_max + 1) {
                        let hash = MMCQ::get_color_hash(i, j, k);
                        if histogram.contains_key(&hash) {
                            sum += histogram[&hash];
                        }
                    }
                }
                total += sum;
                cumulative_sum.push(total);
            }
        } else {
            color_channel = ColorChannel::Blue;
            // Create a cumulative histogram
            for i in colorspace.b_min..(colorspace.b_max + 1) {
                let mut sum = 0;
                for j in colorspace.r_min..(colorspace.r_max + 1) {
                    for k in colorspace.g_min..(colorspace.g_max + 1) {
                        let hash = MMCQ::get_color_hash(i, j, k);
                        if histogram.contains_key(&hash) {
                            sum += histogram[&hash];
                        }
                    }
                }
                total += sum;
                cumulative_sum.push(total);
            }
        }
        let mut inverse_cumulative: Vec<u32> = Vec::new();
        // Inverse the data
        println!("Beginning iteration through cumulative sum");
        for step in cumulative_sum.iter() {
            inverse_cumulative.push(total - step)
        }
        println!("Result: {:?}", inverse_cumulative);

        let mut median: u8 = colorspace.r_min;
        match color_channel {
            ColorChannel::Red => {
                // Find Median
                // Initialize median as minimum value
                // Starting at minimum and moving up to maximum
                // NOTE failure of no median is found after iteration
                // NOTE make this a return function get_rough_median
                let mut i: u8 = colorspace.r_min;
                let mut median_found: bool = false;
                while !median_found && i <= colorspace.r_max {
                    let current_sum: u32 = cumulative_sum[i as usize];
                    let midpopulation: u32 = total / 2;
                    println!("current_sum: {} > midpop: {}", current_sum, midpopulation);
                    if current_sum > midpopulation {
                        median_found = true;
                        println!("Found the median at {}", i);
                        median = i;
                    }
                    i += 1;
                }
                // NOTE Make into separate function get_valid_median
                // Adjust Median based on sizes
                // median should increase into larger box
                let left = median - colorspace.r_min;
                let right = colorspace.r_max - median;

                // max value - 1, median + right / 2
                if left <= right {
                    // NOTE test for bugs about using `<` or `<=` here
                    if (colorspace.r_max - 1) <= (median + right / 2) {
                        median = colorspace.r_max - 1;
                    } else {
                        median = median + right / 2;
                    }
                } else {
                    if (colorspace.r_min - 1) >= (median + right / 2) {
                        median = colorspace.r_max - 1;
                    } else {
                        median = median + right / 2;
                    }
                }

                println!("After first adjustment, median: {}", median);
                // Median must be on a valid value while not being at last value
                // First, move up when median not in valid value
                println!("Cumulative sum at curr median: {}", cumulative_sum[median as usize]);
                while cumulative_sum[median as usize] == 0 {
                    median += 1;
                }
                println!("After moving up on zero {}", median);
                // Second, move down when current median is total
                while inverse_cumulative[median as usize] == 0 && cumulative_sum[median as usize] > 0 {
                    median -= 1;
                }
                println!("After moving down on total {}", median);
            }
            ColorChannel::Green => {
                println!("Splitting along Green!");
            }
            ColorChannel::Blue => {
                println!("Splitting along Blue!");
            }
        }
        // Split
        println!("New median: {}", median);

        fn split_colorspace(colorspace: &ColorSpace, color_channel: ColorChannel, split_line: u8) -> (ColorSpace, ColorSpace) {
            match color_channel {
                ColorChannel::Red => {
                    let left_colorspace = ColorSpace {
                        r_min: colorspace.r_min,
                        r_max: split_line,
                        g_min: colorspace.g_min,
                        g_max: colorspace.g_max,
                        b_min: colorspace.b_min,
                        b_max: colorspace.b_max,
                        frequency_map: colorspace.frequency_map.clone(),
                    };
                    let right_colorspace = ColorSpace {
                        r_min: split_line,
                        r_max: colorspace.r_max,
                        g_min: colorspace.g_min,
                        g_max: colorspace.g_max,
                        b_min: colorspace.b_min,
                        b_max: colorspace.b_max,
                        frequency_map: colorspace.frequency_map.clone(),
                    };
                    return (left_colorspace, right_colorspace)
                }
                ColorChannel::Green => {
                    let left_colorspace = ColorSpace {
                        r_min: colorspace.r_min,
                        r_max: split_line,
                        g_min: colorspace.g_min,
                        g_max: colorspace.g_max,
                        b_min: colorspace.b_min,
                        b_max: colorspace.b_max,
                        frequency_map: colorspace.frequency_map.clone(),
                    };
                    let right_colorspace = ColorSpace {
                        r_min: split_line,
                        r_max: colorspace.r_max,
                        g_min: colorspace.g_min,
                        g_max: colorspace.g_max,
                        b_min: colorspace.b_min,
                        b_max: colorspace.b_max,
                        frequency_map: colorspace.frequency_map.clone(),
                    };
                    return (left_colorspace, right_colorspace)
                }
                ColorChannel::Blue => {
                    let left_colorspace = ColorSpace {
                        r_min: colorspace.r_min,
                        r_max: split_line,
                        g_min: colorspace.g_min,
                        g_max: colorspace.g_max,
                        b_min: colorspace.b_min,
                        b_max: colorspace.b_max,
                        frequency_map: colorspace.frequency_map.clone(),
                    };
                    let right_colorspace = ColorSpace {
                        r_min: split_line,
                        r_max: colorspace.r_max,
                        g_min: colorspace.g_min,
                        g_max: colorspace.g_max,
                        b_min: colorspace.b_min,
                        b_max: colorspace.b_max,
                        frequency_map: colorspace.frequency_map.clone(),
                    };
                    return (left_colorspace, right_colorspace)
                }
            }

        }
        // Make new colorspaces
        let new_colorspaces = split_colorspace(
            &colorspace, color_channel, median
        );
        println!("New Colorspaces: {}, {}", new_colorspaces.0, new_colorspaces.1);
        
        // Change ColorSpace values

        // case match color channel
        // traverse color_channel
        // If inverse cumulative sum is lower than half of total, then median found
        //  at that median, make copies of the current colorspace as left and right boxes
        // Then check distance which is larger from min to median and max to median
        // Do corrections based on which is larger (don't allow zero count color spaces)
        // Create boxes based on the new dimensions
        // Return a tuple
        // Using sorting heuristic (population in this case)
        
        // Split into two color spaces
    }
}

#[cfg(test)]
mod test_MMCQ {
    use super::*;

    #[test]
    fn test_hash_color() {
        let input = [15, 12, 10];
        let resulted = MMCQ::hash_color(input[0], input[1], input[2]);
        let expected = 15754;
        assert_eq!(resulted, expected, "\nEXPECTED\n{}\nRESULTED\n{}", expected, resulted);
    }
}

enum ColorChannel {
    Red,
    Green,
    Blue,
}
