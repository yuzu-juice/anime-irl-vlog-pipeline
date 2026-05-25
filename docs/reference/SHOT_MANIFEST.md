# Shot Manifest

Each shot is represented as deterministic metadata.

Example:

{
  "shot_id": "shot_001",
  "source_fps": 30,
  "anime_fps": 12,
  "frames": [
    {
      "source_frame": 12,
      "anim_frame": 4,
      "hold": 2,
      "view": "front_3q",
      "expression": "neutral",
      "left_foot_contact": true,
      "right_foot_contact": false
    }
  ]
}

## Purpose

The manifest allows:
- reproducibility
- partial rerender
- cache reuse
- deterministic repair
