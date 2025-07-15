use std::collections::HashMap;


pub struct Histogram(pub Vec<u32>);

pub struct FrequencyMap(pub HashMap<u32,u32>);

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

pub struct BoxQueue(pub Vec<MinMaxBox>);

pub enum ColorChannel {
    Red,
    Green,
    Blue
}
