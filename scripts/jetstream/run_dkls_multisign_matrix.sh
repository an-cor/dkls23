#!/usr/bin/env bash
set -euo pipefail

TRIAL="${1:-1}"

TS="$(date +%Y-%m-%d_%H%M%S)"
MATRIX_ID="${TS}_dkls_phase7c_multisign_matrix_trial${TRIAL}"
MATRIX_DIR="$HOME/socioty-results/dkls/$MATRIX_ID"

mkdir -p "$MATRIX_DIR"

MANIFEST="$MATRIX_DIR/runs.txt"
SUMMARY="$MATRIX_DIR/summary.csv"

echo "run_id,n,t,mode,signatures,status,successful_rounds,failed_rounds,dkg_time_ms,total_sign_time_ms,avg_sign_time_ms,max_rss_per_party_kb,controller_wall_ms,run_dir" > "$SUMMARY"
: > "$MANIFEST"

run_one() {
  local n="$1"
  local t="$2"
  local sigs="$3"
  local mode="$4"

  echo "[matrix] running n=$n t=$t sigs=$sigs mode=$mode trial=$TRIAL"

  local before
  before="$(date +%s)"

  if ! "$HOME/socioty-controller/run_dkls_multisign.sh" "$n" "$t" "$sigs" "$mode" "$TRIAL"; then
    echo "[matrix] WARNING: run command returned nonzero for n=$n t=$t sigs=$sigs mode=$mode"
  fi

  local run_dir
  run_dir="$(ls -td "$HOME"/socioty-results/dkls/*_dkls_n${n}_t${t}_multisign_${sigs}_${mode}_trial${TRIAL} | head -1)"

  echo "$run_dir" >> "$MANIFEST"

  if [[ ! -f "$run_dir/metrics.json" ]]; then
    echo "[matrix] WARNING: missing metrics.json for $run_dir"
    echo "$(basename "$run_dir"),$n,$t,$mode,$sigs,missing_metrics,0,$sigs,,,,,$run_dir" >> "$SUMMARY"
    return 0
  fi

  python3 - "$run_dir" "$SUMMARY" <<'PY'
import csv
import json
import sys
from pathlib import Path

run_dir = Path(sys.argv[1])
summary_path = Path(sys.argv[2])

m = json.loads((run_dir / "metrics.json").read_text())

row = {
    "run_id": m.get("run_id"),
    "n": m.get("n"),
    "t": m.get("t"),
    "mode": m.get("mode"),
    "signatures": m.get("signatures_per_keygen"),
    "status": m.get("status"),
    "successful_rounds": m.get("successful_rounds"),
    "failed_rounds": m.get("failed_rounds"),
    "dkg_time_ms": m.get("dkg", {}).get("party_elapsed_ms_max"),
    "total_sign_time_ms": m.get("signing", {}).get("total_sign_time"),
    "avg_sign_time_ms": m.get("signing", {}).get("avg_sign_time"),
    "max_rss_per_party_kb": m.get("signing", {}).get("max_rss_per_party"),
    "controller_wall_ms": m.get("controller_wall_ms"),
    "run_dir": str(run_dir),
}

with summary_path.open("a", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=list(row.keys()))
    writer.writerow(row)
PY

  sleep 3
}

for n_t in "3 2" "5 3"; do
  read -r n t <<< "$n_t"

  for sigs in 1 5 10; do
    for mode in fixed random; do
      pkill -f tcp_relay_server || true
      run_one "$n" "$t" "$sigs" "$mode"
    done
  done
done

echo "[matrix] complete"
echo "[matrix] dir: $MATRIX_DIR"
echo "[matrix] summary: $SUMMARY"
