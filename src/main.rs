mod cipher;
mod hash;

use cipher::{Settings, Stream};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    ///Run benchmarking suite
    #[clap(short, long)]
    benchmark: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.benchmark {
        run_bench();
        return;
    };

    let s = String::from(
        "This is clear text lorem ipsum something something long text make longer etc.",
    );
    let mut msg = bincode::serialize(&s).unwrap();

    println!("Message is: {:?}", msg);
    let mut password =
        rpassword::prompt_password("Encryption password: ").unwrap();

    let pb = ProgressBar::new(msg.len() as u64);
    pb.tick();

    let set = cipher::Settings::new(10usize.pow(2), 60);
    let mut stream = cipher::Stream::from_settings(password, set);

    for i in 0..msg.len() {
        pb.inc(1);
        stream.apply(&mut msg[i..=i]);
    }
    //stream.apply(&mut msg);

    pb.finish_with_message("done");

    println!("Turned into: {:?}", msg);
    password = rpassword::prompt_password("Decryption password: ").unwrap();

    let mut stream =
        cipher::Stream::from_settings(password, stream.get_settings());
    stream.apply(&mut msg);

    println!("Decoded to: {:?}", msg);
}

fn run_bench() {
    println!("Bench");

    bench(8usize.pow(8), 10, 3);
}

fn bench(
    msg_size: usize,
    s_cost: usize,
    t_cost: usize,
) -> std::time::Duration {
    let mut v1 = Vec::<u8>::with_capacity(msg_size);
    for _ in 0..msg_size {
        v1.push(rand::random());
    }
    let pb = ProgressBar::new(msg_size as u64);
    pb.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) ({eta})"));
    let now = std::time::Instant::now();
    let mut stream =
        Stream::from_settings("abc", Settings::new(s_cost, t_cost));
    for i in 0..v1.len() {
        stream.apply(&mut v1[i..=i]);
        pb.inc(1);
    }
    pb.finish_with_message("done");
    now.elapsed()
}
