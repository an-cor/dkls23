use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use msg_relay::MsgRelay;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Barrier, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelayWireMessage {
    from: u32,
    run_id: String,
    payload: Option<String>,
    payload_b64: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let bind_addr = arg_value("--bind").unwrap_or_else(|| "0.0.0.0:9100".to_string());
    let expected_parties: usize = arg_value("--expected-parties")
        .unwrap_or_else(|| "3".to_string())
        .parse()?;

    let listener = TcpListener::bind(&bind_addr).await?;
    let relay = Arc::new(MsgRelay::new(None));
    let barrier = Arc::new(Barrier::new(expected_parties));
    let registered = Arc::new(Mutex::new(0usize));

    println!(
        "tcp_relay_server listening on {} expected_parties={}",
        bind_addr, expected_parties
    );

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accepted connection from {}", addr);

        let relay = relay.clone();
        let barrier = barrier.clone();
        let registered = registered.clone();

        tokio::spawn(async move {
            if let Err(err) = handle_client(stream, relay, barrier, registered).await {
                eprintln!("client error: {:?}", err);
            }
        });
    }
}

async fn handle_client(
    stream: TcpStream,
    relay: Arc<MsgRelay>,
    barrier: Arc<Barrier>,
    registered: Arc<Mutex<usize>>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    let first_line = lines
        .next_line()
        .await?
        .ok_or_else(|| anyhow!("client disconnected before registration"))?;

    let hello: RelayWireMessage = serde_json::from_str(&first_line)?;
    let party_id = hello.from;
    let run_id = hello.run_id.clone();

    {
        let mut count = registered.lock().await;
        *count += 1;
        println!(
            "registered party {} with run_id={} count={}",
            party_id, run_id, *count
        );
    }

    let wait_result = barrier.wait().await;

    if wait_result.is_leader() {
        println!("all expected parties registered; releasing protocol barrier");
    }

    let mut relay_conn = relay.connect();

    loop {
        tokio::select! {
            maybe_line = lines.next_line() => {
                let maybe_line = maybe_line?;

                let Some(line) = maybe_line else {
                    println!("party {} disconnected", party_id);
                    break;
                };

                let wire: RelayWireMessage = match serde_json::from_str(&line) {
                    Ok(wire) => wire,
                    Err(err) => {
                        eprintln!("invalid json from party {}: {:?}", party_id, err);
                        continue;
                    }
                };

                if let Some(text) = wire.payload {
                    println!(
                        "debug text from party {} run_id={}: {}",
                        party_id, wire.run_id, text
                    );
                }

                if let Some(payload_b64) = wire.payload_b64 {
                    match general_purpose::STANDARD.decode(payload_b64) {
                        Ok(bytes) => {
                            println!(
                                "relay inbound: run_id={} from={} bytes={}",
                                wire.run_id,
                                wire.from,
                                bytes.len()
                            );

                            relay_conn.send_message(bytes);
                        }
                        Err(err) => {
                            eprintln!("base64 decode error from party {}: {:?}", party_id, err);
                        }
                    }
                }
            }

            maybe_msg = relay_conn.recv() => {
                let Some(bytes) = maybe_msg else {
                    println!("relay connection closed for party {}", party_id);
                    break;
                };

                println!(
                    "relay outbound: run_id={} to_party={} bytes={}",
                    run_id,
                    party_id,
                    bytes.len()
                );

                let wire = RelayWireMessage {
                    from: 0,
                    run_id: run_id.clone(),
                    payload: None,
                    payload_b64: Some(general_purpose::STANDARD.encode(bytes)),
                };

                let line = serde_json::to_string(&wire)?;
                writer.write_all(line.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        }
    }

    Ok(())
}

fn arg_value(flag: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
}
