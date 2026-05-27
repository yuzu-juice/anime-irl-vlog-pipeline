use crate::pipeline::stage::{Stage, StageKind};

pub struct Render;

impl Stage for Render {
    fn kind(&self) -> StageKind {
        StageKind::Render
    }

    fn run(&self) {
        println!("  generating anime cel frames...");
    }
}
