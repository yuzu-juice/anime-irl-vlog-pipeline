pub mod analysis;
pub mod composite;
pub mod ingest;
pub mod motion;
pub mod render;
pub mod review;

pub(crate) fn create_output_dirs(paths: &[String]) -> anyhow::Result<()> {
    for path in paths {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
