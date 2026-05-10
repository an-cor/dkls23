use anyhow::{anyhow, Result};
use dkls_metrics::tcp_relay_connection::TcpRelayConnection;
use sl_dkls23::keygen::{self, utils::setup_keygen};
use std::{env, time::Instant};

#[tokio::main]
async fn main() -> Result<()> {
    let id: u8 = required_arg("--id")?.parse()?;
    let n: u8 = required_arg("--n")?.parse()?;
    let t: u8 = required_arg("--t")?.parse()?;
    let relay_addr = required_arg("--relay")?;
    let run_id = arg_value("--run-id").unwrap_or_else(|| "dkls_dkg_test".to_string());

    if id == 0 || id > n {
        return Err(anyhow!("party id must be in range 1..=n"));
    }

    println!(
        "dkls_party starting: id={} n={} t={} relay={} run_id={}",
        id, n, t, relay_addr, run_id
    );

    let instance = [7u8; 32];

    let setups = setup_keygen(Some(instance), t, n, None);

    let index = (id - 1) as usize;
    let (setup, seed) = setups
        .into_iter()
        .nth(index)
        .ok_or_else(|| anyhow!("missing setup for party {}", id))?;

    let relay = TcpRelayConnection::connect(id as u32, relay_addr, run_id.clone()).await?;

    let start = Instant::now();

    let keyshare = keygen::run(setup, seed, relay).await?;

    let elapsed = start.elapsed();

    println!(
        "dkls_party complete: id={} key_id={} elapsed_ms={}",
        id,
        hex::encode(keyshare.key_id),
        elapsed.as_millis()
    );

    let out_dir = format!("/home/exouser/socioty-results/dkls/{}", run_id);
    std::fs::create_dir_all(&out_dir)?;

    let share_path = format!("{}/party-{}.share", out_dir, id);
    std::fs::write(&share_path, keyshare.as_slice())?;

    let metrics_path = format!("{}/party-{}.metrics.txt", out_dir, id);
    std::fs::write(
        &metrics_path,
        format!(
            "party_id={}\nn={}\nt={}\nrun_id={}\nkey_id={}\nelapsed_ms={}\n",
            id,
            n,
            t,
            run_id,
            hex::encode(keyshare.key_id),
            elapsed.as_millis()
        ),
    )?;

    println!("wrote share to {}", share_path);
    println!("wrote metrics to {}", metrics_path);

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
