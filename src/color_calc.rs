use std::collections::HashMap;
use std::vec::Vec;
use image::Rgba;


/// Modified Median Cut Quantization (MMCQ) encapsulates all the
/// functionality and constants for conducting the algorithm.
/// The algorithm uses binary operations. It removes smaller-
/// valued bits and leaves the larger-valued bits (i.e. 00001111 becomes
/// 00001 taking away the right-most bits) to build its palette with.
/// Defaults to 5 significant bits.
pub struct MMCQ {}

impl MMCQ {
    // Settings for color binning, how many bits to preserve
    const SIGNIFICANT_BITS: u8 = 5;
    const BIT_SHIFT: u8 = 8 - Self::SIGNIFICANT_BITS;

    pub fn get_frequency_map(pixels: &Vec<&Rgba<u8>>) -> HashMap<u32, u32> {
        let mut history: HashMap<u32, u32> = HashMap::new();
    
        // Bin/categorize colors through bit shifting
        for px in pixels {

            let [r, g, b, _a] = px.0;

            // Shift binary to right, remove low value bits
            let r_rshift = r >> Self::BIT_SHIFT;
            let g_rshift = g >> Self::BIT_SHIFT;
            let b_rshift = b >> Self::BIT_SHIFT;

            // Calculate a hash index
            let hash: u32 = Self::get_color_hash(
                r_rshift, g_rshift, b_rshift
            );
            // Count how many times color combination appears in image
            match history.get(&hash) {
                Some(count) => { history.insert(hash, count + 1); }
                None => { history.insert(hash, 1); }
            }
        }
        history
    }

    /// Creates an ID / index number / hash for each binned color
    /// combination, particularly important when implementing HashMap.
    /// The method takes RGB values and returns an unsigned integer
    /// representing the ID. Uses bit shifting to create unique ID's
    ///
    /// # Examples
    /// ```rust
    /// let result = color_calc::MMCQ::get_color_hash(15, 12, 10)
    /// assert_eq!(result, 15754);
    /// ```
    fn get_color_hash(r: u8, g: u8, b: u8) -> u32 {
        let r_lshift: u32 = (r as u32) << 2 * Self::SIGNIFICANT_BITS;
        let g_lshift: u32 = (g as u32) << Self::SIGNIFICANT_BITS;
        let b_lshift: u32 = b as u32;
        r_lshift + g_lshift + b_lshift
    }

    /// Calculates the RGB value range and a frequency map for a
    /// condensed color characteristics summary
    ///
    pub fn get_colorspace(
        pixels: &Vec<&Rgba<u8>>,
        histogram: &HashMap<u32,u32>) -> ColorSpace {
        // NOTE: It seems that get_colorspace should be part
        // of get_frequency_map calculation for better encapsulation
        let mut colorspace: ColorSpace = ColorSpace::new(histogram.clone());

        for px in pixels {
            let r = px.0[0] >> Self::BIT_SHIFT;
            let g = px.0[1] >> Self::BIT_SHIFT;
            let b = px.0[2] >> Self::BIT_SHIFT;

            colorspace.update(ColorChannel::Red, r);
            colorspace.update(ColorChannel::Green, g);
            colorspace.update(ColorChannel::Blue, b);
        }
        colorspace
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
    fn test_get_color_hash() {
        let input = [15, 12, 10];
        let resulted = MMCQ::get_color_hash(input[0], input[1], input[2]);
        let expected = 15754;
        assert_eq!(resulted, expected, "\nEXPECTED\n{}\nRESULTED\n{}", expected, resulted);
    }

}

pub struct ColorSpace {
    pub r_min: u8,
    pub r_max: u8,
    pub g_min: u8,
    pub g_max: u8,
    pub b_min: u8,
    pub b_max: u8,
    pub frequency_map: HashMap<u32, u32>,
}

enum ColorChannel {
    Red,
    Green,
    Blue,
}

impl std::fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "r({},{}), g({},{}), b({},{})",
            self.r_min, self.r_max,
            self.g_min, self.g_max,
            self.b_min, self.b_max,
        )
    }
}

impl ColorSpace {
    pub fn new(histo: HashMap<u32,u32>) -> ColorSpace {
        ColorSpace {
            r_min: 255,
            r_max: 0,
            g_min: 255,
            g_max: 0,
            b_min: 255,
            b_max: 0,
            frequency_map: histo,
        }
    }

    pub fn volume(&self) -> u32 {
        (self.r_max - self.r_min) as u32 *
            (self.g_max - self.g_min) as u32 *
            (self.b_max - self.b_min) as u32
    }

    pub fn count(&self) -> u32 {
        let mut sum_count: u32 = 0;
        for count in self.frequency_map.clone().into_values() {
            sum_count += count;
        }
        sum_count
    }
    
    fn update(&mut self, color_channel: ColorChannel, value: u8) -> () {
        match color_channel {
            ColorChannel::Red => {
                if value < self.r_min {
                    self.r_min = value;
                } else if value > self.r_max {
                    self.r_max = value;
                }
            }
            ColorChannel::Green => {
                if value < self.g_min {
                    self.g_min = value;
                } else if value > self.g_max {
                    self.g_max = value;
                }
            }
            ColorChannel::Blue => {
                if value < self.b_min {
                    self.b_min = value;
                } else if value > self.b_max {
                    self.b_max = value;
                }
            }
        }
    }

    fn clone(&self) -> ColorSpace {
        ColorSpace {
            r_min: self.r_min,
            r_max: self.r_max,
            g_min: self.g_min,
            g_max: self.g_max,
            b_min: self.b_min,
            b_max: self.b_max,
            frequency_map: self.frequency_map.clone(),
        }

    }
}


#[cfg(test)]
mod test_ColorSpace {
    use super::*;

    fn test_count(){
        let hashmap = HashMap::from([
            (26380, 11), (1057, 1), (0, 1),
            (25166, 10), (20041, 206), (22122, 2813),
            (21958, 28), (10530, 48), (14693, 24),
            (32767, 1)
        ]);
        // let queue = Queue::new(hashmap,);
        // let expected = ;
        // assert_eq!(resulted, expected, "\nEXPECTED\n{}\nRESULTED\n{}", expected, resulted);
        // histo = {26380: 11, 1057: 1, 0: 1, 25166: 10, 20041: 206, 22122: 2813, 21958: 28, 10530: 48, 14693: 24, 32767:1}
        // vbox = colorthief.VBox(0,31,0,31,0,31, histo)
        // assert vbox.count == 3143
    }
}
