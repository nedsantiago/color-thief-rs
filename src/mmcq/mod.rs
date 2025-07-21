use std::vec::Vec;
use std::cmp;
use crate::data_models::{
    ColorChannel, MinMaxBox, Histogram,
    DimHistograms, BoxQueue, FrequencyMap
};
use crate::stats;
use image::Rgba;


pub fn create_box_queue(minmax_box: MinMaxBox) -> BoxQueue {
    // Put MinMaxBox in a BoxQueue
    let mut init_box_queue: Vec<MinMaxBox> = Vec::new();
    init_box_queue.push(minmax_box);
    BoxQueue {
        0: init_box_queue
    }
}

pub fn iterative_split(frequency_map: FrequencyMap, mut box_queue: BoxQueue) -> BoxQueue {
    // Get highest MinMaxBox from a count-sorted vector
    let minmax_box: MinMaxBox = match box_queue.0.pop() {
        Some(val) => val,
        None => panic!("Iterative Split tried to access an empty Queue and failed."),
    };

    // Find longest dimension in MinMaxBox (biggest range)
    let red_range: u8 = minmax_box.rmax - minmax_box.rmin;
    let green_range: u8 = minmax_box.gmax - minmax_box.gmin;
    let blue_range: u8 = minmax_box.bmax - minmax_box.bmin;

    let longest_channel: ColorChannel = if red_range >= green_range
            && red_range >= blue_range {
        ColorChannel::Red
    } else if green_range > red_range && green_range > blue_range {
        ColorChannel::Green
    } else {
        ColorChannel::Blue
    };

    // NOTE may need to refactor so that this function is in
    // main.rs (reduce dependency between libraries)
    let histogram = stats::generate_cumul_histo(frequency_map, longest_channel, minmax_box.clone());

    // Split the largest MinMaxBox
    let splitted_box: [MinMaxBox; 2] = cut_at_mmcqmedian(
        &histogram, minmax_box
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
    // After you get median, split the MinMaxBox
}

fn calc_mmcqmedian(histogram: Histogram, min: u8, max: u8, total: u32) -> u8 {
    // Calculate inverse cumulative histogram
    // Create a cumulative histogram (may implement in main)
    let median_target: u32 = total / 2;
    let mut cumsum: u32 = 0;
    let mut median: u8 = 0;
    // Find the median based on count (true median)
    let mut cumsum_histogram: Vec<u32> = Vec::new();
    for (i, &count) in histogram.0.iter().enumerate() {
        cumsum += count;
        cumsum_histogram.push(cumsum);
        if cumsum > median_target {
            median = i as u8;
            break;
        }
    }
    median += min;
    dbg!(median);

    // Adjust the median to the larger cut
    let lower_range: u8 = median - min;
    let upper_range: u8 = max - median;
    dbg!(lower_range);
    dbg!(upper_range);
    // If lower half is larger or equivalent to upper half
    if lower_range <= upper_range {
        // Adjust median (NOTE what if median was at maximum value?)
        // NOTE color-thief-py rounds a float here thus modulo was used
        median = cmp::min(max - 1, median + (upper_range / 2) + upper_range % 2);
    } else {
        dbg!(median - 1 - lower_range / 2);
        median = cmp::max(min, median - 1 - (lower_range / 2 + lower_range % 2));
    }
    // Adjust the median to a bin with a count
    dbg!(cumsum_histogram[median as usize]);
    while cumsum_histogram[median as usize] == 0 {
        median += 1;
    }
    // If walked median is the total, move back when possible
    dbg!((total - cumsum_histogram[median as usize] == 0) && cumsum_histogram[(median - 1) as usize] != 0);
    while (total - cumsum_histogram[median as usize] == 0) && cumsum_histogram[(median - 1) as usize] != 0 {
        median -= 1;
    }
    median
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

    pub fn hash_pixel(pixel: &Rgba<u8>) -> u32 {
        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];
        MMCQ::hash_rgb(r, g, b)
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
    use std::collections::HashMap;
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
        // let rhisto: Vec<u32> = vec![3, 3, 3, 3];
        // let ghisto: Vec<u32> = vec![3, 3, 1, 1, 1, 1, 0, 2];
        // let bhisto: Vec<u32> = vec![3, 1, 2, 3, 3];
        // let dim_histograms = DimHistograms {
        //     0: [
        //         Histogram {
        //             0: rhisto
        //         },
        //         Histogram {
        //             0: ghisto
        //         },
        //         Histogram {
        //             0: bhisto
        //         },
        //     ]
        // };
        let frequency_map: FrequencyMap = FrequencyMap(
            HashMap::from([
                (32767, 1), (31710, 1),
                (30653, 1),
                (28539, 2), (27482, 1),
                (26425, 1), (25368, 1),
                (24311, 1), (23254, 1),
            ])
        );
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
        let found = iterative_split(frequency_map, box_queue);
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
            12 as u32,
        );
        let found = calc_mmcqmedian(input.0, input.1, input.2, input.3);
        // let expected = 17;
        let expected = 8;
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
