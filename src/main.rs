mod args;
mod lyapunov;

use args::LyapunovArgs;
use clap::Parser;
use lyapunov::Lyapunov;
use std::process;

fn main() {
    let args = LyapunovArgs::parse();

    let sequence = args.sequence.to_lowercase();

    let domain: Vec<f32> = args.x.split(",").map(|s| s.parse().unwrap()).collect();
    if domain.len() != 2 {
        println!("Domain must be of format '0,4'");
        process::exit(1)
    }
    let x_min = domain[0];
    let x_max = domain[1];

    let range: Vec<f32> = args.y.split(",").map(|s| s.parse().unwrap()).collect();
    if range.len() != 2 {
        println!("Range must be of format '0,4'");
        process::exit(1)
    }
    let y_min = range[0];
    let y_max = range[1];

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

    let l = Lyapunov::new(
        sequence,
        args.n,
        x_min,
        x_max,
        y_min,
        y_max,
        width,
        height,
        args.output,
    );
    l.generate_image()
}
