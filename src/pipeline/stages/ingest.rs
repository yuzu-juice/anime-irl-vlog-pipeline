use crate::pipeline::stage::{Stage, StageKind};

pub struct Ingest;

impl Stage for Ingest {
    fn kind(&self) -> StageKind {
        StageKind::Ingest
    }

    fn requires(&self) -> Vec<String> {
        vec!["input.mp4".into()]
    }

    fn produces(&self) -> Vec<String> {
        vec!["frames".into()]
    }

    fn run(&self) -> anyhow::Result<()> {
        println!("  extracting frames from input video...");
        super::create_output_dirs(&self.produces())?;
        Ok(())
    }
}
