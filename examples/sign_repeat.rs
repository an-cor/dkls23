use std::sync::Arc;
use tokio::task::JoinSet;

use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use k256::ecdsa::{RecoveryId, VerifyingKey};

use sl_dkls23::keygen::Keyshare;
use sl_dkls23::sign;
use sl_mpc_mate::coord::SimpleMessageRelay;

mod common;

async fn sign_once(
    shares: &[Arc<Keyshare>],
    chain_path: &str,
    hash: [u8; 32],
) {
    let coord = SimpleMessageRelay::new();
    let mut parties = JoinSet::new();

    for setup in common::shared::setup_dsg(&shares[0..2], chain_path, hash) {
        let mut rng = ChaCha20Rng::from_entropy();
        let relay = coord.connect();
        parties.spawn(sign::run(setup, rng.gen(), relay));
    }

    let vk = VerifyingKey::from_affine(shares[0].public_key().to_affine())
        .unwrap();

    while let Some(fini) = parties.join_next().await {
        let fini = fini.unwrap();

        if let Err(ref err) = fini {
            println!("error: {err:?}");
        }

        let (sig, recid) = fini.unwrap();
        let recid2 =
            RecoveryId::trial_recovery_from_prehash(&vk, &hash, &sig)
                .unwrap();

        assert_eq!(recid, recid2);

        println!("verified signature for hash {}", hex::encode(hash));
        println!("signature: {:?}", sig);
        println!("recovery id: {:?}", recid);
    }
}

#[tokio::main]
async fn main() {
    let shares = common::shared::gen_keyshares(2, 3).await;
    let chain_path = "m";

    let msg_a = [1u8; 32];
    let msg_b = [2u8; 32];

    sign_once(&shares, chain_path, msg_a).await;
    sign_once(&shares, chain_path, msg_b).await;

    println!("done: reused one keygen for two signing sessions");
}
