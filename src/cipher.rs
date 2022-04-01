mod stream {
    use crate::hash;
    fn raw_encrypt_in_place(
        data: &mut [u8],
        key: impl AsRef<[u8]>,
        salt: &[u8; 64],
        s_cost: usize,
        t_cost: usize,
    ) -> [u8; 64] {
        let mut stream = hash::Sashimi::new();
        stream.update(key);
        let mut mask = stream.finalize(salt, s_cost, t_cost);
        let mut cnt: usize = 0;
        for byte in data {
            *byte = *byte ^ mask[cnt];
            cnt += 1;
            if cnt >= mask.len() {
                stream.reset();
                stream.update(mask);
                mask = stream.finalize(salt, s_cost, t_cost);
                cnt = 0;
            }
        }
        salt.clone()
    }

    fn raw_decrypt_in_place(
        data: &mut [u8],
        key: impl AsRef<[u8]>,
        salt: &[u8; 64],
        s_cost: usize,
        t_cost: usize,
    ) -> [u8; 64] {
        raw_encrypt_in_place(data, key, salt, s_cost, t_cost)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn raw_encrypt() {
            let orig: Vec<u8> = (1..128).collect::<Vec<u8>>();
            let mut data = orig.clone();
            let salt = [6; 64];
            raw_encrypt_in_place(data.as_mut_slice(), "key", &salt, 16, 2);
            raw_decrypt_in_place(data.as_mut_slice(), "key", &salt, 16, 2);
            assert_eq!(orig, data);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn import_path_test() {
        let _t = hash::Sashimi::new();
    }
}
