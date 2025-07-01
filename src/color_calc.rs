use std::collections::HashMap;
use image::Rgba;

pub enum RGB {
    red,
    green,
    blue
}

pub struct History {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct MMCQ {}

impl MMCQ {
    // Settings for color binning
    const SIGNIFICANT_BITS: u8 = 5;
    const bit_shift: u8 = 8 - Self::SIGNIFICANT_BITS;

    pub fn get_history(pixels: &Vec<&Rgba<u8>>) -> History {
        let mut history = History{
            red: 0,
            green: 0,
            blue: 0,
        };
    
        // Get color index
        // Shift colors based on bit shift
        // Check minimums and maximums
        for px in pixels {
            history.red = px.0[0];
            history.green = px.0[1];
            history.blue = px.0[2];
            println!("px:{:?}", px);
            //history.red = px.0.red;
            //color_bins.insert(RGB::green, px.0.green);
            //color_bins.insert(RGB::blue, px.0.blue);
        }
        history
    }
}

