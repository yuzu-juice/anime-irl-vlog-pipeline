use crate::pipeline::stage::{Stage, StageKind};

pub struct Review;

impl Stage for Review {
    fn kind(&self) -> StageKind {
        StageKind::Review
    }

    fn requires(&self) -> Vec<String> {
        vec!["comp".into()]
    }

    fn produces(&self) -> Vec<String> {
        vec!["final".into()]
    }

    fn run(&self) -> anyhow::Result<()> {
        println!("  human-in-the-loop review (skipped in automation)");
        Ok(())
    }
}
