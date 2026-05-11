#!/usr/bin/env bash
set -euo pipefail

N="${1:?usage: run_dkls_dkg_dsg.sh <n> <t> [trial]}"
T="${2:?usage: run_dkls_dkg_dsg.sh <n> <t> [trial]}"
TRIAL="${3:-1}"

SIGNER_COUNT=$((T + 1))
SIGNER_IDS="$(seq -s, 1 "$SIGNER_COUNT")"

MASTER_RUN_ID="$(date -u +%Y-%m-%d_%H%M%S)_dkls_n${N}_t${T}_dkg_dsg_trial${TRIAL}"
MASTER_RUN_DIR="$HOME/socioty-results/dkls/$MASTER_RUN_ID"
mkdir -p "$MASTER_RUN_DIR"

echo "MASTER_RUN_ID=$MASTER_RUN_ID"
echo "MASTER_RUN_DIR=$MASTER_RUN_DIR"
echo "SIGNER_COUNT=$SIGNER_COUNT"
echo "SIGNER_IDS=$SIGNER_IDS"

echo "Running DKG..."
"$HOME/socioty-controller/run_dkls_dkg.sh" "$N" "$T" "$TRIAL"

DKG_RUN_DIR="$(ls -td "$HOME"/socioty-results/dkls/*_dkls_n${N}_t${T}_dkg_trial${TRIAL} | head -1)"

if [[ -z "$DKG_RUN_DIR" ]]; then
  echo "Could not find DKG run dir"
  exit 1
fi

echo "DKG_RUN_DIR=$DKG_RUN_DIR"

echo "Copying DKG results..."
cp -r "$DKG_RUN_DIR" "$MASTER_RUN_DIR/dkg"

DSG_RUN_DIR="$MASTER_RUN_DIR/dsg"
mkdir -p "$DSG_RUN_DIR"

echo "Running DSG..."

pkill -f tcp_relay_server || true
pkill -f dkls_sign_party || true

nohup "$HOME/dkls23/target/release/tcp_relay_server" \
  --bind 127.0.0.1:9100 \
  --expected-parties "$SIGNER_COUNT" \
  > "$DSG_RUN_DIR/relay.log" 2>&1 &

RELAY_PID="$!"

sleep 1

PIDS=()

for ((i=1; i<=SIGNER_COUNT; i++)); do
  echo "Starting DSG signer party $i..."

  timeout 60s /usr/bin/time -v \
    -o "$DSG_RUN_DIR/party-${i}.time.txt" \
    "$HOME/dkls23/target/release/dkls_sign_party" \
    --id "$i" --n "$N" --t "$T" \
    --relay 127.0.0.1:9100 \
    --run-id "$MASTER_RUN_ID/dsg" \
    --share-dir "$DKG_RUN_DIR" \
    --signer-ids "$SIGNER_IDS" \
    > "$DSG_RUN_DIR/party-${i}.stdout.log" \
    2> "$DSG_RUN_DIR/party-${i}.stderr.log" &

  PIDS+=("$!")
done

STATUS="success"

for pid in "${PIDS[@]}"; do
  if ! wait "$pid"; then
    STATUS="failed"
  fi
done

kill "$RELAY_PID" 2>/dev/null || true

cat > "$MASTER_RUN_DIR/metadata.json" <<META
{
  "run_id": "$MASTER_RUN_ID",
  "protocol": "dkls",
  "operation": "dkg_dsg",
  "n": $N,
  "t": $T,
  "trial": $TRIAL,
  "status": "$STATUS",
  "dkg_run_dir": "$DKG_RUN_DIR",
  "dsg_run_dir": "$DSG_RUN_DIR",
  "signer_count": $SIGNER_COUNT,
  "signer_ids": [$(seq -s, 1 "$SIGNER_COUNT")]
}
META

echo "Writing combined metrics..."

python3 - "$MASTER_RUN_DIR" "$MASTER_RUN_ID" "$N" "$T" "$TRIAL" "$STATUS" "$SIGNER_IDS" "$SIGNER_COUNT" <<'PY'
import json
import sys
from pathlib import Path

master_dir = Path(sys.argv[1])
run_id = sys.argv[2]
n = int(sys.argv[3])
t = int(sys.argv[4])
trial = int(sys.argv[5])
status = sys.argv[6]
signer_ids_str = sys.argv[7]
signer_count = int(sys.argv[8])

signer_ids = [int(x) for x in signer_ids_str.split(",") if x]

dkg_metrics_path = master_dir / "dkg" / "metrics.json"
dsg_dir = master_dir / "dsg"

with dkg_metrics_path.open() as f:
    dkg_metrics = json.load(f)

def parse_kv(path: Path) -> dict:
    data = {}
    if not path.exists():
        return data
    for line in path.read_text().splitlines():
        if "=" in line:
            k, v = line.split("=", 1)
            data[k.strip()] = v.strip()
    return data

def parse_time_file(path: Path) -> dict:
    import re

    if not path.exists():
        return {
            "wall_time": None,
            "max_rss_kb": None,
            "user_time_sec": None,
            "system_time_sec": None,
            "cpu_percent": None,
        }

    text = path.read_text()

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

dsg_parties = []

for party_id in signer_ids:
    metrics = parse_kv(dsg_dir / f"party-{party_id}.sign.metrics.txt")
    timing = parse_time_file(dsg_dir / f"party-{party_id}.time.txt")

    elapsed_ms = metrics.get("elapsed_ms")
    try:
        elapsed_ms = int(elapsed_ms) if elapsed_ms is not None else None
    except ValueError:
        elapsed_ms = None

    dsg_parties.append({
        "party_id": party_id,
        "key_id": metrics.get("key_id"),
        "elapsed_ms": elapsed_ms,
        "status": metrics.get("status"),
        "signature_path": str(dsg_dir / f"party-{party_id}.signature.txt"),
        **timing,
    })

elapsed_values = [p["elapsed_ms"] for p in dsg_parties if p["elapsed_ms"] is not None]
rss_values = [p["max_rss_kb"] for p in dsg_parties if p.get("max_rss_kb") is not None]

dsg_metrics = {
    "status": status,
    "signer_count": signer_count,
    "signer_ids": signer_ids,
    "parties": dsg_parties,
    "elapsed_ms_avg": round(sum(elapsed_values) / len(elapsed_values), 2) if elapsed_values else None,
    "elapsed_ms_max": max(elapsed_values) if elapsed_values else None,
    "max_rss_kb": max(rss_values) if rss_values else None,
    "avg_rss_kb": round(sum(rss_values) / len(rss_values), 2) if rss_values else None,
}

combined = {
    "run_id": run_id,
    "protocol": "dkls",
    "operation": "dkg_dsg",
    "n": n,
    "t": t,
    "trial": trial,
    "status": status,
    "dkg": dkg_metrics,
    "dsg": dsg_metrics,
}

out = master_dir / "metrics.json"
out.write_text(json.dumps(combined, indent=2) + "\n")
PY

echo "DKG+DSG complete: $MASTER_RUN_DIR"
echo "STATUS=$STATUS"
echo "Combined metrics: $MASTER_RUN_DIR/metrics.json"

find "$MASTER_RUN_DIR" -maxdepth 2 -type f | sort

if [[ "$STATUS" != "success" ]]; then
  exit 1
fi
