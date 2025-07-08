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

    // fn test_get_frequency_map() {
    //     let pixels = vec![
    //         Rgba{r:255,g:255,b:255}, Rgba{r:247,g:247,b:247},
    //         Rgba{r:239,g:239,b:239}, Rgba{r:231,g:231,b:231},
    //         Rgba{r:223,g:223,b:223}, Rgba{r:215,g:215,b:215},
    //         Rgba{r:207,g:207,b:207}, Rgba{r:199,g:199,b:199},
    //         Rgba{r:191,g:191,b:191}, Rgba{r:183,g:183,b:183},
    //     ];
    //     let input = pixels;
    //     let resulted = MMCQ::get_frequency_map(input);
    //     let expected = HashMap::from([
    //         (32767, 1), (31710, 1),
    //         (30653, 1), (28539, 1),
    //         (28539, 1), (27482, 1),
    //         (26425, 1), (24311, 1),
    //         (23254, 1), (22197, 1),
    //     ]);
    //     assert_eq!(resulted, expected, "\nEXPECTED\n{:?}\nRESULTED\n{:?}", expected, resulted);
    // }
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
