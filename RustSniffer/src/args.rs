use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List available network interfaces and exit
    #[arg(long)]
    pub list: bool,

    /// Interface to capture on
    #[arg(short, long)]
    pub interface: Option<String>,

    /// Number of packets to capture (0 = unlimited)
    #[arg(short, long, default_value_t = 0)]
    pub count: u64,

    /// Output format: text or json
    #[arg(short, long, default_value = "text")]
    pub output: String,
    /// Simple filter (examples: "tcp", "udp", "port 80") — basic matching only
    #[arg(long)]
    pub filter: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}