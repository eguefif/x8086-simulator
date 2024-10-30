pub struct Registers {
    pub eax: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers { eax: 0 }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
