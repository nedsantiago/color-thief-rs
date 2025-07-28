use image::Rgba;
use std::collections::HashMap;
use crate::mmcq::MMCQ;
use crate::data_models::{
    Histogram, DimHistograms, FrequencyMap, MinMaxBox, ColorChannel
};


pub fn calc_dim_histograms(pixels: &Vec<Rgba<u8>>) -> DimHistograms {
    // NOTE this can be a good place to do multithreading
    let rhistogram = calc_histogram(ColorChannel::Red, &pixels);
    let ghistogram = calc_histogram(ColorChannel::Green, &pixels);
    let bhistogram = calc_histogram(ColorChannel::Blue, &pixels);

    DimHistograms {
        0: [rhistogram, ghistogram, bhistogram]
    }
}

pub fn calc_histogram(
    color_ch: ColorChannel, pixels: &Vec<Rgba<u8>>) -> Histogram {
    // Match algorithm to ColorChannel
    match color_ch {
        ColorChannel::Red => {
            generate_histogram(color_ch, &pixels)
        }
        ColorChannel::Green => {
            generate_histogram(color_ch, &pixels)
        }
        ColorChannel::Blue => {
            generate_histogram(color_ch, &pixels)
        }
    }
}

fn generate_histogram(color_ch: ColorChannel, pixels: &Vec<Rgba<u8>>) -> Histogram {
    let color_ch: usize = color_ch as usize;
    let mut histogram: Vec<u32> = Vec::new();
    let first_pixel = pixels[0];

    let first_val = first_pixel.0[color_ch];
    let mut min = first_val;
    let mut max = first_val;

    // Get Frequency per pixel value, where value is index
    for pixel in pixels {
        // Value will be used as index
        let val: u8 = pixel.0[color_ch];

        // Update min and max
        replace_minmax(val, &mut min, &mut max);
        // Lenghten histogram when too short
        while histogram.len() <= max as usize {
            // New values will be initialized to zero
            let count: u32 = 0;
            histogram.push(count);
        }
        // Increment value at index by one
        histogram[val as usize] += 1;
    }
    // Remove all values from zero to minimum value
    histogram.drain(..(min as usize));
    Histogram{
        0: histogram
    }
}

pub fn calc_frequency_map(pixels: &Vec<Rgba<u8>>, hash_algo: &dyn Fn(&Rgba<u8>) -> u32) -> FrequencyMap {
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();
    for pixel in pixels {
        let hash: u32 = hash_algo(pixel);
        let count = frequency_map.entry(hash).or_insert(0);
        *count += 1;
    }
    FrequencyMap{
        0: frequency_map
    }
}

pub fn calc_minmax_box(pixels: &Vec<Rgba<u8>>) -> MinMaxBox {
    // Initialize to first value
    let first_pixel: Rgba<u8> = pixels[0];
    let mut rmin: u8 = first_pixel.0[0];
    let mut rmax: u8 = first_pixel.0[0];
    let mut gmin: u8 = first_pixel.0[1];
    let mut gmax: u8 = first_pixel.0[1];
    let mut bmin: u8 = first_pixel.0[2];
    let mut bmax: u8 = first_pixel.0[2];

    // Find minimum and maximum values
    for pixel in pixels {
        let red: u8 = pixel.0[0];
        let green: u8 = pixel.0[1];
        let blue: u8 = pixel.0[2];
        // Replace with red if new min or max
        replace_minmax(red, &mut rmin, &mut rmax);
        replace_minmax(green, &mut gmin, &mut gmax);
        replace_minmax(blue, &mut bmin, &mut bmax);
    }

    // Generate the MinMaxBox
    MinMaxBox {
        rmin: rmin,
        rmax: rmax,
        gmin: gmin,
        gmax: gmax,
        bmin: bmin,
        bmax: bmax,
    }
}

fn replace_minmax(val: u8, min: &mut u8, max: &mut u8) -> () {
    if val < *min {
        *min = val;
    }
    if val > *max {
        *max = val;
    }
}

pub fn calc_cumul_histo(frequency_map: &FrequencyMap, color_channel: &ColorChannel, minmax_box: MinMaxBox) -> (Histogram, u32) {
    let frequency_map = &frequency_map.0;

    let ijk_range: [u8; 6] = [
        minmax_box.rmin,
        minmax_box.rmax,
        minmax_box.gmin,
        minmax_box.gmax,
        minmax_box.bmin,
        minmax_box.bmax,
    ];

    // Iterate through the bounding box min maxes
    let mut total: u32 = 0;
    let mut partialsum = Vec::new();
    for i in ijk_range[0]..(ijk_range[1] + 1) {
        let mut isum: u32 = 0;
        for j in ijk_range[2]..(ijk_range[3] + 1) {
            isum = 0;
            for k in ijk_range[4]..(ijk_range[5] + 1) {
                let color_hash = MMCQ::hash_rgb(i, j, k);
                let val = match frequency_map.get(&color_hash) {
                    Some(v) => v,
                    None => &0
                };
                isum += val;
            }
        total += isum;
        }
        partialsum.push(total);
    }
    (
        Histogram {
            0: partialsum
        },
        total
    )
}


