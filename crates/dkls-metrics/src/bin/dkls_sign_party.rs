use anyhow::{anyhow, Result};
use dkls_metrics::tcp_relay_connection::TcpRelayConnection;
use sl_dkls23::{keygen::Keyshare, sign};
use std::{env, sync::Arc, time::Instant};

#[tokio::main]
async fn main() -> Result<()> {
    let id: u8 = required_arg("--id")?.parse()?;
    let n: u8 = required_arg("--n")?.parse()?;
    let t: u8 = required_arg("--t")?.parse()?;
    let relay_addr = required_arg("--relay")?;
    let run_id = required_arg("--run-id")?;
    let share_dir = required_arg("--share-dir")?;

    if id == 0 || id > n {
        return Err(anyhow!("party id must be in range 1..=n"));
    }

    println!(
        "dkls_sign_party starting: id={} n={} t={} relay={} run_id={} share_dir={}",
        id, n, t, relay_addr, run_id, share_dir
    );

    let signer_ids: Vec<u8> = arg_value("--signer-ids")
        .unwrap_or_else(|| "1,2,3".to_string())
        .split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if !signer_ids.contains(&id) {
        return Err(anyhow!("party {} is not in signer set {:?}", id, signer_ids));
    }

    let out_dir = format!("/home/exouser/socioty-results/dkls/{}", run_id);
    std::fs::create_dir_all(&out_dir)?;

    let mut shares: Vec<Arc<Keyshare>> = Vec::new();

    for signer_id in &signer_ids {
        let path = format!(
            "{}/party-{:02}/party-{}.share",
            share_dir, signer_id, signer_id
        );

        println!("loading signer share {}", path);

        let bytes = std::fs::read(&path)?;
        let share = Arc::new(
            Keyshare::from_bytes(&bytes)
                .ok_or_else(|| anyhow!("failed decoding share {}", path))?,
        );

        shares.push(share);
    }

    println!("loaded {} signer shares", shares.len());

    let key_id = hex::encode(shares[0].key_id);
    println!("signing key_id={}", key_id);

    let start = Instant::now();

    let mut instance = [0u8; 32];

    for (i, byte) in run_id.as_bytes().iter().enumerate() {
        instance[i % 32] ^= *byte;
    }

    let setups = sign::setup_dsg(Some(instance), &shares, "m");
    //let setups = sign::setup_dsg(None, &shares, "m");
    println!("constructed {} DSG setups", setups.len());

    let my_index = signer_ids
        .iter()
        .position(|v| *v == id)
        .ok_or_else(|| anyhow!("could not find my signer index"))?;

    let (setup, seed) = setups
        .into_iter()
        .nth(my_index)
        .ok_or_else(|| anyhow!("missing setup for signer"))?;

    println!("party {} selected DSG setup index {}", id, my_index);

    let relay_party_id = my_index as u32;

    let relay = TcpRelayConnection::connect(relay_party_id, relay_addr, run_id.clone()).await?;

    let signature = sign::run(setup, seed, relay).await?;

    let elapsed = start.elapsed();

    println!("party {} completed signing", id);

    let sig_path = format!("{}/party-{}.signature.txt", out_dir, id);
    std::fs::write(&sig_path, format!("{:?}", signature))?;

    let metrics_path = format!("{}/party-{}.sign.metrics.txt", out_dir, id);
    std::fs::write(
        &metrics_path,
        format!(
            "party_id={}\nn={}\nt={}\nrun_id={}\nkey_id={}\nsigner_ids={:?}\nelapsed_ms={}\nstatus=sign_completed\n",
            id,
            n,
            t,
            run_id,
            key_id,
            signer_ids,
            elapsed.as_millis()
        ),
    )?;

    println!("wrote signature to {}", sig_path);
    println!("wrote sign metrics to {}", metrics_path);

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
