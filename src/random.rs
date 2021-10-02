// https://en.wikipedia.org/wiki/Xorshift
pub struct Random {
    a: u32,
}

impl Random {
    pub fn new(seed: u32) -> Random {
        Random { a: seed }
    }

    pub fn rand(&mut self) -> u32 {
        self.a ^= self.a << 13;
        self.a ^= self.a >> 17;
        self.a ^= self.a << 5;
        self.a
    }
}
