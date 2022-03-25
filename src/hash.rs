use sha3::{Digest, Sha3_512};

pub struct Sashimi {
    buffer: Vec<[u8; 64]>,
    hash: Sha3_512,
    cnt: u64,
}

impl Sashimi {
    pub fn new() -> Self {
        Sashimi {
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
    pub fn finalize(&mut self) -> [u8; 64] {
        self.hash.update(self.get_cnt());
        self.cnt += 1;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn keccak_impl_test() {
        assert_eq!(2 + 2, 4);
    }
}
