use std::collections::HashMap;


pub struct Histogram(pub Vec<u8>);

pub struct FrequencyMap(pub HashMap<u32,u32>);

pub struct MinMaxBox {
    pub rmin: u8,
    pub rmax: u8,
    pub gmin: u8,
    pub gmax: u8,
    pub bmin: u8,
    pub bmax: u8,
}

pub struct BoxQueue(pub Vec<MinMaxBox>);
