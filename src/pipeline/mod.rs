pub mod stage;
pub mod stages;

use stage::Stage;
use std::path::Path;

pub struct Pipeline {
    stages: Vec<Box<dyn Stage>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { stages: vec![] }
    }

    pub fn then<S: Stage + 'static>(mut self, stage: S) -> Self {
        self.stages.push(Box::new(stage));
        self
    }

    #[cfg(test)]
    pub(crate) fn from_stages(stages: Vec<Box<dyn Stage>>) -> Self {
        Self { stages }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        for stage in &self.stages {
            validate_required_inputs(stage.as_ref())?;
            println!("[run]  {:?}", stage.kind());
            stage.run()?;
            validate_produced_outputs(stage.as_ref())?;
        }
        Ok(())
    }
}

fn validate_required_inputs(stage: &dyn Stage) -> anyhow::Result<()> {
    validate_existing_paths(stage.kind(), "missing inputs", stage.requires())
}

fn validate_produced_outputs(stage: &dyn Stage) -> anyhow::Result<()> {
    validate_existing_paths(stage.kind(), "missing outputs", stage.produces())
}

fn validate_existing_paths(
    kind: stage::StageKind,
    message: &str,
    paths: Vec<String>,
) -> anyhow::Result<()> {
    let missing: Vec<String> = paths
        .into_iter()
        .filter(|path| !Path::new(path).exists())
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        anyhow::bail!("{} for {:?}: {}", message, kind, missing.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stage::StageKind;
    use std::cell::RefCell;
    use std::path::Path;
    use std::rc::Rc;
    use tempfile::TempDir;

    struct MockStage {
        kind: StageKind,
        requires: Vec<String>,
        produces: Vec<String>,
        calls: Rc<RefCell<Vec<StageKind>>>,
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

        fn run(&self) -> anyhow::Result<()> {
            self.calls.borrow_mut().push(self.kind);
            Ok(())
        }
    }

    fn test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    fn path_string(path: &Path) -> String {
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn stages_run_in_order() {
        let calls = Rc::new(RefCell::new(vec![]));

        let pipeline = Pipeline::from_stages(vec![
            Box::new(MockStage {
                kind: StageKind::Ingest,
                requires: vec![],
                produces: vec![],
                calls: calls.clone(),
            }),
            Box::new(MockStage {
                kind: StageKind::Analysis,
                requires: vec![],
                produces: vec![],
                calls: calls.clone(),
            }),
            Box::new(MockStage {
                kind: StageKind::Motion,
                requires: vec![],
                produces: vec![],
                calls: calls.clone(),
            }),
        ]);

        pipeline.run().unwrap();

        assert_eq!(
            *calls.borrow(),
            vec![StageKind::Ingest, StageKind::Analysis, StageKind::Motion]
        );
    }

    #[test]
    fn missing_input_fails_before_running_stage() {
        let calls = Rc::new(RefCell::new(vec![]));
        let dir = test_dir();

        let pipeline = Pipeline::from_stages(vec![Box::new(MockStage {
            kind: StageKind::Ingest,
            requires: vec![path_string(&dir.path().join("missing-input"))],
            produces: vec![],
            calls: calls.clone(),
        })]);

        let err = pipeline.run().unwrap_err();

        assert!(err.to_string().contains("missing inputs"));
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn missing_output_fails_after_running_stage() {
        let calls = Rc::new(RefCell::new(vec![]));
        let dir = test_dir();

        let pipeline = Pipeline::from_stages(vec![Box::new(MockStage {
            kind: StageKind::Review,
            requires: vec![],
            produces: vec![path_string(&dir.path().join("missing-output"))],
            calls: calls.clone(),
        })]);

        let err = pipeline.run().unwrap_err();

        assert!(err.to_string().contains("missing outputs"));
        assert_eq!(*calls.borrow(), vec![StageKind::Review]);
    }
}
