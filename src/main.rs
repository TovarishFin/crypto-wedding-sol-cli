use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use crypto_wedding_cli::{actions, network, util};
use dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};
use std::env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)] // Reads these fields from `Cargo.toml`
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GetOwnAccount,
    AirdropFunds,
    CreateAndAirdropAccount, // mostly used for testing...
    SetupWedding(SetupWedding),
    CancelWedding(CancelWedding),
    SetupPartner(SetupPartner),
    ClosePartner(ClosePartner),
    UpdatePartner(UpdatePartner),
    UpdateName(UpdateName),
    UpdateVows(UpdateVows),
    GiveAnswer(GiveAnswer),
    Divorce(Divorce),
    PrintWedding(PrintWedding),
    PrintPartner(PrintPartner),
    WatchWedding(WatchWedding),
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

#[derive(Args)]
struct UpdatePartner {
    #[clap(value_parser, long)]
    other: Pubkey,
    #[clap(value_parser, long)]
    name: String,
    #[clap(value_parser, long)]
    vows: String,
}

#[derive(Args)]
struct UpdateName {
    #[clap(value_parser, long)]
    other: Pubkey,
    #[clap(value_parser, long)]
    name: String,
}

#[derive(Args)]
struct UpdateVows {
    #[clap(value_parser, long)]
    other: Pubkey,
    #[clap(value_parser, long)]
    vows: String,
}

#[derive(Args)]
struct GiveAnswer {
    #[clap(value_parser, long)]
    other: Pubkey,
    #[clap(value_parser, long)]
    say_yes: bool,
}

#[derive(Args)]
struct Divorce {
    #[clap(value_parser, long)]
    other: Pubkey,
}

#[derive(Args)]
struct PrintWedding {
    #[clap(value_parser, long)]
    partner0: Pubkey,
    #[clap(value_parser, long)]
    partner1: Pubkey,
}

#[derive(Args)]
struct PrintPartner {
    #[clap(value_parser, long)]
    partner: Pubkey,
}

#[derive(Args)]
struct WatchWedding {
    #[clap(value_parser, long)]
    partner0: Pubkey,
    #[clap(value_parser, long)]
    partner1: Pubkey,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = format!(
        "{}/.crypto_wedding_cli_env",
        dirs::home_dir().unwrap().to_str().unwrap()
    );

    dotenv::from_path(path)?;
    let signer = Keypair::from_base58_string(env::var("SIGNER_PRIV")?.as_ref());
    println!("operating as: {:?}", signer.pubkey());

    match cli.command {
        Commands::GetOwnAccount => {
            println!("getting own account info...");
            let rpc_client = RpcClient::new(network::RPC_URL);
            let signer_pub = signer.pubkey();
            let balance = rpc_client.get_balance(&signer_pub)?;

            println!("public key: {:?}", signer_pub);
            println!("balance: {:?}", balance / util::LAMPORTS_PER_SOL);
        }
        Commands::AirdropFunds => {
            println!("requesting airdrop...");
            let rpc_client = RpcClient::new(network::RPC_URL);

            let signer_pub = signer.pubkey();
            let sig = network::request_airdrop(&rpc_client, &signer_pub, 2).unwrap();
            let latest = rpc_client.get_latest_blockhash()?;
            rpc_client.confirm_transaction_with_spinner(
                &sig,
                &latest,
                CommitmentConfig::finalized(),
            )?;

            let balance = rpc_client.get_balance(&signer_pub)?;
            println!(
                "{:?} new balance: {:?}",
                signer_pub,
                balance / util::LAMPORTS_PER_SOL
            );
        }
        Commands::CreateAndAirdropAccount => {
            println!("creating account...");
            let account = Keypair::new();
            let account_pub = account.pubkey();
            println!("account created");

            println!("requesting airdrop for new account...");
            let rpc_client = RpcClient::new(network::RPC_URL);

            network::request_airdrop(&rpc_client, &account_pub, 2).unwrap();

            let balance = rpc_client.get_balance(&account_pub)?;
            println!("airdrop completed");

            println!("account created and funded:");
            println!("public key: {:?}", account_pub);
            println!("private key: {:?}", account.to_base58_string());
            println!("balance: {:?}", balance / util::LAMPORTS_PER_SOL);
        }
        Commands::SetupWedding(SetupWedding { partner0, partner1 }) => {
            println!("setting up wedding...");
            let sig = actions::setup_wedding(&signer, &partner0, &partner1)?;

            actions::print_wedding(&partner0, &partner1);

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::CancelWedding(CancelWedding { partner0, partner1 }) => {
            println!("cancelling wedding...");
            let sig = actions::cancel_wedding(&signer, &partner0, &partner1)?;

            actions::print_wedding(&partner0, &partner1);

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::SetupPartner(SetupPartner { other, name, vows }) => {
            println!("setting up partner PDA account...");
            let sig = actions::setup_partner(&signer, &other, &name, &vows)?;

            actions::print_partner(&signer.pubkey());

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::ClosePartner(ClosePartner { other }) => {
            println!("closing partner PDA account...");
            let sig = actions::close_partner(&signer, &other)?;

            actions::print_partner(&signer.pubkey());

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::UpdatePartner(UpdatePartner { other, name, vows }) => {
            println!("updating partner PDA account...");
            let sig = actions::update_partner(&signer, &other, name.as_ref(), vows.as_ref())?;

            actions::print_partner(&signer.pubkey());

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::UpdateName(UpdateName { other, name }) => {
            println!("updating name on partner PDA account...");
            let sig = actions::update_name(&signer, &other, name.as_ref())?;

            actions::print_partner(&signer.pubkey());

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::UpdateVows(UpdateVows { other, vows }) => {
            println!("updating vows on partner PDA account...");
            let sig = actions::update_vows(&signer, &other, vows.as_ref())?;

            actions::print_partner(&signer.pubkey());

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::GiveAnswer(GiveAnswer { other, say_yes }) => {
            println!("giving answer on partner PDA account...");
            let sig = actions::give_answer(&signer, &other, say_yes)?;

            actions::print_partner(&signer.pubkey());

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::Divorce(Divorce { other }) => {
            println!("divorcing...");
            let sig = actions::divorce(&signer, &other)?;

            actions::print_wedding(&signer.pubkey(), &other);

            println!("tx: {:?}", util::get_tx_link(&sig, Some("devnet")));
        }
        Commands::PrintWedding(PrintWedding { partner0, partner1 }) => {
            actions::print_wedding(&partner0, &partner1);
        }
        Commands::PrintPartner(PrintPartner { partner }) => {
            actions::print_partner(&partner);
        }
        Commands::WatchWedding(WatchWedding { partner0, partner1 }) => {
            actions::watch_wedding(&partner0, &partner1)?;
        }
    };

    Ok(())
}
