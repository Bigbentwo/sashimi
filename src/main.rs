use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_512};

mod hash;
use hash::Sashimi;

fn main() {
    let mut hash = Sha3_512::new();
    hash.update("data");
    let hash: [u8; 64] = hash.finalize().as_slice().try_into().unwrap();
    for val in &hash {
        print!("{:X}", val);
    }
    println!("");

    let mut test = Sashimi::new();
    test.update("Hello");
    println!("{:?}", test.finalize());

    let mut rng = ChaCha20Rng::from_entropy();
    println!("{}", rng.gen_range(0..10000));
    println!("{:?}", rng.gen::<[u8; 32]>());

    println!("{:?}", b"test");
}
