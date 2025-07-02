use std::collections::HashMap;
use image::Rgba;

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

    fn get_color_hash(r: u8, g: u8, b: u8) -> u32 {
        let r_lshift: u32 = (r as u32) << 2 * Self::SIGNIFICANT_BITS;
        let g_lshift: u32 = (g as u32) << Self::SIGNIFICANT_BITS;
        let b_lshift: u32 = b as u32;
        r_lshift + g_lshift + b_lshift
    }

    fn get_colorspace(pixels: &Vec<&Rgba<u8>>, histogram: HashMap<u32,u32>) {
        // NOTE: It seems that get_colorspace should be part
        // of get_frequency_map calculation for better encapsulation
        let mut colorspace = ColorSpace::new();

        for px in pixels {
            let r = px.0[0] >> Self::BIT_SHIFT;
            let g = px.0[1] >> Self::BIT_SHIFT;
            let b = px.0[2] >> Self::BIT_SHIFT;

            colorspace.update(ColorChannel::Red, r);
            colorspace.update(ColorChannel::Green, g);
            colorspace.update(ColorChannel::Blue, b);
        }
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
    // History goes here
}

enum ColorChannel {
    Red,
    Green,
    Blue,
}

impl ColorSpace {
    pub fn new() -> ColorSpace {
        ColorSpace {
            r_min: 255,
            r_max: 0,
            g_min: 255,
            g_max: 0,
            b_min: 255,
            b_max: 0,
        }
    }

    pub fn volume(&self) -> u32 {
        (self.r_max - self.r_min) as u32 *
            (self.g_max - self.g_min) as u32 *
            (self.b_max - self.b_min) as u32
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


