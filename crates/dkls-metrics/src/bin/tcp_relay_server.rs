use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelayMessage {
    from: u32,
    to: u32,
    run_id: String,
    payload: String,
}

type Tx = mpsc::UnboundedSender<String>;
type Clients = Arc<Mutex<HashMap<u32, Tx>>>;

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

    let first_line = match lines.next_line().await? {
        Some(line) => line,
        None => return Ok(()),
    };

    let hello: RelayMessage = serde_json::from_str(&first_line)?;
    let party_id = hello.from;

    println!("registered party {} with run_id={}", party_id, hello.run_id);

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    clients.lock().await.insert(party_id, tx);

    let write_task = tokio::spawn(async move {
        while let Some(line) = rx.recv().await {
            if writer.write_all(line.as_bytes()).await.is_err() {
                break;
            }
            if writer.write_all(b"\n").await.is_err() {
                break;
            }
        }
    });

    while let Some(line) = lines.next_line().await? {
        let msg: RelayMessage = match serde_json::from_str(&line) {
            Ok(msg) => msg,
            Err(err) => {
                eprintln!("invalid json from party {}: {:?}; line={}", party_id, err, line);
                continue;
            }
        };

        println!(
            "relay: run_id={} from={} to={} payload={}",
            msg.run_id, msg.from, msg.to, msg.payload
        );

        let maybe_tx = clients.lock().await.get(&msg.to).cloned();

        if let Some(dest_tx) = maybe_tx {
            dest_tx.send(line)?;
        } else {
            eprintln!("destination party {} not connected yet", msg.to);
        }
    }

    clients.lock().await.remove(&party_id);
    write_task.abort();

    println!("party {} disconnected", party_id);

    Ok(())
}

fn arg_value(flag: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
}
