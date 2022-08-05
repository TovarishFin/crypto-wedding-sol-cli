use crate::{network, util};
use anchor_client::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
    signer::{keypair::Keypair, Signer},
    system_program,
};

pub fn setup_wedding(
    creator: &Keypair,
    partner0: &Pubkey,
    partner1: &Pubkey,
) -> Result<Signature, ClientError> {
    let p_partner0 = util::find_partner_pda(partner0);
    let p_partner1 = util::find_partner_pda(partner1);
    let p_wedding = util::find_wedding_pda(partner0, partner1);

    let prog = util::get_crypto_wedding(Some(creator));
    let sig = prog
        .request()
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
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn cancel_wedding(
    user: &Keypair,
    partner0: &Pubkey,
    partner1: &Pubkey,
) -> Result<Signature, ClientError> {
    let prog = util::get_crypto_wedding(Some(user));

    let p_wedding = util::find_wedding_pda(partner0, partner1);
    let state: crypto_wedding::state::Wedding = prog.account(p_wedding)?;

    let sig = prog
        .request()
        .accounts(crypto_wedding::accounts::CancelWedding {
            user: user.pubkey(),
            creator: state.creator,
            user_partner0: partner0.clone(),
            user_partner1: partner1.clone(),
            wedding: p_wedding,
        })
        .args(crypto_wedding::instruction::CancelWedding {})
        .signer(user)
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn setup_partner(
    user: &Keypair,
    other: &Pubkey,
    name: &str,
    vows: &str,
) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = util::find_partner_pda(&user_pub);
    let p_wedding = util::find_wedding_pda(&user_pub, other);

    let prog = util::get_crypto_wedding(Some(user));
    let sig = prog
        .request()
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
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn close_partner(user: &Keypair, other: &Pubkey) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = util::find_partner_pda(&user_pub);
    let p_wedding = util::find_wedding_pda(&user_pub, other);

    let prog = util::get_crypto_wedding(Some(user));
    let sig = prog
        .request()
        .accounts(crypto_wedding::accounts::ClosePartner {
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            wedding: p_wedding,
        })
        .args(crypto_wedding::instruction::ClosePartner {})
        .signer(user)
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}
