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

pub fn update_partner(
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
        .accounts(crypto_wedding::accounts::UpdatePartner {
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            wedding: p_wedding,
            system_program: system_program::id(),
        })
        .args(crypto_wedding::instruction::UpdatePartner {
            name: name.to_string(),
            vows: vows.to_string(),
        })
        .signer(user)
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn update_name(user: &Keypair, other: &Pubkey, name: &str) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = util::find_partner_pda(&user_pub);
    let p_wedding = util::find_wedding_pda(&user_pub, other);

    let prog = util::get_crypto_wedding(Some(user));
    let sig = prog
        .request()
        .accounts(crypto_wedding::accounts::UpdateName {
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            wedding: p_wedding,
            system_program: system_program::id(),
        })
        .args(crypto_wedding::instruction::UpdateName {
            name: name.to_string(),
        })
        .signer(user)
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn update_vows(user: &Keypair, other: &Pubkey, vows: &str) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = util::find_partner_pda(&user_pub);
    let p_wedding = util::find_wedding_pda(&user_pub, other);

    let prog = util::get_crypto_wedding(Some(user));
    let sig = prog
        .request()
        .accounts(crypto_wedding::accounts::UpdateVows {
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            wedding: p_wedding,
            system_program: system_program::id(),
        })
        .args(crypto_wedding::instruction::UpdateVows {
            vows: vows.to_string(),
        })
        .signer(user)
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn give_answer(user: &Keypair, other: &Pubkey, answer: bool) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = util::find_partner_pda(&user_pub);
    let p_other_partner = util::find_partner_pda(other);
    let p_wedding = util::find_wedding_pda(&user_pub, other);

    let prog = util::get_crypto_wedding(Some(user));
    let sig = prog
        .request()
        .accounts(crypto_wedding::accounts::GiveAnswer {
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            other_partner: p_other_partner,
            wedding: p_wedding,
            system_program: system_program::id(),
        })
        .args(crypto_wedding::instruction::GiveAnswer { answer })
        .signer(user)
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn divorce(user: &Keypair, other: &Pubkey) -> Result<Signature, ClientError> {
    let user_pub = user.pubkey();
    let p_partner = util::find_partner_pda(&user_pub);
    let p_other_partner = util::find_partner_pda(other);
    let p_wedding = util::find_wedding_pda(&user_pub, other);

    let prog = util::get_crypto_wedding(None);
    let state: crypto_wedding::state::Wedding = prog.account(p_wedding)?;

    let prog = util::get_crypto_wedding(Some(user));
    let sig = prog
        .request()
        .accounts(crypto_wedding::accounts::Divorce {
            creator: state.creator,
            user: user.pubkey(),
            other: other.clone(),
            partner: p_partner,
            other_partner: p_other_partner,
            wedding: p_wedding,
            system_program: system_program::id(),
        })
        .args(crypto_wedding::instruction::Divorce {})
        .signer(user)
        .send()?;

    let rpc_client = RpcClient::new(network::RPC_URL);
    let latest = rpc_client.get_latest_blockhash()?;
    rpc_client.confirm_transaction_with_spinner(&sig, &latest, CommitmentConfig::confirmed())?;

    Ok(sig)
}

pub fn get_wedding_state(
    partner0: &Pubkey,
    partner1: &Pubkey,
) -> Result<crypto_wedding::state::Wedding, ClientError> {
    let p_wedding = util::find_wedding_pda(partner0, partner1);
    let prog = util::get_crypto_wedding(None);
    let state: crypto_wedding::state::Wedding = prog.account(p_wedding)?;

    Ok(state)
}

pub fn get_partner_state(partner: &Pubkey) -> Result<crypto_wedding::state::Partner, ClientError> {
    let p_partner = util::find_partner_pda(partner);
    let prog = util::get_crypto_wedding(None);
    let state: crypto_wedding::state::Partner = prog.account(p_partner)?;
    Ok(state)
}

pub fn print_wedding(user: &Pubkey, other: &Pubkey) {
    match get_wedding_state(user, other) {
        Ok(wedding) => {
            println!("---| wedding state |---");
            println!("creator: {:?}", wedding.creator);
            println!("partner0: {:?}", wedding.partner0);
            println!("partner1: {:?}", wedding.partner1);
            println!("status: {:?}", wedding.status);
            println!("-----------------------");
        }
        Err(err) => {
            println!("---| wedding state |---");
            println!("error getting state: {:?}", err);
            println!("-----------------------");
        }
    }
}

pub fn print_partner(partner: &Pubkey) {
    match get_partner_state(partner) {
        Ok(partner) => {
            println!("---| partner state |---");
            println!("wedding: {:?}", partner.wedding);
            println!("user: {:?}", partner.user);
            println!("name: {:?}", partner.name);
            println!("vows: {:?}", partner.vows);
            println!("answer: {:?}", partner.answer);
            println!("-----------------------");
        }
        Err(err) => {
            println!("---| partner state |---");
            println!("error getting partner state: {:?}", err);
            println!("-----------------------");
        }
    }
}

pub fn watch_wedding(partner0: &Pubkey, partner1: &Pubkey) -> Result<(), ClientError> {
    // wedding storage fields
    let mut last_creator = None;
    let mut last_partner0 = None;
    let mut last_partner1 = None;
    let mut last_status = None;

    // partner0 storage fields
    let mut last_wedding0 = None;
    let mut last_user0 = None;
    let mut last_name0 = None;
    let mut last_vows0 = None;
    let mut last_answer0 = None;

    // partner1 storage fields
    let mut last_wedding1 = None;
    let mut last_user1 = None;
    let mut last_name1 = None;
    let mut last_vows1 = None;
    let mut last_answer1 = None;

    println!("watching wedding and related partner accounts...");
    println!("new state will be printed when updated...");

    // poll for new state every 1000 ms and print if anything is different
    loop {
        match get_wedding_state(partner0, partner1) {
            Ok(crypto_wedding::state::Wedding {
                creator,
                partner0: p_partner0,
                partner1: p_partner1,
                status,
            }) => {
                let current_creator = Some(creator);
                let current_partner0 = Some(p_partner0);
                let current_partner1 = Some(p_partner1);
                let current_status = Some(status);

                if last_creator != current_creator
                    || last_partner0 != current_partner0
                    || last_partner1 != current_partner1
                    || last_status != current_status
                {
                    print_wedding(&partner0, &partner1);
                }

                last_creator = current_creator;
                last_partner0 = current_partner0;
                last_partner1 = current_partner1;
                last_status = current_status;
            }
            Err(_) => {
                if last_creator != None
                    || last_partner0 != None
                    || last_partner1 != None
                    || last_status != None
                {
                    println!("wedding account no longer exists...");
                }

                last_creator = None;
                last_partner0 = None;
                last_partner1 = None;
                last_status = None;
            }
        };

        match get_partner_state(partner0) {
            Ok(crypto_wedding::state::Partner {
                wedding,
                user,
                name,
                vows,
                answer,
            }) => {
                let current_wedding0 = Some(wedding);
                let current_user0 = Some(user);
                let current_name0 = Some(name);
                let current_vows0 = Some(vows);
                let current_answer0 = Some(answer);

                if last_wedding0 != current_wedding0
                    || last_user0 != current_user0
                    || last_name0 != current_name0
                    || last_vows0 != current_vows0
                    || last_answer0 != current_answer0
                {
                    print_partner(&user)
                }

                last_wedding0 = current_wedding0;
                last_user0 = current_user0;
                last_name0 = current_name0;
                last_vows0 = current_vows0;
                last_answer0 = current_answer0;
            }
            Err(_) => {
                if last_wedding0 != None
                    || last_user0 != None
                    || last_name0 != None
                    || last_vows0 != None
                    || last_answer0 != None
                {
                    println!("partner0 account no longer exists...");
                }

                last_wedding0 = None;
                last_user0 = None;
                last_name0 = None;
                last_vows0 = None;
                last_answer0 = None;
            }
        };

        match get_partner_state(partner1) {
            Ok(crypto_wedding::state::Partner {
                wedding,
                user,
                name,
                vows,
                answer,
            }) => {
                let current_wedding1 = Some(wedding);
                let current_user1 = Some(user);
                let current_name1 = Some(name);
                let current_vows1 = Some(vows);
                let current_answer1 = Some(answer);

                if last_wedding1 != current_wedding1
                    || last_user1 != current_user1
                    || last_name1 != current_name1
                    || last_vows1 != current_vows1
                    || last_answer1 != current_answer1
                {
                    print_partner(&user)
                }

                last_wedding1 = current_wedding1;
                last_user1 = current_user1;
                last_name1 = current_name1;
                last_vows1 = current_vows1;
                last_answer1 = current_answer1;
            }
            Err(_) => {
                if last_wedding1 != None
                    || last_user1 != None
                    || last_name1 != None
                    || last_vows1 != None
                    || last_answer1 != None
                {
                    println!("partner1 account no longer exists...");
                }

                last_wedding1 = None;
                last_user1 = None;
                last_name1 = None;
                last_vows1 = None;
                last_answer1 = None;
            }
        };

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
