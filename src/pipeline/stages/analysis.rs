use crate::pipeline::stage::{Stage, StageKind};

pub struct Analysis;

impl Stage for Analysis {
    fn kind(&self) -> StageKind {
        StageKind::Analysis
    }

    fn requires(&self) -> Vec<String> {
        vec!["frames".into()]
    }

    fn produces(&self) -> Vec<String> {
        vec![
            "masks".into(),
            "depth".into(),
            "pose_raw".into(),
            "plate".into(),
        ]
    }

    fn run(&self) -> anyhow::Result<()> {
        println!("  running segmentation, pose, depth extraction...");
        Ok(())
    }
}
