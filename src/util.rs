use crate::network;
use anchor_client::Program;
use solana_sdk::{pubkey::Pubkey, signature::Signature, signer::keypair::Keypair};

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

pub fn get_crypto_wedding(signer: Option<&Keypair>) -> Program {
    let client = network::get_client(signer);
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

pub fn get_tx_link(sig: &Signature, cluster: Option<&str>) -> String {
    match cluster {
        None => {
            format!("https://explorer.solana.com/tx/{}", sig)
        }
        Some(network) => {
            format!("https://explorer.solana.com/tx/{}?cluster={}", sig, network)
        }
    }
}
