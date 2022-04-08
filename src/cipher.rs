use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use crate::hash;

#[derive(Clone)]
pub struct Settings {
    salt: [u8; 64],
    s_cost: usize,
    t_cost: usize,
}

impl Settings {
    // Generate new object with salt from entropy
    pub fn new(s_cost: usize, t_cost: usize) -> Self {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut salt = [0u8; 64];
        rng.fill(&mut salt);
        Settings {
            salt,
            s_cost,
            t_cost,
        }
    }
    pub fn new_with_default() -> Self {
        const S_COST: usize = 16;
        const T_COST: usize = 2;
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
        let settings = Settings::new_with_default();
        Self::from_settings(key, settings)
    }
    pub fn from_settings(key: impl AsRef<[u8]>, settings: Settings) -> Self {
        let mut stream = hash::Balloon::new();
        stream.update(key);
        let mask =
            stream.finalize(settings.salt, settings.s_cost, settings.t_cost);
        Stream {
            settings,
            stream,
            mask,
            cnt: 0,
        }
    }
    pub fn apply(&mut self, data: &mut [u8]) {
        for byte in data {
            if self.cnt >= self.mask.len() {
                self.stream.reset();
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
    fn salt_random() {
        let s1 = Settings::new_with_default();
        let s2 = Settings::new_with_default();
        assert_ne!(s1.salt, s2.salt);
    }
}
