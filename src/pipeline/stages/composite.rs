use crate::pipeline::stage::{Stage, StageKind};

pub struct Composite;

impl Stage for Composite {
    fn kind(&self) -> StageKind {
        StageKind::Composite
    }

    fn run(&self) {
        println!("  compositing layers and exporting...");
    }
}
