#![allow(non_snake_case)]
mod packets;
mod ports;
use clap::{Parser, Args, Subcommand};

/// Sniffy, an all-in-one network sniffer (not official, I guess)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand)]
enum Commands {

    /// Initiate Sniffy 
    Startscan(Options)
}

#[derive(Args)]
struct Options {

    /// Sniff Packets
    #[arg(long)]
    packets: bool,

    /// Sniff Ports
    #[arg(long)]
    ports: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Startscan(Options) => {
            if Options.packets {
                println!("Sniffy is now sniffing packets...");
            }
            else if Options.ports {
                println!("Sniffy is now sniffing ports...");
            }
            else {
                println!("No scan options specified");
            }
        }
    }

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.startscan)
    // }
}
