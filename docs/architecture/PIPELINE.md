# Pipeline

## Stage A — Ingest

Input:
- vlog footage

Outputs:
- source frames
- anime cadence frames

Example:
- source: 30fps
- anime: 12fps

## Stage B — Shot Analysis

Outputs:
- actor masks
- clean plates
- pose
- depth
- occluders
- floor plane
- camera motion

## Stage C — Motion Abstraction

Converts real human motion into anime-readable motion.

Includes:
- smoothing
- foot locking
- hold insertion
- cadence simplification
- view classification

## Stage D — Character Rendering

Input:
- cleaned pose
- character pack
- timing metadata

Output:
- anime cel frames
- alpha
- repair masks

## Stage E — Compositing

Composition order:

clean background
→ anime character
→ contact shadows
→ foreground occluders
→ grading
→ export

## Stage F — Review

Human-in-the-loop correction layer.

Required operations:
- regenerate frame
- repair face
- repair alpha
- adjust foot placement
- edit occlusion masks
- adjust exposure timing
