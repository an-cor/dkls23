use anyhow::{anyhow, Result};
use dkls_metrics::tcp_relay_connection::TcpRelayConnection;
use sl_dkls23::keygen::Keyshare;
use std::{env, sync::Arc, time::Instant};

#[tokio::main]
async fn main() -> Result<()> {
    let id: u8 = required_arg("--id")?.parse()?;
    let n: u8 = required_arg("--n")?.parse()?;
    let t: u8 = required_arg("--t")?.parse()?;
    let relay_addr = required_arg("--relay")?;
    let run_id = required_arg("--run-id")?;
    let share_path = required_arg("--share")?;

    if id == 0 || id > n {
        return Err(anyhow!("party id must be in range 1..=n"));
    }

    println!(
        "dkls_sign_party starting: id={} n={} t={} relay={} run_id={} share={}",
        id, n, t, relay_addr, run_id, share_path
    );

    // Phase 6B first version: fixed signer set.
    // For n=3,t=2, all three parties sign.
    let signer_ids: Vec<u8> = arg_value("--signer-ids")
        .unwrap_or_else(|| "1,2,3".to_string())
        .split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if !signer_ids.contains(&id) {
        return Err(anyhow!("party {} is not in signer set {:?}", id, signer_ids));
    }

    // Load this party's DKG share.
    // dkls_party wrote keyshare.as_slice(), so use Keyshare::from_bytes().
    let share_bytes = std::fs::read(&share_path)?;
    let my_share = Arc::new(
        Keyshare::from_bytes(&share_bytes)
            .ok_or_else(|| anyhow!("failed to decode keyshare from {}", share_path))?,
    );

    println!(
        "loaded share: id={} key_id={}",
        id,
        hex::encode(my_share.key_id)
    );

    // IMPORTANT:
    // sign::setup_dsg expects the signer subset, not just this party's share.
    // For a distributed signer, each party only owns its own share, but the helper
    // may require the full subset to construct all setup messages.
    //
    // Next step is to inspect crates/dkls-metrics/src/dsg.rs and src/sign/mod.rs
    // to confirm whether we need:
    //   A) all signer shares on every VM, or
    //   B) a way to construct only this party's SetupMessage from its local share.
    //
    // The relay connection is ready for when we wire sign::run.
    let _relay = TcpRelayConnection::connect(id as u32, relay_addr, run_id.clone()).await?;

    let start = Instant::now();

    // TODO Phase 6B:
    // let (setup, seed) = ...
    // let signature = sign::run(setup, seed, relay).await?;

    let elapsed = start.elapsed();

    let out_dir = format!("/home/exouser/socioty-results/dkls/{}", run_id);
    std::fs::create_dir_all(&out_dir)?;

    let metrics_path = format!("{}/party-{}.sign.metrics.txt", out_dir, id);
    std::fs::write(
        &metrics_path,
        format!(
            "party_id={}\nn={}\nt={}\nrun_id={}\nkey_id={}\nsigner_ids={:?}\nelapsed_ms={}\nstatus=skeleton_loaded_share\n",
            id,
            n,
            t,
            run_id,
            hex::encode(my_share.key_id),
            signer_ids,
            elapsed.as_millis()
        ),
    )?;

    println!("wrote sign metrics skeleton to {}", metrics_path);

    Ok(())
}

fn required_arg(flag: &str) -> Result<String> {
    arg_value(flag).ok_or_else(|| anyhow!("missing required argument {}", flag))
}

fn arg_value(flag: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
}
