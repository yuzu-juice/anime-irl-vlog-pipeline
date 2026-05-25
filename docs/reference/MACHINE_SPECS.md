# Machine Specifications

## Philosophy

This project should target:
- creator workstations
- consumer GPUs
- local iteration

NOT:
- datacenter inference
- massive cluster rendering

## Recommended Development Machine

### Minimum Usable

CPU:
- 8-core modern CPU

RAM:
- 32 GB

GPU:
- RTX 4070 Ti / equivalent

VRAM:
- 12 GB

Storage:
- 1 TB NVMe SSD

Suitable for:
- MVP
- short clips
- low-resolution iteration
- compositing development

## Recommended

CPU:
- Ryzen 7950X / Intel equivalent

RAM:
- 64 GB

GPU:
- RTX 4090

VRAM:
- 24 GB

Storage:
- 2 TB+ NVMe SSD

Suitable for:
- diffusion rendering
- batch processing
- longer shots
- repair workflows

## High-End Research

GPU:
- RTX 6000 Ada
- A6000
- H100 class

VRAM:
- 48–80 GB

Suitable for:
- model experimentation
- large diffusion workflows
- multi-shot processing

## Important Observation

GPU is NOT the only bottleneck.

Major bottlenecks:
- cache invalidation
- review workflow
- compositing iteration
- mask correction
- character consistency
- disk throughput
