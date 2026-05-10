use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};

use sl_mpc_mate::coord::{MessageSendError, Relay, Sink, Stream};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelayWireMessage {
    from: u32,
    run_id: String,
    payload: Option<String>,
    payload_b64: Option<String>,
}

pub struct TcpRelayConnection {
    outbound_tx: mpsc::UnboundedSender<Vec<u8>>,
    inbound_rx: mpsc::UnboundedReceiver<Vec<u8>>,
}

impl TcpRelayConnection {
    pub async fn connect(
        party_id: u32,
        relay_addr: String,
        run_id: String,
    ) -> Result<Self> {
        let stream = TcpStream::connect(&relay_addr).await?;
        let (reader, mut writer) = stream.into_split();

        let (outbound_tx, mut outbound_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let (inbound_tx, inbound_rx) = mpsc::unbounded_channel::<Vec<u8>>();

        let hello = RelayWireMessage {
            from: party_id,
            run_id: run_id.clone(),
            payload: Some(format!("register party {}", party_id)),
            payload_b64: None,
        };

        writer
            .write_all(format!("{}\n", serde_json::to_string(&hello)?).as_bytes())
            .await?;

        let write_run_id = run_id.clone();

        tokio::spawn(async move {
            while let Some(msg) = outbound_rx.recv().await {
                let wire = RelayWireMessage {
                    from: party_id,
                    run_id: write_run_id.clone(),
                    payload: None,
                    payload_b64: Some(general_purpose::STANDARD.encode(msg)),
                };

                let line = match serde_json::to_string(&wire) {
                    Ok(line) => line,
                    Err(err) => {
                        eprintln!("failed to serialize outbound relay message: {err:?}");
                        continue;
                    }
                };

                if writer.write_all(line.as_bytes()).await.is_err() {
                    break;
                }

                if writer.write_all(b"\n").await.is_err() {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            let mut lines = BufReader::new(reader).lines();

            loop {
                match lines.next_line().await {
                    Ok(Some(line)) => {
                        let wire: RelayWireMessage = match serde_json::from_str(&line) {
                            Ok(wire) => wire,
                            Err(err) => {
                                eprintln!("failed to deserialize inbound relay message: {err:?}");
                                continue;
                            }
                        };

                        if let Some(payload_b64) = wire.payload_b64 {
                            match general_purpose::STANDARD.decode(payload_b64) {
                                Ok(bytes) => {
                                    if inbound_tx.send(bytes).is_err() {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    eprintln!("failed to decode inbound base64 payload: {err:?}");
                                }
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(err) => {
                        eprintln!("tcp relay read error: {err:?}");
                        break;
                    }
                }
            }
        });

        Ok(Self {
            outbound_tx,
            inbound_rx,
        })
    }
}

impl Stream for TcpRelayConnection {
    type Item = Vec<u8>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        Pin::new(&mut this.inbound_rx).poll_recv(cx)
    }
}

impl Sink<Vec<u8>> for TcpRelayConnection {
    type Error = MessageSendError;

    fn poll_ready(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(
        self: Pin<&mut Self>,
        item: Vec<u8>,
    ) -> Result<(), Self::Error> {
        self.get_mut()
            .outbound_tx
            .send(item)
            .map_err(|_| MessageSendError)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Relay for TcpRelayConnection {}
