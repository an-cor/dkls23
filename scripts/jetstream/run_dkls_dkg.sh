#!/usr/bin/env bash
set -euo pipefail

N="${1:?usage: run_dkls_dkg.sh <n> <t> [trial]}"
T="${2:?usage: run_dkls_dkg.sh <n> <t> [trial]}"
TRIAL="${3:-1}"

KEY="$HOME/.ssh/socioty_controller_key"
CONFIG="$HOME/socioty-controller/parties.json"
REPO="$HOME/dkls23"
RELAY_IP="10.1.20.67"
RELAY_PORT="9100"

TS="$(date +%Y-%m-%d_%H%M%S)"
RUN_ID="${TS}_dkls_n${N}_t${T}_dkg_trial${TRIAL}"
RUN_DIR="$HOME/socioty-results/dkls/$RUN_ID"

mkdir -p "$RUN_DIR"

echo "RUN_ID=$RUN_ID"
echo "RUN_DIR=$RUN_DIR"

CONTROLLER_START_MS="$(date +%s%3N)"

cat > "$RUN_DIR/metadata.json" <<META
{
  "run_id": "$RUN_ID",
  "protocol": "dkls",
  "operation": "dkg",
  "n": $N,
  "t": $T,
  "trial": $TRIAL,
  "relay": "$RELAY_IP:$RELAY_PORT",
  "started_at": "$(date -Iseconds)"
}
META

echo "Stopping old relay if present..."
pkill -f "tcp_relay_server" || true
sleep 1

echo "Starting controller relay..."
cd "$REPO"
source "$HOME/.cargo/env"

/usr/bin/time -v -o "$RUN_DIR/controller_time.txt" \
  "$REPO/target/release/tcp_relay_server" \
  --bind "$RELAY_IP:$RELAY_PORT" \
  > "$RUN_DIR/controller.log" \
  2> "$RUN_DIR/controller.err" &

RELAY_PID=$!
echo "$RELAY_PID" > "$RUN_DIR/relay.pid"

sleep 2

echo "Launching $N parties..."

PARTY_PIDS=()
mapfile -t PARTIES < <(jq -c ".parties[:$N][]" "$CONFIG")

for party in "${PARTIES[@]}"; do
  id="$(echo "$party" | jq -r '.id')"
  public_ip="$(echo "$party" | jq -r '.public_ip')"
  user="$(echo "$party" | jq -r '.user')"

  party_name="$(printf 'party-%02d' "$id")"
  party_dir="$RUN_DIR/$party_name"
  mkdir -p "$party_dir"

  echo "Starting party $id at $public_ip..."

  ssh -i "$KEY" "$user@$public_ip" "
    set -euo pipefail
    cd ~/dkls23
    source \"\$HOME/.cargo/env\"

    REMOTE_RUN_DIR=\"\$HOME/socioty-results/dkls/$RUN_ID\"
    mkdir -p \"\$REMOTE_RUN_DIR\"

    /usr/bin/time -v -o \"\$REMOTE_RUN_DIR/party-${id}.time.txt\" \
      ./target/release/dkls_party \
      --id $id \
      --n $N \
      --t $T \
      --relay $RELAY_IP:$RELAY_PORT \
      --run-id $RUN_ID \
      > \"\$REMOTE_RUN_DIR/party-${id}.stdout.log\" \
      2> \"\$REMOTE_RUN_DIR/party-${id}.stderr.log\"
  " > "$party_dir/ssh.stdout.log" 2> "$party_dir/ssh.stderr.log" &
  PARTY_PIDS+=($!)

done

echo "Waiting for party SSH jobs..."

FAIL=0

for pid in "${PARTY_PIDS[@]}"; do
  if ! wait "$pid"; then
    FAIL=1
  fi
done

echo "Stopping controller relay..."
kill "$RELAY_PID" 2>/dev/null || true
sleep 1

echo "Collecting party results..."

