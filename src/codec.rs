use std::ops::{AddAssign, ShlAssign};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+~";
const BLOCK_SIZE: usize = 6;
const BYTE_SIZE: usize = 8;

pub struct Encoder {
    encoded: String,
    value: u8,
    index: usize,
}

pub struct Decoder {
    buffer: Vec<u8>,
    byte: usize,
    bit: usize,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder { encoded: String::new(), value: 0, index: 0 }
    }

    fn encode(&mut self) {
        self.value <<= BLOCK_SIZE - self.index;
        let character = ALPHABET.chars().nth(self.value as usize).unwrap();
        self.encoded.push(character);
        self.value = 0;
        self.index = 0;
    }

    pub fn result(mut self) -> String {
        self.encode();
        if self.value != 0 {
            self.encode();
        }
        self.encoded
    }

    pub fn push<I: Into<i64>>(&mut self, item: I, size: u8) -> Option<()> {
        if size > 64 {
            return None;
        }

        let item = item.into();

        for i in 0..size {
            if self.index == BLOCK_SIZE {
                self.encode();
            }
            self.value <<= 1;
            let shift = size - i - 1;
            self.value += ((item >> shift) & 0b1) as u8;
            self.index += 1;
        }

        Some(())
    }
}

impl Decoder {
    pub fn decode(data: &str) -> Option<Decoder> {
        let index = |character: u8| ALPHABET.find(character as char).unwrap() as u8;
        let mut buffer = Vec::with_capacity(data.len() * BLOCK_SIZE / BYTE_SIZE);

        for block in data.as_bytes().chunks(4) {
            match &block[..] {
                // | aaaaaabb | bbbbcccc | ccdddddd |
                &[a, b, c, d] => {
                    let a = index(a);
                    let b = index(b);
                    let c = index(c);
                    let d = index(d);

                    buffer.push((a << 2) + (b >> 4));
                    buffer.push(((b & 0b1111) << 4) + (c >> 2));
                    buffer.push(((c & 0b11) << 6) + d)
                }
                // | aaaaaabb | bbbbcccc | cc..... .|
                &[a, b, c] => {
                    let a = index(a);
                    let b = index(b);
                    let c = index(c);

                    buffer.push((a << 2) + (b >> 4));
                    buffer.push(((b & 0b1111) << 4) + (c >> 2));
                    buffer.push(c & 0b11);
                }
                // | aaaaaabb | bbbb.... |
                &[a, b] => {
                    let a = index(a);
                    let b = index(b);

                    buffer.push((a << 2) + (b >> 4));
                    buffer.push((b & 0b1111) << 4);
                }
                // | aaaaaa.. |
                &[a] => buffer.push(index(a) << 2),
                _ => unreachable!(),
            }
        }

        Some(Decoder { buffer, byte: 0, bit: 0 })
    }

    fn take_aligned_bytes<T>(&mut self, mut current: T, count: usize) -> T
    where
        T: Default + AddAssign<T> + ShlAssign<usize> + From<u8> + Copy,
    {
        for byte in self.buffer.iter().skip(self.byte).take(count as usize) {
            current <<= BYTE_SIZE - 1;
            current += T::from(*byte);
        }

        self.byte += count;

        current
    }

    fn take_overlapping_bits<T>(&mut self, mut current: T, count: usize) -> T
    where
        T: Default + AddAssign<T> + ShlAssign<usize> + From<u8> + Copy,
    {
        current <<= count;
        let mask = 2_u8.pow(count as u32) - 1;
        let data = self.buffer[self.byte] & mask;
        current += T::from(data);

        // and go to then next byte
        self.byte += 1;
        self.bit = 0;

        current
    }

    fn take_bits<T>(&mut self, mut current: T, count: usize) -> T
    where
        T: Default + AddAssign<T> + ShlAssign<usize> + From<u8> + Copy,
    {
        current <<= count;
        let mask = 2_u8.pow(count as u32) - 1;
        let shift = (BYTE_SIZE - count - self.bit) as u8;
        let data = (self.buffer[self.byte] >> shift) & mask;
        current += T::from(data);

        self.bit += count;

        current
    }

    pub fn take<T>(&mut self, size: u8) -> Option<T>
    where
        T: Default + AddAssign<T> + ShlAssign<usize> + From<u8> + Copy,
    {
        let size = size as usize;
        if size > 64 {
            return None;
        }

        let mut copy_bytes = size / BYTE_SIZE;
        let mut copy_bits = size % BYTE_SIZE;
        let mut result = T::default();

        // copy last bits of current byte
        if copy_bytes > 0 && self.bit > 0 {
            let take_bits = BYTE_SIZE - self.bit;
            result = self.take_bits(result, take_bits);
            let new_size = size - take_bits;
            copy_bytes = new_size / BYTE_SIZE;
            copy_bits = new_size % BYTE_SIZE;
            self.bit = 0;
            self.byte += 1;
        }

        // copy full bytes
        result = self.take_aligned_bytes(result, copy_bytes);

        // copy bits with overlapping
        if self.bit + copy_bits >= BYTE_SIZE {
            // copy last bits of current byte
            let last_bits = BYTE_SIZE - self.bit;
            result = self.take_overlapping_bits(result, last_bits);
            copy_bits -= last_bits;
        }

        // copy bits of current byte
        if copy_bits > 0 {
            result = self.take_bits(result, copy_bits);
        }

        Some(result)
    }

