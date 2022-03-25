use sha3::{Digest, Sha3_512};
use rand::prelude::*;

fn main() {
    let mut hash = Sha3_512::new();
    //hash.update("data");
    let hash: [u8; 8] = hash.finalize().as_slice().try_into().unwrap();
    for val in &hash {
        print!("{:X}", val);
    }
    println!("");

    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
    println!("{}", rng.gen_range(0..10000));
    println!("{:?}", rng.gen::<[u8; 32]>());

    println!("{:?}", b"test");
}