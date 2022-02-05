use std::time::SystemTime;

use clap::Parser;

mod config;
mod metadata;

/// Generative art program for Solana NFTs
#[derive(Parser, Debug)]
#[clap()]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Generate(Generate),
    Init(Init),
    Verify(Verify),
}

/// Generate artwork and metadata
#[derive(Parser, Debug)]
pub struct Generate {
    /// Whether to use already present metadata to generate art
    #[clap(long)]
    skip_metadata: bool,

    /// Whether to only generate metadata and not the art
    #[clap(long)]
    skip_art: bool,

    /// Location of assets to generate
    #[clap(short, long, default_value = "./assets")]
    assets: String,

    /// Location of configuration file
    #[clap(short, long, default_value = "./assets/config.json")]
    config: String,

    /// Ouput location of generated art
    #[clap(short, long, default_value = "./generated")]
    output: String,
}

/// Initialize assets directory
#[derive(Parser, Debug)]
pub struct Init {
    /// Location of assets folder to initialize
    #[clap(default_value = "./assets")]
    folder: String,

    /// Overwrite assets folder if already exists
    #[clap(long)]
    overwrite: bool,

    /// Create a config.json from an existing assets folder, ignores folder option
    #[clap(long)]
    from_existing: Option<String>,
}

/// Verify generated assets integrity
#[derive(Parser, Debug)]
pub struct Verify {
    /// Location of generated folder to verify
    #[clap(default_value = "./generated")]
    folder: String,
}

fn main() {
    let options = Options::parse();
    println!("Starting generator");
    let now = SystemTime::now();

    match options.subcmd {
        SubCommand::Generate(c) => cmd::generate::handle(c),
        SubCommand::Init(c) => cmd::init::handle(c),
        SubCommand::Verify(c) => cmd::verify::handle(c),
    }

    println!(
        "Generator finished in {:#?}",
        now.elapsed().unwrap_or_default()
    );
}
