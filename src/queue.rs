use std::vec::Vec;
use std::collections::HashMap;


struct Queue<'a> {
    sort_key: &'a dyn Fn(HashMap<u32, u32>) -> u32,
    contents: Vec<HashMap<u32, u32>>,
    pub sorted: bool,
}

// NOTE Try type generic T
impl <'a> Queue<'a> {
    pub fn new(
        f: &'a dyn Fn(HashMap<u32, u32>) -> u32,
        data: Vec<HashMap<u32, u32>>) -> Self {
        Self {
            sort_key: f,
            contents: data,
            sorted: false,
        }
    }
}
