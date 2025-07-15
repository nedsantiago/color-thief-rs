use image::Rgba;
use std::collections::HashMap;
use crate::data_models::{ Histogram, FrequencyMap, MinMaxBox };
use crate::data_models;


fn calc_histogram(pixels: &Vec<Rgba<u8>>) -> Histogram {
    let mut histogram: Vec<u32> = Vec::new();
    let first_pixel = pixels[0];

    // Assume red channel for now
    let first_red = first_pixel.0[0];
    let mut min = first_red;
    let mut max = first_red;

    for pixel in pixels {
        let red: u8 = pixel.0[0];
        let green: u8 = pixel.0[1];
        let blue: u8 = pixel.0[2];

        let vec_index = red;
        replace_minmax(vec_index, &mut min, &mut max);

        while histogram.len() < max as usize {
            let count: u32 = 0;
            histogram.push(count);
        }
        histogram[(vec_index - 1) as usize] += 1;
    }
    histogram.drain(..((min - 1) as usize));
    data_models::Histogram{
        0: histogram
    }
}

fn calc_frequency_map(pixels: &Vec<Rgba<u8>>) -> FrequencyMap {
    FrequencyMap(
        HashMap::from([
            (32767, 1), (31710, 1),
            (30653, 1), (28539, 1),
            (28539, 1), (27482, 1),
            (26425, 1), (24311, 1),
            (23254, 1), (22197, 2),
        ])
    )
}

fn calc_minmax_box(pixels: &Vec<Rgba<u8>>) -> MinMaxBox {
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


#[cfg(test)]
mod test_stats {
    use super::*;

    #[test]
    fn test_calc_histogram() {
        let input: Vec<Rgba<u8>> = vec![
            Rgba::from([31 as u8; 4]), Rgba::from([30 as u8; 4]),
            Rgba::from([29 as u8; 4]), Rgba::from([28 as u8; 4]),
            Rgba::from([27 as u8; 4]), Rgba::from([26 as u8; 4]),
            Rgba::from([25 as u8; 4]), Rgba::from([24 as u8; 4]),
            Rgba::from([23 as u8; 4]), Rgba::from([22 as u8; 4]),
        ];
        let found = calc_histogram(&input).0;
        let expected = Histogram(
            vec![
                1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
            ]
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_calc_frequency_map() {
        let input = vec![
            Rgba::from([31 as u8; 4]), Rgba::from([30 as u8; 4]),
            Rgba::from([29 as u8; 4]), Rgba::from([28 as u8; 4]),
            Rgba::from([27 as u8; 4]), Rgba::from([26 as u8; 4]),
            Rgba::from([25 as u8; 4]), Rgba::from([24 as u8; 4]),
            Rgba::from([23 as u8; 4]), Rgba::from([22 as u8; 4]),
        ];
        let found = calc_frequency_map(&input).0;
        let expected = FrequencyMap(
            HashMap::from([
                (32767, 1), (31710, 1),
                (30653, 1), (28539, 1),
                (28539, 1), (27482, 1),
                (26425, 1), (24311, 1),
                (23254, 1), (22197, 1),
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
}
