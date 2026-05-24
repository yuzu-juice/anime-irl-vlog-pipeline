# Character Pack Specification

Character packs define a stable anime identity.

## Goals

Preserve:
- face
- silhouette
- hair
- clothing
- palette
- emotional readability

## Structure

character_pack/
  refs/
  expressions/
  palettes/
  prompts/
  metadata.json

## Required Views

- front
- front_3q
- side
- back_3q
- back

## Metadata Example

{
  "name": "anime_self",
  "height_ratio": 0.94,
  "style": "slice-of-life anime",
  "cadence": "12fps stepped",
  "palette_lock": true
}

## Important Rule

A character pack is NOT:
- a prompt
- a single image
- a LoRA alone

It is a reproducible identity system.
