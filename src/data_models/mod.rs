use std::collections::HashMap;
use std::ops::Add;


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Histogram(pub Vec<u32>);

// Encapsulation with effectively-functional programming methods
// impl Histogram {
//     fn total(self) -> u32 {
//         let mut total = 0;
//         for count in self.0 {
//             total += count;
//         }
//         total
//     }
// }

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DimHistograms(pub [Histogram; 3]);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrequencyMap(pub HashMap<u32,u32>);

impl std::fmt::Display for FrequencyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.0
        )
    }
}

impl count for FrequencyMap(&self, minmax_box: &MinMaxBox, hash_alg: Fn([u8; 3]) -> u32 ) {
    for r in minmax_box.rmin..(minmax_box.rmax + 1) {
        for g in minmax_box.gmin..(minmax_box.gmax + 1) {
            for b in minmax_box.bmin..(minmax_box.bmax + 1) {
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MinMaxBox {
    pub rmin: u8,
    pub rmax: u8,
    pub gmin: u8,
    pub gmax: u8,
    pub bmin: u8,
    pub bmax: u8,
}

impl std::fmt::Display for MinMaxBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MinMaxBox(R({},{}), G({},{}), B({},{}))",
            self.rmin, self.rmax,
            self.gmin, self.gmax,
            self.bmin, self.bmax,
        )
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoxQueue(pub Vec<MinMaxBox>);

impl std::fmt::Display for BoxQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0[0],
        )
    }
}

#[derive(Debug, Clone)]
pub enum ColorChannel {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl Add<i32> for ColorChannel {
    type Output = ColorChannel;

    fn add(self, other: i32) -> ColorChannel {
        let sum_mod3 = (other + self as i32) % 3;
        
        match sum_mod3 {
            0 => ColorChannel::Red,
            1 => ColorChannel::Green,
            2 => ColorChannel::Blue,
            _ => panic!("sum_mod3 should only be between 0 and 2, received {}", sum_mod3),
        }
    }
}
