use sha3::{Digest, Sha3_256};

fn main() {
    let mut hash = Sha3_256::new();
    hash.update("data");
    let hash: [u8; 32] = hash.finalize().as_slice().try_into().unwrap();
    println!("{:?}", hash);
}
