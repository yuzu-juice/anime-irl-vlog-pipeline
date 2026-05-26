use crate::pipeline::context::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageKind {
    Ingest,
    Analysis,
    Motion,
    Render,
    Composite,
    Review,
}

pub trait Stage {
    fn kind(&self) -> StageKind;
    fn requires(&self) -> Vec<String>;
    fn produces(&self) -> Vec<String>;
    fn run(&self, ctx: &Context) -> anyhow::Result<()>;
}