    #[allow(dead_code)]
    pub fn skip(&mut self, size: usize) {
        self.byte += size / BYTE_SIZE;
        self.bit += size % BYTE_SIZE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_6bits() {
        let mut encoder = Encoder::new();
        encoder.push(0b111111, 6).unwrap();
        assert_eq!(encoder.result(), "~".to_owned());
    }

    #[test]
    fn serialize_8bits() {
        let mut encoder = Encoder::new();
        encoder.push(0b11111100, 8).unwrap();
        assert_eq!(encoder.result(), "~a".to_owned());
    }

    #[test]
    fn serialize_16bits() {
        let mut encoder = Encoder::new();
        encoder.push(0b111111000000, 12).unwrap();
        assert_eq!(encoder.result(), "~a".to_owned());
    }

    #[test]
    fn serialize_6_empty_bits() {
        let mut encoder = Encoder::new();
        encoder.push(0, 6).unwrap();
        assert_eq!(encoder.result(), "a".to_owned());
    }

    #[test]
    fn serialize_full_byte() {
        let mut encoder = Encoder::new();
        encoder.push(42, 8).unwrap();
        assert_eq!(encoder.result(), "kG".to_owned());
    }

    #[test]
    fn serialize_6bit_int() {
        let mut encoder = Encoder::new();
        encoder.push(42, 6).unwrap();
        assert_eq!(encoder.result(), "Q".to_owned());
    }

    #[test]
    fn to_many_to_push() {
        let mut encoder = Encoder::new();
        assert_eq!(encoder.push(0, 65), None);
    }

    #[test]
    fn deserialize_6_set_bits() {
        let mut decoder = Decoder::decode("~").unwrap();
        assert_eq!(decoder.take::<u8>(6), Some(0b111111));
    }

    #[test]
    fn deserialize_6_empty_bits() {
        let mut decoder = Decoder::decode("a").unwrap();
        assert_eq!(decoder.take::<u8>(6), Some(0));
    }

    #[test]
    fn deserialize_full_byte() {
        let mut decoder = Decoder::decode("kG").unwrap();
        assert_eq!(decoder.take::<u8>(8), Some(42));
    }

    #[test]
    fn golden_path() {
        for store_bits in [6, 8, 16, 24, 32] {
            let original = vec![4, 8, 15, 16, 23, 42];

            let mut encoder = Encoder::new();
            for item in &original {
                encoder.push(*item, store_bits);
            }
            let result = encoder.result();

            let mut decoder = Decoder::decode(&result).unwrap();
            let mut deserialized = Vec::with_capacity(original.len());
            for _ in 0..original.len() {
                deserialized.push(decoder.take::<i32>(store_bits).unwrap());
            }

            assert_eq!(original, deserialized, "store_bits = {store_bits}");
        }
    }

    #[test]
    fn check_pattern() {
        let pattern_size = 6;
        let patterns = vec![
            (0b101010, vec![1, 0, 1, 0, 1, 0]),
            (0b010101, vec![0, 1, 0, 1, 0, 1]),
            (0b110011, vec![1, 1, 0, 0, 1, 1]),
            (0b001100, vec![0, 0, 1, 1, 0, 0]),
        ];

        for (pattern, expected) in patterns {
            let mut encoder = Encoder::new();
            encoder.push(pattern, pattern_size);
            let result = encoder.result();

            let mut decoder = Decoder::decode(&result).unwrap();
            let mut result = Vec::new();
            for _ in 0..pattern_size {
                result.push(decoder.take::<u8>(1).unwrap());
            }

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn unaligned_data() {
        let sizes = vec![1, 6, 16, 32];
        let expected = vec![1, 42, 420, 4444];

        let mut encoder = Encoder::new();
        for (value, size) in expected.iter().zip(&sizes) {
            encoder.push(*value, *size);
        }
        let encoded = encoder.result();

        let mut decoder = Decoder::decode(&encoded).unwrap();
        let mut result = vec![];
        for size in sizes {
            result.push(decoder.take::<i32>(size).unwrap());
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn skip_bits() {
        let mut encoder = Encoder::new();
        encoder.push(0b10100101, 8);
        let encoded = encoder.result();

        let mut decoder = Decoder::decode(&encoded).unwrap();
        decoder.skip(4);

        assert_eq!(decoder.take(4), Some(0b0101));
    }
}
