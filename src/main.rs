// src/main.rs
/*
 * Main executable for KernelTestnetKitSDKX
 */

use clap::Parser;
use kerneltestnetkitsdkx::{Result, run};

#[derive(Parser)]
#[command(version, about = "KernelTestnetKitSDKX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
