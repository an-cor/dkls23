#!/usr/bin/env python3

import csv
import json
from pathlib import Path


def avg(values):
    values = [v for v in values if v is not None]
    return round(sum(values) / len(values), 2) if values else None


def max_or_none(values):
    values = [v for v in values if v is not None]
    return max(values) if values else None


def main():
    root = Path.home() / "socioty-results" / "dkls"
    out = root / "dkls_dkg_dsg_phase6d_summary.csv"

    rows = []

    for metrics_path in sorted(root.glob("*_dkls_n*_t*_dkg_dsg_trial*/metrics.json")):
        with metrics_path.open() as f:
            d = json.load(f)

        dkg = d.get("dkg", {})
        dsg = d.get("dsg", {})

        dkg_parties = dkg.get("parties", [])
        dsg_parties = dsg.get("parties", [])

        dkg_elapsed = [p.get("elapsed_ms") for p in dkg_parties]
        dkg_rss = [p.get("max_rss_kb") for p in dkg_parties]
        dsg_elapsed = [p.get("elapsed_ms") for p in dsg_parties]
        dsg_rss = [p.get("max_rss_kb") for p in dsg_parties]

        rows.append({
            "run_id": d.get("run_id"),
            "status": d.get("status"),
            "n": d.get("n"),
            "t": d.get("t"),
            "trial": d.get("trial"),
            "signer_count": dsg.get("signer_count"),
            "signer_ids": ",".join(map(str, dsg.get("signer_ids", []))),
            "dkg_status": dkg.get("status"),
            "dsg_status": dsg.get("status"),
            "dkg_controller_wall_ms": dkg.get("controller", {}).get("controller_wall_ms"),
            "dkg_party_avg_elapsed_ms": avg(dkg_elapsed),
            "dkg_party_max_elapsed_ms": max_or_none(dkg_elapsed),
            "dkg_party_max_rss_kb": max_or_none(dkg_rss),
            "dsg_party_avg_elapsed_ms": dsg.get("elapsed_ms_avg") or avg(dsg_elapsed),
            "dsg_party_max_elapsed_ms": dsg.get("elapsed_ms_max") or max_or_none(dsg_elapsed),
            "dsg_party_avg_rss_kb": dsg.get("avg_rss_kb") or avg(dsg_rss),
            "dsg_party_max_rss_kb": dsg.get("max_rss_kb") or max_or_none(dsg_rss),
            "metrics_path": str(metrics_path),
        })

    fieldnames = [
        "run_id",
        "status",
        "n",
        "t",
        "trial",
        "signer_count",
        "signer_ids",
        "dkg_status",
        "dsg_status",
        "dkg_controller_wall_ms",
        "dkg_party_avg_elapsed_ms",
        "dkg_party_max_elapsed_ms",
        "dkg_party_max_rss_kb",
        "dsg_party_avg_elapsed_ms",
        "dsg_party_max_elapsed_ms",
        "dsg_party_avg_rss_kb",
        "dsg_party_max_rss_kb",
        "metrics_path",
    ]

    with out.open("w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)

    print(out)


if __name__ == "__main__":
    main()