#[cfg(test)]
mod test_stats {
    use super::*;
    use image::Rgba;
    use crate::mmcq::MMCQ;

    #[test]
    fn test_calc_histogram0() {
        let input: Vec<Rgba<u8>> = vec![
            Rgba::from([31 as u8; 4]), Rgba::from([30 as u8; 4]),
            Rgba::from([29 as u8; 4]), Rgba::from([28 as u8; 4]),
            Rgba::from([27 as u8; 4]), Rgba::from([26 as u8; 4]),
            Rgba::from([25 as u8; 4]), Rgba::from([24 as u8; 4]),
            Rgba::from([23 as u8; 4]), Rgba::from([22 as u8; 4]),
        ];
        let found = calc_histogram(ColorChannel::Red, &input).0;
        let expected = Histogram(
            vec![
                1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
            ]
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_calc_histogram1() {
        let input: Vec<Rgba<u8>> = vec![
            Rgba::from([255 as u8; 4]), Rgba::from([255 as u8; 4]),
            Rgba::from([247 as u8; 4]), Rgba::from([247 as u8; 4]),
            Rgba::from([247 as u8; 4]), Rgba::from([247 as u8; 4]),
            Rgba::from([247 as u8; 4]), Rgba::from([247 as u8; 4]),
            Rgba::from([247 as u8; 4]), Rgba::from([247 as u8; 4]),
        ];
        let found = calc_histogram(ColorChannel::Red, &input).0;
        let expected = Histogram(
            vec![
                8, 0, 0, 0, 0,
                0, 0, 0, 2,
            ]
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_calc_histogram2() {
        let input: Vec<Rgba<u8>> = vec![
            Rgba::from([0 as u8; 4]), Rgba::from([0 as u8; 4]),
            Rgba::from([0 as u8; 4]), Rgba::from([0 as u8; 4]),
            Rgba::from([1 as u8; 4]), Rgba::from([1 as u8; 4]),
            Rgba::from([1 as u8; 4]), Rgba::from([1 as u8; 4]),
        ];
        let found = calc_histogram(ColorChannel::Red, &input).0;
        let expected = Histogram(
            vec![
                4, 4,
            ]
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[ignore]
    #[test]
    fn test_calc_histogram3() {
        let input: Vec<Rgba<u8>> = vec![
            Rgba::from([ 16,   8,   0, 255]), Rgba::from([ 32,  24,  16, 255]),
            Rgba::from([ 56,  48,  40, 255]), Rgba::from([ 72,  64,  56, 255]),
            Rgba::from([ 96,  88,  80, 255]), Rgba::from([120, 112, 104, 255]),
            Rgba::from([136, 128, 120, 255]), Rgba::from([160, 152, 144, 255]),
            Rgba::from([184, 176, 168, 255]), Rgba::from([200, 192, 184, 255]),
            Rgba::from([224, 216, 208, 255]), Rgba::from([240, 232, 224, 255]),
        ];
        let found = calc_histogram(ColorChannel::Red, &input).0;
        let expected = Histogram(
            vec![
                1, 1, 2, 2, 2, 3, 3, 4, 4, 4,
                5, 5, 5, 6, 6, 7, 7, 7, 8, 8,
                8, 9, 9, 10, 10, 10, 11, 11, 12
            ]
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_calc_frequency_map0() {
        let input = vec![
            Rgba::from([31 as u8; 4]), Rgba::from([30 as u8; 4]),
            Rgba::from([29 as u8; 4]), Rgba::from([28 as u8; 4]),
            Rgba::from([27 as u8; 4]), Rgba::from([26 as u8; 4]),
            Rgba::from([25 as u8; 4]), Rgba::from([24 as u8; 4]),
            Rgba::from([23 as u8; 4]), Rgba::from([22 as u8; 4]),
        ];
        fn hash_algo(pixel: &Rgba<u8>) -> u32 {
            (pixel.0[0] + pixel.0[1] + pixel.0[2]) as u32
        }
        let found = calc_frequency_map(&input, &hash_algo).0;
        let expected = FrequencyMap(
            HashMap::from([
                (31 * 3, 1), (30 * 3, 1),
                (29 * 3, 1), (28 * 3, 1),
                (27 * 3, 1), (26 * 3, 1),
                (25 * 3, 1), (24 * 3, 1),
                (23 * 3, 1), (22 * 3, 1),
            ])
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_calc_frequency_map1() {
        let input = vec![
            Rgba::from([31 as u8; 4]), Rgba::from([30 as u8; 4]),
            Rgba::from([29 as u8; 4]), Rgba::from([27 as u8; 4]),
            Rgba::from([27 as u8; 4]), Rgba::from([26 as u8; 4]),
            Rgba::from([25 as u8; 4]), Rgba::from([24 as u8; 4]),
            Rgba::from([23 as u8; 4]), Rgba::from([22 as u8; 4]),
        ];

        let found = calc_frequency_map(&input, &MMCQ::hash_pixel).0;
        let expected = FrequencyMap(
            HashMap::from([
                (32767, 1), (31710, 1),
                (30653, 1),
                (28539, 2), (27482, 1),
                (26425, 1), (25368, 1),
                (24311, 1), (23254, 1),
            ])
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_calc_minmax_box() {
        let input = vec![
            Rgba::from([31 as u8; 4]), Rgba::from([30 as u8; 4]),
            Rgba::from([29 as u8; 4]), Rgba::from([28 as u8; 4]),
            Rgba::from([27 as u8; 4]), Rgba::from([26 as u8; 4]),
            Rgba::from([25 as u8; 4]), Rgba::from([24 as u8; 4]),
            Rgba::from([23 as u8; 4]), Rgba::from([22 as u8; 4]),
        ];
        let found = calc_minmax_box(&input);
        let expected = MinMaxBox {
            rmin: 22,
            rmax: 31,
            gmin: 22,
            gmax: 31,
            bmin: 22,
            bmax: 31,
        };
    
        assert_eq!(expected.rmin, found.rmin, "Logic Error: rmin");
        assert_eq!(expected.rmax, found.rmax, "Logic Error: rmax");
        assert_eq!(expected.gmin, found.gmin, "Logic Error: gmin");
        assert_eq!(expected.gmax, found.gmax, "Logic Error: gmax");
        assert_eq!(expected.bmin, found.bmin, "Logic Error: bmin");
        assert_eq!(expected.bmax, found.bmax, "Logic Error: bmax");
    }

    #[test]
    fn test_replace_minmax() {
        let mut min: u8 = 255;
        let mut max: u8 = 0;
        let val = 100;
        replace_minmax(val, &mut min, &mut max);

        assert_eq!(val, min, "Logic Error: Minimum should have been replaced");
        assert_eq!(val, max, "Logic Error: Maximum should have been replaced");
    }

    #[test]
    fn test_calc_cumul_histo0() {
        let frequency_map: FrequencyMap = FrequencyMap(
            HashMap::from([
                (2080, 1), (4194, 1),
                (7365, 1), (9479, 1),
                (12650, 1), (15821, 1),
                (17935, 1), (21106, 1),
                (24277, 1), (26391, 1),
                (29562, 1), (31676, 1),
            ])
        );

        let color_channel: ColorChannel = ColorChannel::Red;
        let minmax_box: MinMaxBox = MinMaxBox {
            rmin: 2,
            rmax: 30,
            gmin: 1,
            gmax: 29,
            bmin: 0,
            bmax: 28,
        };
        let expected = Histogram {
            0: [
                1, 1, 2, 2, 2, 3, 3, 4, 4,
                4, 5, 5, 5, 6, 6, 7, 7, 7,
                8, 8, 8, 9, 9, 10, 10, 10, 11, 11, 12
            ].to_vec()
        };
        let found = calc_cumul_histo(&frequency_map, &color_channel, minmax_box);
        assert_eq!(expected.0, found.0.0, "Logic Error:");
    }

    #[test]
    fn test_calc_cumul_histo1() {
        let frequency_map: FrequencyMap = FrequencyMap(
            HashMap::from([
                ( 2080, 1), ( 4194, 1),
                ( 7365, 1), ( 9479, 1),
                (12650, 1), (15821, 1),
                (17935, 1), (21106, 1),
                (24277, 1), (26391, 1),
                (29562, 1), (31676, 1),
            ])
        );

        let color_channel: ColorChannel = ColorChannel::Green;
        let minmax_box: MinMaxBox = MinMaxBox {
            rmin: 9,
            rmax: 30,
            gmin: 1,
            gmax: 29,
            bmin: 0,
            bmax: 28,
        };
        let expected = Histogram {
            0: [
                0, 0, 0, 0, 0, 0,
                0, 1, 1, 1, 2, 2,
                2, 3, 3, 4, 4, 4,
                5, 5, 5, 6, 6, 7,
                7, 7, 8, 8, 9
            ].to_vec()
        };
        let found = calc_cumul_histo(&frequency_map, &color_channel, minmax_box);
        assert_eq!(expected.0, found.0.0, "Logic Error:");
    }
}
