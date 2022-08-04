use crypto_wedding_cli::*;
use dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signer::{keypair::Keypair, Signer},
};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let signer = Keypair::from_base58_string(env::var("SIGNER_PRIV")?.as_ref());
    let partner0 = Keypair::from_base58_string(env::var("U_PARTNER0_PRIV")?.as_ref());
    let partner1 = Keypair::from_base58_string(env::var("U_PARTNER1_PRIV")?.as_ref());

    let signer_pub = signer.pubkey().clone();
    let partner0_pub = partner0.pubkey().clone();
    let partner1_pub = partner1.pubkey().clone();
    println!("signer {:?}", signer_pub);
    println!("partner0 {:?}", partner0_pub);
    println!("partner1 {:?}", partner1_pub);
    println!("---");

    let prog = get_crypto_wedding(None);
    let p_wedding = find_wedding_pda(&partner0_pub, &partner1_pub);
    let p_partner0 = find_partner_pda(&partner0_pub);
    let p_partner1 = find_partner_pda(&partner1_pub);

    let rpc_client = RpcClient::new(RPC_URL);

    check_airdrop_users()?;

    let sig = setup_wedding(&partner0, &partner0_pub, &partner1_pub)?;
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    println!("created wedding");

    let s_wedding: crypto_wedding::state::Wedding = prog.account(p_wedding)?;
    println!("s_wedding::creator: {:?}", s_wedding.creator);
    println!("s_wedding::status: {:?}", s_wedding.status);
    println!("s_wedding::partner0: {:?}", s_wedding.partner0);
    println!("s_wedding::partner1: {:?}", s_wedding.partner1);
    println!("---");

    let sig = setup_partner(&partner0, &partner1_pub, "boberino", "i will do stuff")?;
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    println!("setup partner 0");

    let s_partner0: crypto_wedding::state::Partner = prog.account(p_partner0)?;
    println!("s_partner0::wedding: {:?}", s_partner0.wedding);
    println!("s_partner0::user: {:?}", s_partner0.user);
    println!("s_partner0::name: {:?}", s_partner0.name);
    println!("s_partner0::vows: {:?}", s_partner0.vows);
    println!("s_partner0::answer: {:?}", s_partner0.answer);
    println!("---");

    let sig = setup_partner(&partner1, &partner0_pub, "naomi", "i will do stuff too")?;
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    println!("setup partner 1");

    let s_partner1: crypto_wedding::state::Partner = prog.account(p_partner1)?;
    println!("s_partner1::wedding: {:?}", s_partner1.wedding);
    println!("s_partner1::user: {:?}", s_partner1.user);
    println!("s_partner1::name: {:?}", s_partner1.name);
    println!("s_partner1::vows: {:?}", s_partner1.vows);
    println!("s_partner1::answer: {:?}", s_partner1.answer);
    println!("---");

    let sig = cancel_wedding(&partner1, &partner0_pub, &partner1_pub)?;
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    println!("cancelled wedding");
    println!("---");

    let sig = close_partner(&partner0, &partner1_pub)?;
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    println!("closed partner 0");
    println!("---");

    let sig = close_partner(&partner1, &partner0_pub)?;
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    println!("closed partner 1");
    println!("---");

    Ok(())
}
