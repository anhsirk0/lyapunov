use clap::Parser;

/// Lyapunov fractal generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(allow_negative_numbers = true)]
#[clap(allow_hyphen_values = true)]
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

    /// Dimensions of generated image
    #[clap(short, long, value_parser, default_value = "1000x1000")]
    pub dimensions: String,

    /// Domain (x axis) min and max value
    #[clap(short, long, value_parser, default_value = "0,4")]
    pub x: String,

    /// Range (y axis) min and max value
    #[clap(short, long, value_parser, default_value = "0,4")]
    pub y: String,
}
