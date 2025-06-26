pub struct Box {
    pub r_min: u8,
    pub r_max: u8,
    pub g_min: u8,
    pub g_max: u8,
    pub b_min: u8,
    pub b_max: u8,
    // History goes here
}

impl Box {
    pub fn volume(&self) -> u32 {
        (self.r_max - self.r_min) as u32 *
            (self.g_max - self.g_min) as u32 *
            (self.b_max - self.b_min) as u32
    }
}
