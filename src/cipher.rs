use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use crate::hash;

#[derive(Clone)]
pub struct Settings {
    salt: [u8; 64],
    s_cost: usize,
    t_cost: usize,
}

pub struct SaltedKey {
    data: Vec<u8>,
}

impl SaltedKey {
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        let mut h = hash::Balloon::new();
        let settings = Settings::defaults_for_key();
        h.update(key);
        h.update(&settings.salt);
        Self {
            // I should set the defaults somewhere else
            data: h
                .finalize(settings.salt, settings.s_cost, settings.t_cost)
                .to_vec(),
        }
    }
}

impl Settings {
    // Generate new object with salt from entropy
    pub fn new(s_cost: usize, t_cost: usize) -> Self {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut salt = [0u8; 64];
        rng.fill(&mut salt);
        Self {
            salt,
            s_cost,
            t_cost,
        }
    }
    pub fn defaults_for_stream() -> Self {
        const S_COST: usize = 16;
        const T_COST: usize = 2;
        Self::new(S_COST, T_COST)
    }
    pub fn defaults_for_key() -> Self {
        const S_COST: usize = 20000; // results in around 1.3 MB
        const T_COST: usize = 4;
        Self::new(S_COST, T_COST)
    }
}

pub struct Stream {
    settings: Settings,
    stream: hash::Balloon,
    mask: [u8; 64],
    cnt: usize,
}

impl Stream {
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        let settings = Settings::defaults_for_stream();
        Self::from_settings(key, settings)
    }
    pub fn from_settings(key: impl AsRef<[u8]>, settings: Settings) -> Self {
        let mut stream = hash::Balloon::new();
        stream.update(key);
        let mask =
            stream.finalize(settings.salt, settings.s_cost, settings.t_cost);
        Self {
            settings,
            stream,
            mask,
            cnt: 0,
        }
    }
    pub fn apply(&mut self, mut data: impl AsMut<[u8]>) {
        for byte in data.as_mut() {
            // generate new mask
            if self.cnt >= self.mask.len() {
                self.stream.reset(); // I think this is broken
                self.stream.update(self.mask);
                self.mask = self.stream.finalize(
                    self.settings.salt,
                    self.settings.s_cost,
                    self.settings.t_cost,
                );
                self.cnt = 0;
            }
            *byte = *byte ^ self.mask[self.cnt];
            self.cnt += 1;
        }
    }
    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::SaltedKey;
    use super::Settings;
    use super::Stream;
    use crate::hash;

    #[test]
    fn import_path_test() {
        let _t = hash::Balloon::new();
    }

    #[test]
    fn encrypt_in_place() {
        let orig: Vec<u8> = (0..128).collect::<Vec<u8>>();
        let mut data = orig.clone();
        let settings = Settings::new(16, 3);
        let mut stream1 =
            Stream::from_settings("Super secret password", settings.clone());
        let mut stream2 =
            Stream::from_settings("Super secret password", settings.clone());
        stream1.apply(&mut data[..]);
        stream2.apply(&mut data[..]);
        assert_eq!(orig, data);
    }

    #[test]
    fn strings() {
        let orig: Vec<u8> = "abc".into();
        let mut data = orig.clone();
        let settings = Settings::new(16, 3);
        let mut stream =
            Stream::from_settings("Super secret password", settings);
        stream.apply(&mut data);
        assert_ne!(orig, data);
    }

    #[test]
    fn salt_random() {
        let s1 = Settings::defaults_for_stream();
        let s2 = Settings::defaults_for_stream();
        assert_ne!(s1.salt, s2.salt);
    }

    #[test]
    fn key_derive() {
        let _key = SaltedKey::new("Password");
    }
}
