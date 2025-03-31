use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Librarian, the media malware scanner")]
pub struct Arguments {
    #[arg(short, long)]
    pub folder: String,

    #[arg(short, long = "entropy-threshold", default_value = "8.0")]
    pub entropy_threshold: f64,

    #[arg(short, long, default_value = "2")]
    pub threads: usize
}
