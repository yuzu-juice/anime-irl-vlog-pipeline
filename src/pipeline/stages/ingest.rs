use crate::pipeline::stage::{Stage, StageKind};

pub struct Ingest;

impl Stage for Ingest {
    fn kind(&self) -> StageKind {
        StageKind::Ingest
    }

    fn run(&self) {
        println!("extracting frames from input video...");
    }
}
