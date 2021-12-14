// https://en.wikipedia.org/wiki/Xorshift
pub struct Random(u32);

impl Random {
    pub fn new(seed: u32) -> Random {
        Random(seed)
    }

    pub fn rand(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}
