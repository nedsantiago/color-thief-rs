use std::collections::HashMap;


pub struct Histogram(pub Vec<u32>);

pub struct DimHistograms(pub [Histogram; 3]);

pub struct FrequencyMap(pub HashMap<u32,u32>);

#[derive(PartialEq)]
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

pub enum ColorChannel {
    Red = 0,
    Green = 1,
    Blue = 2,
}
