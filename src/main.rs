use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
// use sha3::{Digest, Sha3_512};

mod cipher;
mod hash;
use hash::Sashimi;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();

    let mut rng = ChaCha20Rng::from_entropy();
    let mut salt = [0u8; 64];
    rng.fill(&mut salt);

    println!("salt: {:?}", salt);

    let mut test = Sashimi::new();
    test.update("Hello2");
    println!("sashimi: {:?}", test.finalize(&salt, 100, 300));
}
