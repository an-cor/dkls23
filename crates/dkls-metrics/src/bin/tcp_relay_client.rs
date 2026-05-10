use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    time::sleep,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelayMessage {
    from: u32,
    to: u32,
    run_id: String,
    payload: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let id: u32 = required_arg("--id")?.parse()?;
    let relay = required_arg("--relay")?;
    let run_id = arg_value("--run-id").unwrap_or_else(|| "test".to_string());

    let send_to = arg_value("--send-to").map(|v| v.parse::<u32>()).transpose()?;
    let message = arg_value("--message");

    println!("party {} connecting to relay {}", id, relay);

    let stream = TcpStream::connect(&relay).await?;
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    let hello = RelayMessage {
        from: id,
        to: id,
        run_id: run_id.clone(),
        payload: format!("register party {}", id),
    };

    writer
        .write_all(format!("{}\n", serde_json::to_string(&hello)?).as_bytes())
        .await?;

    println!("party {} registered with run_id={}", id, run_id);

    if let (Some(to), Some(payload)) = (send_to, message) {
        sleep(Duration::from_millis(500)).await;

        let msg = RelayMessage {
            from: id,
            to,
            run_id: run_id.clone(),
            payload,
        };

        let line = serde_json::to_string(&msg)?;
        writer.write_all(format!("{}\n", line).as_bytes()).await?;

        println!("party {} sent message to party {}", id, to);
    }

    println!("party {} waiting for messages...", id);

    while let Some(line) = lines.next_line().await? {
        let msg: RelayMessage = serde_json::from_str(&line)?;

        println!(
            "party {} received: run_id={} from={} to={} payload={}",
            id, msg.run_id, msg.from, msg.to, msg.payload
        );
    }

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
