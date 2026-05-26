use std::fs;
use std::path::Path;

use crate::pipeline::context::Context;
use crate::pipeline::stage::Stage;

pub fn needs_run(stage: &dyn Stage, ctx: &Context) -> bool {
    let output_files: Vec<String> = stage
        .produces()
        .iter()
        .map(|p| format!("{}/{}", ctx.project_root, p))
        .collect();
    if output_files.is_empty() {
        return true;
    }
    if output_files
        .iter()
        .any(|path| !Path::new(path).exists())
    {
        return true;
    }
    let oldest_output = output_files
        .iter()
        .filter_map(|path| fs::metadata(path).ok().and_then(|m| m.modified().ok()))
        .min();
    let input_files: Vec<String> = stage
        .requires()
        .iter()
        .map(|p| format!("{}/{}", ctx.project_root, p))
        .collect();
    let newest_input = input_files
        .iter()
        .filter_map(|path| fs::metadata(path).ok().and_then(|m| m.modified().ok()))
        .max();
    match (oldest_output, newest_input) {
        (Some(out_time), Some(in_time)) => in_time > out_time,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use crate::pipeline::stage::StageKind;

    struct MockStage {
        kind: StageKind,
        requires: Vec<String>,
        produces: Vec<String>,
    }

    impl Stage for MockStage {
        fn kind(&self) -> StageKind {
            self.kind
        }
        fn requires(&self) -> Vec<String> {
            self.requires.clone()
        }
        fn produces(&self) -> Vec<String> {
            self.produces.clone()
        }
        fn run(&self, _ctx: &Context) -> anyhow::Result<()> {
            Ok(())
        }
    }

    fn ctx(dir: &str) -> Context {
        Context {
            project_root: dir.to_string(),
        }
    }

    #[test]
    fn empty_outputs_needs_run() {
        let stage = MockStage {
            kind: StageKind::Ingest,
            requires: vec![],
            produces: vec![],
        };
        assert!(needs_run(&stage, &ctx("test-cache-empty")));
    }

    #[test]
    fn missing_output_needs_run() {
        let stage = MockStage {
            kind: StageKind::Ingest,
            requires: vec![],
            produces: vec!["nope".into()],
        };
        assert!(needs_run(&stage, &ctx("test-cache-missing")));
    }

    #[test]
    fn outputs_newer_does_not_need_run() {
        let dir = "test-cache-older";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();
        fs::write(format!("{}/input", dir), "x").unwrap();
        thread::sleep(Duration::from_millis(10));
        fs::write(format!("{}/output", dir), "x").unwrap();

        let stage = MockStage {
            kind: StageKind::Ingest,
            requires: vec!["input".into()],
            produces: vec!["output".into()],
        };
        assert!(!needs_run(&stage, &ctx(dir)));

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn input_newer_needs_run() {
        let dir = "test-cache-newer";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();
        fs::write(format!("{}/output", dir), "x").unwrap();
        thread::sleep(Duration::from_millis(10));
        fs::write(format!("{}/input", dir), "x").unwrap();

        let stage = MockStage {
            kind: StageKind::Ingest,
            requires: vec!["input".into()],
            produces: vec!["output".into()],
        };
        assert!(needs_run(&stage, &ctx(dir)));

        let _ = fs::remove_dir_all(dir);
    }
}
