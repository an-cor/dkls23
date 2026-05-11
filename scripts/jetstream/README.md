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

