use sha3::{Digest, Sha3_256};
use rand::prelude::*;

fn main() {
    let mut hash = Sha3_256::new();
    //hash.update("data");
    let hash: Vec<u8> = hash.finalize()[..].to_vec();
    for val in &hash {
        print!("{:X}", val);
    }
    println!("");

    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
    println!("{}", rng.gen_range(0..10000));
    println!("{:?}", rng.gen::<[u8; 32]>());

    println!("{:?}", b"test");
}