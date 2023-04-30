use clap::{Parser, Args, Subcommand};

/// Simple program to greet a person
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
    startscan(Options)
}

#[derive(Args)]
struct Options {

    /// Sniff Packets
    #[arg(long)]
    packets: Option<bool>,

    /// Sniff Ports
    #[arg(long)]
    ports: Option<bool>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Cli::parse();
    
    match &args.command {
        Commands::startscan(packets) => {
            println!("'myapp add' was used, name is: {:?}", packets.packets)
        }
    }




    // if args.packets.is_some() && args.startscan {
    //     println!("Scanning for packets...");
    // }

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.startscan)
    // }

}
