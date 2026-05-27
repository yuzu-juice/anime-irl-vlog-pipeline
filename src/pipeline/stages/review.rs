use crate::pipeline::stage::{Stage, StageKind};

pub struct Review;

impl Stage for Review {
    fn kind(&self) -> StageKind {
        StageKind::Review
    }

    fn run(&self) {
        println!("  human-in-the-loop review (skipped in automation)");
    }
}
