#!/usr/bin/env bash
set -euo pipefail

N="${1:?usage: run_dkls_dsg_round.sh N T SIGNER_IDS DKG_RUN_DIR SIGN_DIR ROUND_RUN_ID}"
T="${2:?usage: run_dkls_dsg_round.sh N T SIGNER_IDS DKG_RUN_DIR SIGN_DIR ROUND_RUN_ID}"
SIGNER_IDS="${3:?usage: run_dkls_dsg_round.sh N T SIGNER_IDS DKG_RUN_DIR SIGN_DIR ROUND_RUN_ID}"
DKG_RUN_DIR="${4:?usage: run_dkls_dsg_round.sh N T SIGNER_IDS DKG_RUN_DIR SIGN_DIR ROUND_RUN_ID}"
SIGN_DIR="${5:?usage: run_dkls_dsg_round.sh N T SIGNER_IDS DKG_RUN_DIR SIGN_DIR ROUND_RUN_ID}"
ROUND_RUN_ID="${6:?usage: run_dkls_dsg_round.sh N T SIGNER_IDS DKG_RUN_DIR SIGN_DIR ROUND_RUN_ID}"

mkdir -p "$SIGN_DIR"

IFS=',' read -ra IDS <<< "$SIGNER_IDS"
SIGNER_COUNT="${#IDS[@]}"

RELAY_PORT=$((9100 + RANDOM % 2000))
RELAY_ADDR="127.0.0.1:${RELAY_PORT}"

echo "[dsg-round] starting relay for $ROUND_RUN_ID"

pkill -f "tcp_relay_server.*${RELAY_PORT}" || true
sleep 1

cd "$HOME/dkls23"
source "$HOME/.cargo/env"

/usr/bin/time -v -o "$SIGN_DIR/relay_time.txt" \
  "$HOME/dkls23/target/release/tcp_relay_server" \
  --bind "$RELAY_ADDR" \
  --expected-parties "$SIGNER_COUNT" \
  > "$SIGN_DIR/relay.log" \
  2> "$SIGN_DIR/relay.err" &

RELAY_PID=$!
echo "$RELAY_PID" > "$SIGN_DIR/relay.pid"

echo "[dsg-round] waiting for relay readiness..."

for attempt in $(seq 1 20); do
  if nc -z 127.0.0.1 "$RELAY_PORT"; then
    echo "[dsg-round] relay ready on port $RELAY_PORT"
    break
  fi

  sleep 1

  if [[ "$attempt" == "20" ]]; then
    echo "[dsg-round] relay failed to become ready" >&2
    cat "$SIGN_DIR/relay.err" >&2 || true
    exit 1
  fi
done

PIDS=()

for i in "${IDS[@]}"; do
  echo "[dsg-round] launching signer party $i"

  timeout 180s /usr/bin/time -v \
    -o "$SIGN_DIR/party-${i}.time.txt" \
    "$HOME/dkls23/target/release/dkls_sign_party" \
    --id "$i" \
    --n "$N" \
    --t "$T" \
    --relay "$RELAY_ADDR" \
    --run-id "$ROUND_RUN_ID" \
    --share-dir "$DKG_RUN_DIR" \
    --signer-ids "$SIGNER_IDS" \
    > "$SIGN_DIR/party-${i}.stdout.log" \
    2> "$SIGN_DIR/party-${i}.stderr.log" &

  PIDS+=("$!")
done

STATUS=success

for pid in "${PIDS[@]}"; do
  if ! wait "$pid"; then
    STATUS=failed
  fi
done

kill "$RELAY_PID" 2>/dev/null || true
wait "$RELAY_PID" 2>/dev/null || true
pkill -f "tcp_relay_server" || true
sleep 2

echo "$STATUS" > "$SIGN_DIR/status.txt"

if [[ "$STATUS" != "success" ]]; then
  echo "[dsg-round] round failed: $ROUND_RUN_ID" >&2
  exit 1
fi

echo "[dsg-round] round complete: $ROUND_RUN_ID"
