# ML Worker Interface

## Philosophy

ML inference is externalized.

Rust core should not directly embed:
- PyTorch
- CUDA logic
- diffusion pipelines

## Communication Options

- filesystem artifacts
- HTTP
- gRPC
- job queue

## Worker Categories

- segmentation worker
- pose worker
- depth worker
- diffusion render worker
- repair worker

## Example Flow

Rust:
- creates render manifest
- schedules job
- waits for artifact

Worker:
- executes inference
- writes outputs
- updates manifest

## Rationale

This keeps:
- Rust deterministic
- ML replaceable
- pipeline modular
- tooling stable
