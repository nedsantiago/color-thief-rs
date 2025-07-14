use std::collections::HashMap;
use crate::data_models::Histogram;
use crate::data_models::FrequencyMap;


fn calc_histogram(pixels: Vec<Rgba<u8>>) -> Histogram {
    Histogram(
        vec![
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 0,
        ]
    )
}

fn calc_frequency_map(pixels: Vec<Rgba<u8>>) -> FrequencyMap {
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

use image::Rgba;
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
        let found = calc_histogram(input).0;
        let expected = Histogram(
            vec![
                1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
            ]
        ).0;
        assert_eq!(expected, found, "Logic Error:");
    }

    #[test]
    fn test_get_frequency_map() {
        let input = vec![
            Rgba::from([31 as u8; 4]), Rgba::from([30 as u8; 4]),
            Rgba::from([29 as u8; 4]), Rgba::from([28 as u8; 4]),
            Rgba::from([27 as u8; 4]), Rgba::from([26 as u8; 4]),
            Rgba::from([25 as u8; 4]), Rgba::from([24 as u8; 4]),
            Rgba::from([23 as u8; 4]), Rgba::from([22 as u8; 4]),
        ];
        let found = calc_frequency_map(input).0;
        let expected = FrequencyMap(
            HashMap::from([
                (32767, 1), (31710, 1),
                (30653, 1), (28539, 1),
                (28539, 1), (27482, 1),
                (26425, 1), (24311, 1),
                (23254, 1), (22197, 1),
            ])
        ).0;
        assert_eq!(expected, found, "Logic Error");
    }
}
