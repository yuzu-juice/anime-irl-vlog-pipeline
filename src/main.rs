mod pipeline;

use pipeline::context::Context;
use pipeline::stages::{analysis, composite, ingest, motion, render, review};
use pipeline::Pipeline;

fn main() -> anyhow::Result<()> {
    let ctx = Context {
        project_root: "test-project".to_string(),
    };
    Pipeline::new()
        .then(ingest::Ingest)
        .then(analysis::Analysis)
        .then(motion::Motion)
        .then(render::Render)
        .then(composite::Composite)
        .then(review::Review)
        .run(&ctx)?;
    Ok(())
}
