#!/usr/bin/env bash
set -euo pipefail

N="${1:?usage: run_dkls_multisign.sh N T SIGNATURES MODE TRIAL}"
T="${2:?usage: run_dkls_multisign.sh N T SIGNATURES MODE TRIAL}"
SIGNATURES="${3:?usage: run_dkls_multisign.sh N T SIGNATURES MODE TRIAL}"
MODE="${4:?usage: run_dkls_multisign.sh N T SIGNATURES MODE TRIAL}"
TRIAL="${5:?usage: run_dkls_multisign.sh N T SIGNATURES MODE TRIAL}"

if [[ "$MODE" != "fixed" && "$MODE" != "random" ]]; then
  echo "MODE must be fixed or random" >&2
  exit 1
fi

SIGNER_COUNT=$((T + 1))
SIGNER_IDS="$(seq -s, 1 "$SIGNER_COUNT")"

TS="$(date +%Y-%m-%d_%H%M%S)"
MASTER_RUN_ID="${TS}_dkls_n${N}_t${T}_multisign_${SIGNATURES}_${MODE}_trial${TRIAL}"
BASE_DIR="$HOME/socioty-results/dkls/$MASTER_RUN_ID"

DKG_RUN_DIR="$BASE_DIR/dkg"

mkdir -p "$BASE_DIR" "$DKG_RUN_DIR"

START_MS="$(date +%s%3N)"

echo "[multisign] run_id=$MASTER_RUN_ID"
echo "[multisign] n=$N t=$T signatures=$SIGNATURES mode=$MODE trial=$TRIAL"
if [[ "$MODE" == "fixed" ]]; then
  echo "[multisign] fixed signer_ids=$SIGNER_IDS"
else
  echo "[multisign] random signer mode enabled"
fi

cat > "$BASE_DIR/metadata.json" <<JSON
{
  "run_id": "$MASTER_RUN_ID",
  "n": $N,
  "t": $T,
  "signer_count": $SIGNER_COUNT,
  "signatures_per_keygen": $SIGNATURES,
  "mode": "$MODE",
  "trial": $TRIAL,
  "signer_ids": "$SIGNER_IDS"
}
JSON

echo "[multisign] running DKG once..."

"$HOME/socioty-controller/run_dkls_dkg.sh" "$N" "$T" "$TRIAL"

LATEST_DKG="$(ls -td "$HOME"/socioty-results/dkls/*_dkls_n${N}_t${T}_dkg_trial${TRIAL} | head -1)"

if [[ ! -d "$LATEST_DKG" ]]; then
  echo "[multisign] ERROR: could not find DKG output" >&2
  exit 1
fi

echo "[multisign] copying DKG output from $LATEST_DKG"
cp -a "$LATEST_DKG"/. "$DKG_RUN_DIR"/

choose_signers() {
  if [[ "$MODE" == "fixed" ]]; then
    seq -s, 1 "$SIGNER_COUNT"
  else
    seq 1 "$N" | shuf | head -n "$SIGNER_COUNT" | sort -n | paste -sd, -
  fi
}

SIGN_ROUND_DIRS=()

for ROUND in $(seq 1 "$SIGNATURES"); do
  ROUND_PADDED="$(printf "%02d" "$ROUND")"
  SIGN_DIR="$BASE_DIR/sign_round_${ROUND_PADDED}"
  mkdir -p "$SIGN_DIR"
  SIGN_ROUND_DIRS+=("$SIGN_DIR")

  ROUND_SIGNER_IDS="$(choose_signers)"

  echo "[multisign] signing round $ROUND_PADDED using signers $ROUND_SIGNER_IDS"

  ROUND_STATUS="success"

  if ! "$HOME/socioty-controller/run_dkls_dsg_round.sh" \
    "$N" "$T" "$ROUND_SIGNER_IDS" "$DKG_RUN_DIR" "$SIGN_DIR" "$MASTER_RUN_ID/sign_round_${ROUND_PADDED}"
  then
    ROUND_STATUS="failed"
  fi

  echo "$ROUND_STATUS" > "$SIGN_DIR/round_status.txt"
done

END_MS="$(date +%s%3N)"
CONTROLLER_WALL_MS=$((END_MS - START_MS))

echo "[multisign] aggregating metrics..."

python3 - "$BASE_DIR" "$N" "$T" "$SIGNER_COUNT" "$SIGNATURES" "$MODE" "$TRIAL" "$SIGNER_IDS" "$CONTROLLER_WALL_MS" <<'PY'
import json
import re
import sys
from pathlib import Path

base = Path(sys.argv[1])
n = int(sys.argv[2])
t = int(sys.argv[3])
signer_count = int(sys.argv[4])
signatures = int(sys.argv[5])
mode = sys.argv[6]
trial = int(sys.argv[7])
signer_ids = sys.argv[8]
controller_wall_ms = int(sys.argv[9])

def load_json(path):
    if path.exists():
        return json.loads(path.read_text())
    return {}

