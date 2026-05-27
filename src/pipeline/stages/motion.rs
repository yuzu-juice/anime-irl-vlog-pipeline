use crate::pipeline::stage::{Stage, StageKind};

pub struct Motion;

impl Stage for Motion {
    fn kind(&self) -> StageKind {
        StageKind::Motion
    }

    fn requires(&self) -> Vec<String> {
        vec!["pose_raw".into()]
    }

    fn produces(&self) -> Vec<String> {
        vec!["pose_clean".into()]
    }

    fn run(&self) -> anyhow::Result<()> {
        println!("  smoothing, foot locking, hold insertion...");
        super::create_output_dirs(&self.produces())?;
        Ok(())
    }
}
