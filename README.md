# anime-irl

Full-Rust architecture design repository for an OSS "anime IRL vlog" pipeline.

Goal:
Replace a real human performer in vlog footage with a low-fps 2D anime self,
while preserving emotional timing, grounding, and compositing quality.

This repository intentionally contains:
- architecture documents
- pipeline specifications
- manifest schemas
- system design
- workflow planning

This repository intentionally does NOT contain:
- implementation code
- ML training code
- model weights
- ComfyUI graphs
- inference runtime code

Core philosophy:

"Anime IRL" is not primarily a video generation problem.
It is a:
- compositing problem
- character consistency problem
- timing problem
- grounding problem
- low-fps animation problem

## System Philosophy

Background:
- real footage
- high fps (24/30fps)

Character:
- 2D anime
- low fps (8–12fps)
- stepped animation
- intentional frame holding

## Core Stack

- Rust
- ffmpeg
- OpenCV
- external ML workers
- filesystem artifact pipeline

## External ML Workers

ML inference is intentionally externalized.

Examples:
- pose estimation
- segmentation
- depth estimation
- diffusion rendering

The Rust core communicates through:
- manifest files
- HTTP
- gRPC
- filesystem queues

## Repository Structure

See `docs/`.
