# Architecture

## High-Level Design

The system is divided into five layers:

1. Shot Analysis Layer
2. Motion Abstraction Layer
3. Character Rendering Layer
4. Compositing Layer
5. Review / Repair Layer

## Principle

The system is:
- pass-oriented
- deterministic
- inspectable
- reproducible
- cacheable

NOT:
- monolithic AI generation
- prompt-only workflow
- end-to-end black box

## Runtime Philosophy

Rust owns:
- orchestration
- manifests
- asset graph
- caching
- compositing
- review tooling
- timeline logic

ML workers own:
- segmentation
- pose extraction
- diffusion rendering
- inpainting
- depth estimation

## Artifact-Oriented Design

Every stage outputs files.

Example:

shot/
  masks/
  depth/
  pose_raw/
  pose_clean/
  plate/
  render/
  comp/
  final/

No stage should require opaque in-memory coupling.
