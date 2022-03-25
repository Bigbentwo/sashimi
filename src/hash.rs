use sha3::{digest::FixedOutputReset, Digest, Sha3_512};

pub struct Sashimi {
    buffer: Vec<[u8; 64]>,
    hash: Sha3_512,
    cnt: u64,
}

impl Sashimi {
    pub fn new() -> Self {
        Self {
            buffer: Vec::<[u8; 64]>::new(),
            hash: Sha3_512::new(),
            cnt: 0,
        }
    }
    pub fn update(&mut self, data: impl AsRef<[u8]>) {
        self.hash.update(data);
    }
    fn get_cnt(&self) -> [u8; 8] {
        unsafe { std::mem::transmute::<u64, [u8; 8]>(self.cnt) }
    }
    fn flush_hash(&mut self) -> [u8; 64] {
        let res = self.hash.finalize_fixed_reset();
        res.as_slice().try_into().unwrap()
    }
    pub fn finalize(&mut self, s_cost: usize, t_cost: usize) -> [u8; 64] {
        self.buffer.resize(s_cost, [0u8; 64]);
        self.hash.update(self.get_cnt());
        self.cnt += 1;
        self.buffer[0] = self.flush_hash();
        for m in 1..s_cost {
            self.hash.update(self.get_cnt());
            self.cnt += 1;
            self.hash.update(self.buffer[m - 1]);
        }

        self.buffer[0]
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use sha3::{Digest, Sha3_512};

    #[test]
    fn keccak_impl_test() {
        let hash = Sha3_512::new();
        assert_eq!(
            hash.finalize()[..],
            hex!(
                "A6 9F 73 CC A2 3A 9A C5 C8 B5 67 DC 18 5A 75 6E 97 C9 82 16
                 4F E2 58 59 E0 D1 DC C1 47 5C 80 A6 15 B2 12 3A F1 F5 F9 4C
                 11 E3 E9 40 2C 3A C5 58 F5 00 19 9D 95 B6 D3 E3 01 75 85 86
                 28 1D CD 26"
            )
        );
    }
}
