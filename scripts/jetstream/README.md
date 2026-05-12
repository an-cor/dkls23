# Jetstream DKLS DKG Orchestration

This folder contains the controller-side scripts for running DKLS DKG across Jetstream VMs.

## Current status

DKG-only orchestration is working across 3, 5, and 10 party VMs.

Verified combinations:

- n=3, t=2
- n=5, t=2
- n=5, t=3
- n=10, t=5
- n=10, t=6
- n=10, t=7
- n=10, t=8

## Run command

From the controller VM:

```bash
~/socioty-controller/run_dkls_dkg.sh 3 2 1


## DSG implementation note: Option A vs Option B

The current DSG runner uses the Option A setup path. Each signer process loads all shares in the selected signer subset, calls `sign::setup_dsg(...)`, and then selects its own local setup. This made it possible to validate distributed DSG orchestration quickly.

This is acceptable for benchmarking and orchestration proof-of-concept work, but it is not the cleanest security model because each signer process temporarily has access to all signer shares in the subset.

Future work should implement Option B: each signer should load only its own share plus public signer metadata, then construct only its own DSG setup. That would better match production threshold-signing expectations.

# DKLS Jetstream Benchmark Orchestration

This directory contains orchestration scripts for distributed DKLS23 benchmarking across Jetstream VMs.

## Scripts

### run_dkls_dkg.sh

Runs distributed DKLS DKG across party VMs.

Example:

```bash
./run_dkls_dkg.sh 3 2 1
