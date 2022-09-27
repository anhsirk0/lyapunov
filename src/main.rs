mod args;
mod lyapunov;

use args::LyapunovArgs;
use clap::Parser;
use lyapunov::Lyapunov;
use std::process;

fn main() {
    let args = LyapunovArgs::parse();

    let sequence = args.sequence.to_lowercase();
    let sizes: Vec<u32> = args
        .resolution
        .split("x")
        .map(|s| s.parse().unwrap())
        .collect();

    if sizes.len() != 2 {
        println!("Resolution must be of format '800x800'");
        process::exit(1)
    }

    let width = sizes[0];
    let height = sizes[1];

    let l = Lyapunov::new(sequence, args.n, width, height, args.output);
    l.generate_image()
}
