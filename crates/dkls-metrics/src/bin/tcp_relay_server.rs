use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelayWireMessage {
    from: u32,
    run_id: String,
    payload: Option<String>,
    payload_b64: Option<String>,
}

type ClientTx = mpsc::UnboundedSender<RelayWireMessage>;
type Clients = Arc<Mutex<HashMap<(String, u32), ClientTx>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let bind_addr = arg_value("--bind").unwrap_or_else(|| "0.0.0.0:9100".to_string());

    let listener = TcpListener::bind(&bind_addr).await?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("tcp_relay_server listening on {}", bind_addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accepted connection from {}", addr);

        let clients = clients.clone();

        tokio::spawn(async move {
            if let Err(err) = handle_client(stream, clients).await {
                eprintln!("client error: {:?}", err);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, clients: Clients) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    let first_line = lines
        .next_line()
        .await?
        .ok_or_else(|| anyhow!("client disconnected before registration"))?;

    let hello: RelayWireMessage = serde_json::from_str(&first_line)?;
    let party_id = hello.from;
    let run_id = hello.run_id.clone();

    println!("registered party {} with run_id={}", party_id, run_id);

    let (tx, mut rx) = mpsc::unbounded_channel::<RelayWireMessage>();

    {
        let mut map = clients.lock().await;
        map.insert((run_id.clone(), party_id), tx);
    }

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

                if let Some(text) = &wire.payload {
                    println!(
                        "debug text from party {} run_id={}: {}",
                        party_id, wire.run_id, text
                    );
                }

                if wire.payload_b64.is_some() {
                    let decoded_len = wire
                        .payload_b64
                        .as_ref()
                        .and_then(|s| general_purpose::STANDARD.decode(s).ok())
                        .map(|b| b.len())
                        .unwrap_or(0);

                    println!(
                        "relay inbound: run_id={} from={} bytes={}",
                        wire.run_id,
                        wire.from,
                        decoded_len
                    );

                    let map = clients.lock().await;

                    for ((client_run_id, client_party_id), client_tx) in map.iter() {
                        if client_run_id == &wire.run_id && *client_party_id != wire.from {
                            println!(
                                "relay broadcast: run_id={} from={} to={} bytes={}",
                                wire.run_id,
                                wire.from,
                                client_party_id,
                                decoded_len
                            );

                            let _ = client_tx.send(wire.clone());
                        }
                    }
                }
            }

            maybe_wire = rx.recv() => {
                let Some(wire) = maybe_wire else {
                    println!("relay channel closed for party {}", party_id);
                    break;
                };

                let line = serde_json::to_string(&wire)?;
                writer.write_all(line.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        }
    }

    {
        let mut map = clients.lock().await;
        map.remove(&(run_id.clone(), party_id));
    }

    Ok(())
}

fn arg_value(flag: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
}
