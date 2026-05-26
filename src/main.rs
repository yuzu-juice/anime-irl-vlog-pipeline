mod pipeline;

use pipeline::Pipeline;
use pipeline::context::Context;
use pipeline::stages::{analysis, composite, ingest, motion, render, review};

fn main() -> anyhow::Result<()> {
    let ctx = Context {
        project_root: ".".to_string(),
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