def parse_time_file(path):
    if not path.exists():
        return {}
    txt = path.read_text(errors="ignore")

    def grab(pattern, cast=str):
        m = re.search(pattern, txt)
        if not m:
            return None
        try:
            return cast(m.group(1))
        except Exception:
            return m.group(1)

    return {
        "user_time_sec": grab(r"User time \(seconds\):\s*([0-9.]+)", float),
        "system_time_sec": grab(r"System time \(seconds\):\s*([0-9.]+)", float),
        "cpu_percent": grab(r"Percent of CPU this job got:\s*([0-9]+)%", int),
        "wall_time": grab(r"Elapsed \(wall clock\) time.*:\s*(.+)", str),
        "max_rss_kb": grab(r"Maximum resident set size \(kbytes\):\s*([0-9]+)", int),
    }

dkg_metrics = load_json(base / "dkg" / "metrics.json")

rounds = []
all_sign_times = []
all_rss = []
signer_ids_per_round = []

for sign_dir in sorted(base.glob("sign_round_*")):
    round_name = sign_dir.name
    party_entries = []
    round_elapsed = []

    ids_this_round = []

    for metrics_path in sorted(sign_dir.glob("party-*.sign.metrics.txt")):
        party_id = int(metrics_path.name.split("-")[1].split(".")[0])
        ids_this_round.append(party_id)

        text = metrics_path.read_text(errors="ignore")

        elapsed_ms = None
        status = None
        key_id = None
        signature_path = None

        for line in text.splitlines():
            if line.startswith("elapsed_ms="):
                elapsed_ms = int(line.split("=", 1)[1])
            elif line.startswith("status="):
                status = line.split("=", 1)[1]
            elif line.startswith("key_id="):
                key_id = line.split("=", 1)[1]
            elif line.startswith("signature_path="):
                signature_path = line.split("=", 1)[1]

        time_metrics = parse_time_file(sign_dir / f"party-{party_id}.time.txt")

        entry = {
            "party_id": party_id,
            "key_id": key_id,
            "elapsed_ms": elapsed_ms,
            "status": status,
            "signature_path": signature_path,
            "relay_time_file": str(sign_dir / "relay_time.txt"),
            **time_metrics,
        }

        if elapsed_ms is not None:
            round_elapsed.append(elapsed_ms)
            all_sign_times.append(elapsed_ms)

        if time_metrics.get("max_rss_kb") is not None:
            all_rss.append(time_metrics["max_rss_kb"])

        party_entries.append(entry)

    ids_this_round = sorted(ids_this_round)
    signer_ids_per_round.append(ids_this_round)

    rounds.append({
        "round": round_name,
        "status": (
            (sign_dir / "round_status.txt").read_text().strip()
            if (sign_dir / "round_status.txt").exists()
            else "unknown"
        ),
        "signer_ids": ids_this_round,
        "elapsed_ms_avg": sum(round_elapsed) / len(round_elapsed) if round_elapsed else None,
        "elapsed_ms_max": max(round_elapsed) if round_elapsed else None,
        "parties": party_entries,
    })

dkg_party_times = []
dkg_rss = []

for p in dkg_metrics.get("parties", []):
    if p.get("elapsed_ms") is not None:
        dkg_party_times.append(p["elapsed_ms"])
    if p.get("max_rss_kb") is not None:
        dkg_rss.append(p["max_rss_kb"])

successful_rounds = sum(
    1 for r in rounds
    if r.get("status") == "success"
)

failed_rounds = signatures - successful_rounds

summary_status = (
    "success"
    if successful_rounds == signatures
    else "partial"
)

successful_sign_times = [
    r["elapsed_ms_max"]
    for r in rounds
    if r.get("status") == "success"
    and r.get("elapsed_ms_max") is not None
]

summary = {
    "run_id": base.name,
    "status": summary_status,
    "n": n,
    "t": t,
    "signer_count": signer_count,
    "mode": mode,
    "trial": trial,
    "signatures_per_keygen": signatures,
    "dkg": {
        "source_metrics": str(base / "dkg" / "metrics.json"),
        "party_elapsed_ms_avg": sum(dkg_party_times) / len(dkg_party_times) if dkg_party_times else None,
        "party_elapsed_ms_max": max(dkg_party_times) if dkg_party_times else None,
        "max_rss_kb": max(dkg_rss) if dkg_rss else None,
    },
    "signing": {
        "rounds": rounds,
        "sign_round_times": successful_sign_times,
        "total_sign_time": sum(successful_sign_times),
        "avg_sign_time": (
            sum(successful_sign_times) / len(successful_sign_times)
            if successful_sign_times else None
        ),
        "max_rss_per_party": max(all_rss) if all_rss else None,
        "signer_ids_per_round": signer_ids_per_round,
    },
    "controller_wall_ms": controller_wall_ms,
    "successful_rounds": successful_rounds,
    "signer_ids": signer_ids,
    "failed_rounds": failed_rounds,
}

(base / "metrics.json").write_text(json.dumps(summary, indent=2))
print(json.dumps(summary, indent=2))
PY

echo "[multisign] complete: $BASE_DIR"
echo "[multisign] metrics: $BASE_DIR/metrics.json"
