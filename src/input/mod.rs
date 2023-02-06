pub struct Weights();

impl Weights {
    pub fn new() -> Self {
        Weights {}
    }
}

impl Default for Weights {
    fn default() -> Self {
        Self::new()
    }
}