for party in "${PARTIES[@]}"; do
  id="$(echo "$party" | jq -r '.id')"
  public_ip="$(echo "$party" | jq -r '.public_ip')"
  user="$(echo "$party" | jq -r '.user')"

  party_name="$(printf 'party-%02d' "$id")"
  party_dir="$RUN_DIR/$party_name"

  scp -i "$KEY" -r \
    "$user@$public_ip:~/socioty-results/dkls/$RUN_ID/"* \
    "$party_dir/" \
    > "$party_dir/scp.stdout.log" \
    2> "$party_dir/scp.stderr.log" || true
done

CONTROLLER_END_MS="$(date +%s%3N)"
CONTROLLER_WALL_MS="$((CONTROLLER_END_MS - CONTROLLER_START_MS))"

echo "Writing summary..."

python3 - "$RUN_DIR" "$RUN_ID" "$N" "$T" "$TRIAL" "$FAIL" "$CONTROLLER_WALL_MS" <<'PY'
import json
import re
import sys
from pathlib import Path

run_dir = Path(sys.argv[1])
run_id = sys.argv[2]
n = int(sys.argv[3])
t = int(sys.argv[4])
trial = int(sys.argv[5])
fail = int(sys.argv[6])
controller_wall_ms = int(sys.argv[7])

def read_text(path: Path) -> str:
    try:
        return path.read_text()
    except FileNotFoundError:
        return ""

def parse_kv_file(path: Path) -> dict:
    data = {}
    for line in read_text(path).splitlines():
        if "=" in line:
            k, v = line.split("=", 1)
            data[k.strip()] = v.strip()
    return data

def parse_time_file(path: Path) -> dict:
    text = read_text(path)

    def find(pattern):
        m = re.search(pattern, text)
        return m.group(1).strip() if m else None

    def to_float(x):
        try:
            return float(x) if x not in (None, "") else None
        except ValueError:
            return None

    def to_int(x):
        try:
            return int(x) if x not in (None, "") else None
        except ValueError:
            return None

    return {
        "wall_time": find(r"Elapsed \(wall clock\) time.*\):\s*(.+)"),
        "max_rss_kb": to_int(find(r"Maximum resident set size \(kbytes\):\s*(\d+)")),
        "user_time_sec": to_float(find(r"User time \(seconds\):\s*([0-9.]+)")),
        "system_time_sec": to_float(find(r"System time \(seconds\):\s*([0-9.]+)")),
        "cpu_percent": find(r"Percent of CPU this job got:\s*(.+)"),
    }

parties = []

for party_id in range(1, n + 1):
    party_dir = run_dir / f"party-{party_id:02d}"
    metrics = parse_kv_file(party_dir / f"party-{party_id}.metrics.txt")
    timing = parse_time_file(party_dir / f"party-{party_id}.time.txt")

    elapsed_ms = metrics.get("elapsed_ms")
    try:
        elapsed_ms = int(elapsed_ms) if elapsed_ms is not None else None
    except ValueError:
        elapsed_ms = None

    parties.append({
        "party_id": party_id,
        "key_id": metrics.get("key_id"),
        "elapsed_ms": elapsed_ms,
        **timing,
    })

controller = parse_time_file(run_dir / "controller_time.txt")
controller["controller_wall_ms"] = controller_wall_ms

summary = {
    "run_id": run_id,
    "protocol": "dkls",
    "operation": "dkg",
    "n": n,
    "t": t,
    "trial": trial,
    "status": "success" if fail == 0 else "failed",
    "run_dir": str(run_dir),
    "controller": controller,
    "parties": parties,
}

tmp = run_dir / "metrics.tmp.json"
out = run_dir / "metrics.json"

tmp.write_text(json.dumps(summary, indent=2) + "\n")
tmp.replace(out)
PY

echo "Summary:"
jq '{run_id, status, n, t, trial, controller, parties}' "$RUN_DIR/metrics.json"

echo "Run complete."
echo "Status: $([ "$FAIL" -eq 0 ] && echo success || echo failed)"
echo "Results: $RUN_DIR"

exit "$FAIL"
