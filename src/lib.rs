use anchor_client::{Client, ClientError, Cluster, Program};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
    signer::{keypair::Keypair, null_signer::NullSigner, Signer},
    system_program,
};
use std::error::Error;
use std::{env, rc::Rc};

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

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
    let amount = amount_sol * LAMPORTS_PER_SOL;
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
    if balance < 2 * LAMPORTS_PER_SOL {
        println!("{:?} balance less than 2 lamps: {:?}", signer_pub, balance);
        println!("requesting airdrop...");
        request_airdrop(&rpc_client, &signer_pub, 2)?;

        let balance = rpc_client.get_balance(&signer_pub)?;
        println!("{:?} new balance: {:?}", signer_pub, balance);
    }

    let balance = rpc_client.get_balance(&partner0_pub)?;
    if balance < 2 * LAMPORTS_PER_SOL {
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
    if balance < 2 * LAMPORTS_PER_SOL {
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

pub fn get_crypto_wedding(signer: Option<&Keypair>) -> Program {
    let client = get_client(signer);
    client.program(crypto_wedding::ID)
}

pub fn find_wedding_pda(partner_a: &Pubkey, partner_b: &Pubkey) -> Pubkey {
    let (partner0, partner1) = crypto_wedding::util::sort_pubkeys(partner_a, partner_b);
    let (pda, _) = Pubkey::find_program_address(
        &[
            b"wedding".as_ref(),
            &partner0.to_bytes(),
            &partner1.to_bytes(),
        ],
        &crypto_wedding::ID,
    );

    pda
}

pub fn find_partner_pda(partner: &Pubkey) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(
        &[b"partner".as_ref(), &partner.to_bytes()],
        &crypto_wedding::ID,
    );

    pda
}

pub fn setup_wedding(
    creator: &Keypair,
    partner0: &Pubkey,
    partner1: &Pubkey,
) -> Result<Signature, ClientError> {
    let p_partner0 = find_partner_pda(partner0);
    let p_partner1 = find_partner_pda(partner1);
    let p_wedding = find_wedding_pda(partner0, partner1);

    let prog = get_crypto_wedding(Some(creator));
    prog.request()
        .accounts(crypto_wedding::accounts::SetupWedding {
            creator: creator.pubkey(),
            user_partner0: partner0.clone(),
            user_partner1: partner1.clone(),
            wedding: p_wedding,
            partner0: p_partner0,
            partner1: p_partner1,
            system_program: system_program::id(),
        })
        .args(crypto_wedding::instruction::SetupWedding {})
        .signer(creator)
        .send()
}

pub fn cancel_wedding(
    user: &Keypair,
    partner0: &Pubkey,
    partner1: &Pubkey,
) -> Result<Signature, ClientError> {
    let prog = get_crypto_wedding(Some(user));

    let p_wedding = find_wedding_pda(partner0, partner1);
    let state: crypto_wedding::state::Wedding = prog.account(p_wedding)?;

    prog.request()
        .accounts(crypto_wedding::accounts::CancelWedding {
            user: user.pubkey(),
            creator: state.creator, // FIXME: get this from the wedding program
            user_partner0: partner0.clone(),
            user_partner1: partner1.clone(),
            wedding: p_wedding,
        })
        .args(crypto_wedding::instruction::CancelWedding {})
        .signer(user)
        .send()
}

pub fn setup_partner(
    user: &Keypair,
    other: &Pubkey,
    name: &str,
    vows: &str,
) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = find_partner_pda(&user_pub);
    let p_wedding = find_wedding_pda(&user_pub, other);

    let prog = get_crypto_wedding(Some(user));
    prog.request()
        .accounts(crypto_wedding::accounts::SetupPartner {
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            wedding: p_wedding,
            system_program: system_program::ID,
        })
        .args(crypto_wedding::instruction::SetupPartner {
            name: name.into(),
            vows: vows.into(),
        })
        .signer(user)
        .send()
}

pub fn close_partner(user: &Keypair, other: &Pubkey) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = find_partner_pda(&user_pub);
    let p_wedding = find_wedding_pda(&user_pub, other);

    let prog = get_crypto_wedding(Some(user));
    prog.request()
        .accounts(crypto_wedding::accounts::ClosePartner {
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            wedding: p_wedding,
        })
        .args(crypto_wedding::instruction::ClosePartner {})
        .signer(user)
        .send()
}
