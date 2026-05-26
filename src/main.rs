mod pipeline;

use pipeline::stages::{analysis, composite, ingest, motion, render, review};
use pipeline::Pipeline;

fn main() -> anyhow::Result<()> {
    Pipeline::new()
        .then(ingest::Ingest)
        .then(analysis::Analysis)
        .then(motion::Motion)
        .then(render::Render)
        .then(composite::Composite)
        .then(review::Review)
        .run()
}
