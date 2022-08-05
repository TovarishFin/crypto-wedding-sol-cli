use crate::util;
use anchor_client::{Client, Cluster};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
    signer::{keypair::Keypair, null_signer::NullSigner, Signer},
};
use std::error::Error;
use std::{env, rc::Rc};

pub const RPC_URL: &str = "https://api.devnet.solana.com";
pub const WS_URL: &str = "wss://api.devnet.solana.com";

pub fn get_client(signer: Option<&Keypair>) -> Client {
    match signer {
        Some(signer) => {
            // TODO: is there not a better way to do this?
            let signer = Keypair::from_bytes(signer.to_bytes().as_ref()).unwrap();
            Client::new_with_options(
                Cluster::Custom(RPC_URL.into(), WS_URL.into()),
                Rc::new(signer),
                CommitmentConfig::confirmed(),
            )
        }
        None => Client::new_with_options(
            Cluster::Custom(RPC_URL.into(), WS_URL.into()),
            Rc::new(NullSigner::new(&Pubkey::new_unique())),
            CommitmentConfig::confirmed(),
        ),
    }
}

pub fn request_airdrop(
    rpc_client: &RpcClient,
    pub_key: &Pubkey,
    amount_sol: u64,
) -> Result<Signature, Box<dyn Error>> {
    let amount = amount_sol * util::LAMPORTS_PER_SOL;
    let sig = rpc_client.request_airdrop(pub_key, amount)?;

    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::finalized())?;

    Ok(sig)
}

pub fn check_airdrop_users() -> Result<(), Box<dyn Error>> {
    let signer = Keypair::from_base58_string(env::var("SIGNER_PRIV")?.as_ref());
    let partner0 = Keypair::from_base58_string(env::var("U_PARTNER0_PRIV")?.as_ref());
    let partner1 = Keypair::from_base58_string(env::var("U_PARTNER1_PRIV")?.as_ref());

    let signer_pub = signer.pubkey().clone();
    let partner0_pub = partner0.pubkey().clone();
    let partner1_pub = partner1.pubkey().clone();

    let rpc_client = RpcClient::new(RPC_URL);

    let balance = rpc_client.get_balance(&signer_pub)?;
    if balance < 2 * util::LAMPORTS_PER_SOL {
        println!("{:?} balance less than 2 lamps: {:?}", signer_pub, balance);
        println!("requesting airdrop...");
        request_airdrop(&rpc_client, &signer_pub, 2)?;

        let balance = rpc_client.get_balance(&signer_pub)?;
        println!("{:?} new balance: {:?}", signer_pub, balance);
    }

    let balance = rpc_client.get_balance(&partner0_pub)?;
    if balance < 2 * util::LAMPORTS_PER_SOL {
        println!(
            "{:?} balance less than 2 lamps: {:?}",
            partner0_pub, balance
        );
        println!("requesting airdrop...");
        request_airdrop(&rpc_client, &partner0_pub, 2)?;

        let balance = rpc_client.get_balance(&partner0_pub)?;
        println!("{:?} new balance: {:?}", partner0_pub, balance);
    }

    let balance = rpc_client.get_balance(&partner1_pub)?;
    if balance < 2 * util::LAMPORTS_PER_SOL {
        println!(
            "{:?} balance less than 2 lamps: {:?}",
            partner1_pub, balance
        );
        println!("requesting airdrop...");
        request_airdrop(&rpc_client, &partner1_pub, 2)?;

        let balance = rpc_client.get_balance(&partner1_pub)?;
        println!("{:?} new balance: {:?}", partner1_pub, balance);
    }

    Ok(())
}
