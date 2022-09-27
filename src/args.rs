use clap::Parser;

/// Lyapunov fractal generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct LyapunovArgs {
    /// Sequence of 'a' & 'b'
    #[clap(short, long, value_parser, default_value = "ab")]
    pub sequence: String,

    /// Number of iterations for Lyapunov Exponent
    #[clap(short, long, value_parser, default_value = "200")]
    pub n: usize,

    /// Output image
    #[clap(short, long, value_parser, default_value = "output.png")]
    pub output: String,

    /// Resolution of generated image
    #[clap(short, long, value_parser, default_value = "1000x1000")]
    pub resolution: String,
}
