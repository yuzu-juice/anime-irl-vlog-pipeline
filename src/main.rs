mod pipeline;

use pipeline::Pipeline;
use pipeline::stages::{analysis, composite, ingest, motion, render, review};

fn main() {
    Pipeline::new()
        .then(ingest::Ingest)
        .then(analysis::Analysis)
        .then(motion::Motion)
        .then(render::Render)
        .then(composite::Composite)
        .then(review::Review)
        .run();
}
