use crate::pipeline::stage::{Stage, StageKind};

pub struct Analysis;

impl Stage for Analysis {
    fn kind(&self) -> StageKind {
        StageKind::Analysis
    }

    fn run(&self) {
        println!("  running segmentation, pose, depth extraction...");
    }
}
