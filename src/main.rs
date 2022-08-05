use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use crypto_wedding_cli::actions;
use dotenv;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use std::env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SetupWedding(SetupWedding),
    CancelWedding(CancelWedding),
    SetupPartner(SetupPartner),
    ClosePartner(ClosePartner),
}

#[derive(Args)]
struct SetupWedding {
    #[clap(value_parser, long)]
    partner0: Pubkey,
    #[clap(value_parser, long)]
    partner1: Pubkey,
}

#[derive(Args)]
struct CancelWedding {
    #[clap(value_parser, long)]
    partner0: Pubkey,
    #[clap(value_parser, long)]
    partner1: Pubkey,
}

#[derive(Args)]
struct SetupPartner {
    #[clap(value_parser, long)]
    other: Pubkey,
    #[clap(value_parser, long)]
    name: String,
    #[clap(value_parser, long)]
    vows: String,
}

#[derive(Args)]
struct ClosePartner {
    #[clap(value_parser, long)]
    other: Pubkey,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    dotenv::dotenv()?;
    let signer = Keypair::from_base58_string(env::var("SIGNER_PRIV")?.as_ref());

    match cli.command {
        Commands::SetupWedding(SetupWedding { partner0, partner1 }) => {
            println!("setup wedding:");
            println!("partner0: {:?}", partner0.to_string());
            println!("partner1: {:?}", partner1.to_string());
            let sig = actions::setup_wedding(&signer, &partner0, &partner1)?;
            println!("tx signature: {:?}", sig.to_string());
        }
        Commands::CancelWedding(CancelWedding { partner0, partner1 }) => {
            println!("cancel wedding:");
            println!("partner0: {:?}", partner0.to_string());
            println!("partner1: {:?}", partner1.to_string());
            let sig = actions::cancel_wedding(&signer, &partner0, &partner1)?;
            println!("tx signature: {:?}", sig.to_string());
        }
        Commands::SetupPartner(SetupPartner { other, name, vows }) => {
            println!("setup partner");
            println!("partner0: {:?}", other.to_string());
            println!("name: {:?}", name);
            println!("vows: {:?}", vows);
            let sig = actions::setup_partner(&signer, &other, &name, &vows)?;
            println!("tx signature: {:?}", sig.to_string());
        }
        Commands::ClosePartner(ClosePartner { other }) => {
            println!("close partner");
            println!("partner0: {:?}", other.to_string());
            let sig = actions::close_partner(&signer, &other)?;
            println!("tx signature: {:?}", sig.to_string());
        }
    };

    Ok(())
}
