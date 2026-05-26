use crate::pipeline::context::Context;
use crate::pipeline::stage::{Stage, StageKind};

pub struct Composite;

impl Stage for Composite {
    fn kind(&self) -> StageKind {
        StageKind::Composite
    }

    fn requires(&self) -> Vec<String> {
        vec!["plate".into(), "render".into(), "masks".into()]
    }

    fn produces(&self) -> Vec<String> {
        vec!["comp".into()]
    }

    fn run(&self, _ctx: &Context) -> anyhow::Result<()> {
        println!("  compositing layers and exporting...");
        Ok(())
    }
}
