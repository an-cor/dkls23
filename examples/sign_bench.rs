use std::sync::Arc;
use std::time::Instant;

use tokio::task::JoinSet;

use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use k256::ecdsa::{RecoveryId, VerifyingKey};

use sl_dkls23::keygen::Keyshare;
use sl_dkls23::sign;
use sl_mpc_mate::coord::SimpleMessageRelay;

mod common;

#[derive(Clone, Copy, Debug)]
enum MessageMode {
    Fixed,
    Random,
}

async fn sign_once(
    shares: &[Arc<Keyshare>],
    t: usize,
    chain_path: &str,
    hash: [u8; 32],
) {
    let coord = SimpleMessageRelay::new();
    let mut parties = JoinSet::new();

    for setup in common::shared::setup_dsg(&shares[0..t], chain_path, hash) {
        let mut rng = ChaCha20Rng::from_entropy();
        let relay = coord.connect();
        parties.spawn(sign::run(setup, rng.gen(), relay));
    }

    let vk = VerifyingKey::from_affine(shares[0].public_key().to_affine()).unwrap();

    let mut checked_one_result = false;

    while let Some(fini) = parties.join_next().await {
        let fini = fini.unwrap();

        if let Err(ref err) = fini {
            println!("error: {err:?}");
        }

        let (sig, recid) = fini.unwrap();
        let recid2 =
            RecoveryId::trial_recovery_from_prehash(&vk, &hash, &sig).unwrap();
        assert_eq!(recid, recid2);

        // only print/check once per signing session summary if you want less noise
        if !checked_one_result {
            checked_one_result = true;
        }
    }
}

fn make_hash(mode: MessageMode, i: usize) -> [u8; 32] {
    match mode {
        MessageMode::Fixed => {
            // deterministic but different per signature
            let byte = ((i % 255) + 1) as u8;
            [byte; 32]
        }
        MessageMode::Random => {
            let mut rng = ChaCha20Rng::seed_from_u64(1000 + i as u64);
            let mut out = [0u8; 32];
            rng.fill(&mut out);
            out
        }
    }
}

async fn run_case(n: usize, t: usize, signatures: usize, mode: MessageMode) {
    let chain_path = "m";

    let dkg_start = Instant::now();
    let shares = common::shared::gen_keyshares(t as u8, n as u8).await;
    let dkg_time = dkg_start.elapsed();

    let sign_start = Instant::now();

    for i in 0..signatures {
        let hash = make_hash(mode, i);
        sign_once(&shares, t, chain_path, hash).await;
    }

    let sign_time = sign_start.elapsed();
    let avg_sign_time = sign_time.as_secs_f64() / signatures as f64;

    println!();
    println!("==============================");
    println!("n={n} t={t} sigs={signatures} mode={mode:?}");
    println!("dkg_time_s={:.6}", dkg_time.as_secs_f64());
    println!("total_sign_time_s={:.6}", sign_time.as_secs_f64());
    println!("avg_sign_time_s={:.6}", avg_sign_time);
    println!("==============================");
    println!();
}

#[tokio::main]
async fn main() {
    let cases = vec![
        (3, 2, 1, MessageMode::Fixed),
        (3, 2, 5, MessageMode::Fixed),
        (3, 2, 10, MessageMode::Fixed),
        (5, 3, 1, MessageMode::Fixed),
        (5, 3, 5, MessageMode::Fixed),
        (5, 3, 10, MessageMode::Fixed),
        // optional second mode
        (3, 2, 1, MessageMode::Random),
        (3, 2, 5, MessageMode::Random),
        (3, 2, 10, MessageMode::Random),
        (5, 3, 1, MessageMode::Random),
        (5, 3, 5, MessageMode::Random),
        (5, 3, 10, MessageMode::Random),
    ];

    for (n, t, signatures, mode) in cases {
        run_case(n, t, signatures, mode).await;
    }
}