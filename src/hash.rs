use sha3::{digest::FixedOutputReset, Digest, Sha3_512};

pub struct Sashimi {
    buffer: Vec<[u8; 64]>,
    passwd_hash: Sha3_512,
}

impl Sashimi {
    pub fn new() -> Self {
        Self {
            buffer: Vec::<[u8; 64]>::new(),
            passwd_hash: Sha3_512::new(),
        }
    }
    pub fn update(&mut self, data: impl AsRef<[u8]>) {
        self.passwd_hash.update(data);
    }
    fn int_to_arr(val: u64) -> [u8; 8] {
        unsafe { std::mem::transmute::<u64, [u8; 8]>(val) }
    }
    fn arr_to_int(arr: &[u8; 8]) -> u64 {
        unsafe { std::mem::transmute::<[u8; 8], u64>(*arr) }
    }

    pub fn finalize(
        &mut self,
        salt: impl AsRef<[u8]>,
        s_cost: usize,
        t_cost: usize,
    ) -> [u8; 64] {
        const DELTA: usize = 3;

        let mut salt_hash = Sha3_512::new();

        let mut cnt: u64 = 0;
        self.buffer.resize(s_cost, [0u8; 64]);
        self.passwd_hash.update(Self::int_to_arr(cnt));
        cnt += 1;

        // fill buffer
        self.buffer[0] = self.passwd_hash.finalize_fixed_reset().into();
        for m in 1..s_cost {
            self.passwd_hash.update(Self::int_to_arr(cnt));
            cnt += 1;
            self.passwd_hash.update(self.buffer[m - 1]);
            self.buffer[m] = self.passwd_hash.finalize_fixed_reset().into();
        }

        // mix buffer
        for t in 0..t_cost {
            for m in 0..s_cost {
                // hash last and current block
                self.passwd_hash.update(Self::int_to_arr(cnt));
                cnt += 1;
                self.passwd_hash
                    .update(self.buffer[m.overflowing_sub(1).0 % s_cost]);
                self.passwd_hash.update(self.buffer[m]);
                self.buffer[m] =
                    self.passwd_hash.finalize_fixed_reset().into();

                // this is kind of ugly
                for i in 0..DELTA {
                    self.passwd_hash.update(Self::int_to_arr(cnt));
                    cnt += 1;
                    self.passwd_hash.update(self.buffer[m]);

                    salt_hash.update(Self::int_to_arr(t as u64));
                    salt_hash.update(Self::int_to_arr(m as u64));
                    salt_hash.update(Self::int_to_arr(i as u64));
                    salt_hash.update(Self::int_to_arr(cnt));
                    cnt += 1;
                    salt_hash.update(&salt);

                    let tmp: [u8; 64] =
                        salt_hash.finalize_fixed_reset().into();

                    // I hate this
                    let mut tmp2: [u8; 8] = [0u8; 8];
                    tmp2.copy_from_slice(&tmp[0..8]);

                    let r_index: usize = Self::arr_to_int(&tmp2) as usize;
                    self.passwd_hash.update(self.buffer[r_index % s_cost]);

                    self.buffer[m] =
                        self.passwd_hash.finalize_fixed_reset().into();
                }
            }
        }

        self.buffer[s_cost - 1]
    }

    pub fn reset(&mut self) {
        self.passwd_hash.reset();
        self.buffer.clear();
        //self.buffer.shrink_to_fit(); // might be a good idea, not sure
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use sha3::{digest::FixedOutputReset, Digest, Sha3_512};

    #[test]
    fn keccak_impl_test() {
        let hash = Sha3_512::new();
        // RHS taken from NIST
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

    #[test]
    fn sashimi_flush() {
        let mut s = crate::Sashimi::new();
        s.update("test");
        let h1 = s.passwd_hash.finalize_fixed_reset();
        s.update("test");
        let h2 = s.passwd_hash.finalize_fixed_reset();
        assert_eq!(h1, h2);
    }
}
