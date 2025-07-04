use std::collections::HashMap;


pub struct Queue<'a> {
    sort_key: &'a dyn Fn(HashMap<u32, u32>) -> u32,
    contents: HashMap<u32, u32>,
    sorted: bool,
}

// NOTE Try type generic T
impl <'a> Queue<'a> {
    pub fn new(
        f: &'a dyn Fn(HashMap<u32, u32>) -> u32,
        data: HashMap<u32, u32>) -> Self {
        Self {
            sort_key: f,
            contents: data,
            sorted: false,
        }
    }
    // NOTE Try implementing count by iterating over hash index
    pub fn count(&mut self) -> u32 {
        self.sorted = true;
        (self.sort_key)(self.contents.clone())
    }
}


#[cfg(test)]
mod test_Queue {
    use super::*;
    }

