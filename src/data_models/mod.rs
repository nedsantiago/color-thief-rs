use std::collections::HashMap;
use std::ops::Add;


pub struct Histogram(pub Vec<u32>);

pub struct DimHistograms(pub [Histogram; 3]);

pub struct FrequencyMap(pub HashMap<u32,u32>);

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
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
            "r({},{}), g({},{}), b({},{})",
            self.rmin, self.rmax,
            self.gmin, self.gmax,
            self.bmin, self.bmax,
        )
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
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

#[derive(Clone)]
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
