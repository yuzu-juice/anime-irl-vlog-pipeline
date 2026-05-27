use crate::pipeline::stage::{Stage, StageKind};

pub struct Motion;

impl Stage for Motion {
    fn kind(&self) -> StageKind {
        StageKind::Motion
    }

    fn run(&self) {
        println!("smoothing, foot locking, hold insertion...");
    }
}
