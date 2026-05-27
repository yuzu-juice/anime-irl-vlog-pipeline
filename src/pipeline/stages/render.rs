use crate::pipeline::stage::{Stage, StageKind};

pub struct Render;

impl Stage for Render {
    fn kind(&self) -> StageKind {
        StageKind::Render
    }

    fn requires(&self) -> Vec<String> {
        vec!["pose_clean".into(), "depth".into()]
    }

    fn produces(&self) -> Vec<String> {
        vec!["render".into()]
    }

    fn run(&self) -> anyhow::Result<()> {
        println!("  generating anime cel frames...");
        Ok(())
    }
}
