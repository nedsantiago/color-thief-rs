use std::vec::Vec;
use crate::data_models::{
    ColorChannel, MinMaxBox, Histogram, DimHistograms, BoxQueue
};
use image::Rgba;


pub fn create_box_queue(minmax_box: MinMaxBox) -> BoxQueue {
    // Put MinMaxBox in a BoxQueue
    let mut init_box_queue: Vec<MinMaxBox> = Vec::new();
    init_box_queue.push(minmax_box);
    BoxQueue {
        0: init_box_queue
    }
}

pub fn iterative_split(dim_histograms: DimHistograms, mut box_queue: BoxQueue) -> BoxQueue {
    // Get highest MinMaxBox from a count-sorted vector
    let minmax_box: MinMaxBox = match box_queue.0.pop() {
        Some(val) => val,
        None => panic!("Iterative Split tried to access an empty Queue and failed."),
    };

    // Find longest dimension in MinMaxBox (biggest range)
    let red_range: u8 = minmax_box.rmax - minmax_box.rmin;
    let green_range: u8 = minmax_box.gmax - minmax_box.gmin;
    let blue_range: u8 = minmax_box.bmax - minmax_box.bmin;

    let longest_channel: ColorChannel = if (red_range >= green_range
            && red_range >= blue_range) {
        ColorChannel::Red
    } else if green_range > red_range && green_range > blue_range {
        ColorChannel::Green
    } else {
        ColorChannel::Blue
    };

    // Split the largest MinMaxBox
    let splitted_box: [MinMaxBox; 2] = cut_at_mmcqmedian(
        &dim_histograms.0[longest_channel as usize], minmax_box
    );

    // Push new MinMaxBoxes back into BoxQueue
    for mmbox in splitted_box {
        box_queue.0.push(mmbox)
    }
    // Until max iterations reached
    box_queue
}

fn cut_at_mmcqmedian(histogram: &Histogram, minmax_box: MinMaxBox) -> [MinMaxBox; 2] {
    // Get median
    // Split Box
    // Cut the perpendicular to longest dimension
    [
        MinMaxBox {
            rmin: minmax_box.rmin,
            rmax: minmax_box.rmax,
            gmin: minmax_box.gmin,
            gmax: minmax_box.gmax,
            bmin: minmax_box.bmin,
            bmax: minmax_box.bmax,
        },
        MinMaxBox {
            rmin: minmax_box.rmin,
            rmax: minmax_box.rmax,
            gmin: minmax_box.gmin,
            gmax: minmax_box.gmax,
            bmin: minmax_box.bmin,
            bmax: minmax_box.bmax,
        },
    ]
}

fn calc_mmcqmedian(histogram: Histogram, min: u8, max: u8) -> u8 {
    // Calculate inverse cumulative histogram
    // NOTE failure when no median is found
    // Find the median (NOTE get_rough_median func)
    // Adjust the median to a bin with a count move median
    // After you get median, split the MinMaxBox

    // Create a cumulative histogram (may implement in main)
    let mut cumhisto: Vec<u32> = Vec::new();
    let mut cumsum: u32 = 0;
    for i in 0..histogram.0.len() {
        let count: u32 = histogram.0[i];
        cumsum += count;
        cumhisto.push(cumsum);
        println!("{:?}, {}", cumhisto, count);
    }
    let total: u32 = cumsum;
    cumsum = 0;
    let mut median: u8 = 0;
    let mut i: usize = 0;
    let mut is_median_found: bool = false;
    while i < histogram.0.len() && !is_median_found {
        cumsum += histogram.0[i];
        println!("currsum:{} total:{}, i:{}", cumsum, total / 2, i);
        if cumsum > total / 2 {
            median = i as u8;
            is_median_found = true;
        }
        i += 1;
    }
    for i in 0..histogram.0.len() {
        let count: u32 = histogram.0[i];
    }
    median + min
}

fn split_box(minmax_box: MinMaxBox, split_val: u8) {
    // Create a left box
    // Create a right box
}

fn two_phase_split(dim_histograms: DimHistograms, minmax_boxes: Vec<MinMaxBox>) {
    println!("Begin Two-Phase Split");
    // Get highest MinMaxBox from a volume-count-sorted vector
    // Get median Split Box
    // Until max iterations reached
}

fn sort_box_queue(box_queue: BoxQueue) {
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
mod test_mmcq {
    use super::*;

    use crate::data_models::{ BoxQueue, MinMaxBox };
    use image::Rgba;

    #[test]
    fn test_create_box_queue() {
        let input = MinMaxBox {
            rmin: 0,
            rmax: 31,
            gmin: 0,
            gmax: 31,
            bmin: 0,
            bmax: 31,
        };
        let found = create_box_queue(input);
        let expected = BoxQueue {
            0: vec![MinMaxBox{
                rmin: 0,
                rmax: 31,
                gmin: 0,
                gmax: 31,
                bmin: 0,
                bmax: 31,
            }],
        };
        assert_eq!(expected.0[0], found.0[0], "Logic Error:");
    }

    #[ignore]
    #[test]
    fn test_iterative_split() {
        let rhisto: Vec<u32> = vec![3, 3, 3, 3];
        let ghisto: Vec<u32> = vec![3, 3, 1, 1, 1, 1, 0, 2];
        let bhisto: Vec<u32> = vec![3, 1, 2, 3, 3];
        let dim_histograms = DimHistograms {
            0: [
                Histogram {
                    0: rhisto
                },
                Histogram {
                    0: ghisto
                },
                Histogram {
                    0: bhisto
                },
            ]
        };
        let minmax_box = MinMaxBox {
            rmin: 0,
            rmax: 3,
            gmin: 11,
            gmax: 18,
            bmin: 0,
            bmax: 4,
        };
        let box_queue = BoxQueue {
            0: vec![minmax_box]
        };
        let found = iterative_split(dim_histograms, box_queue);
        let expected = BoxQueue {
            0: vec![
                MinMaxBox {
                    rmin: 0,
                    rmax: 3,
                    gmin: 11,
                    gmax: 14,
                    bmin: 0,
                    bmax: 4,
                },
                MinMaxBox {
                    rmin: 0,
                    rmax: 3,
                    gmin: 15,
                    gmax: 18,
                    bmin: 0,
                    bmax: 4,
                },
            ]
        };
        assert_eq!(expected, found, "Logic Error:");
    }

    // MinMaxBox {
    //     rmin: 2,
    //     rmax: 30,
    //     gmin: 1,
    //     gmax: 29,
    //     bmin: 0,
    //     bmax: 28,
    // }
    #[test]
    fn test_calc_mmcqmedian() {
        let input = (
            Histogram {
                0: [
                    1, 0, 1, 0, 0, 1, 0, 1, 0,
                    0, 1, 0, 0, 1, 0, 1, 0, 0,
                    1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1].to_vec()
            },
            2 as u8,
            30 as u8,
        );
        let found = calc_mmcqmedian(input.0, input.1, input.2);
        let expected = 17;
        assert_eq!(expected, found, "Logic Error:");
    }

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
