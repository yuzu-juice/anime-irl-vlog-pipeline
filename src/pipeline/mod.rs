pub mod cache;
pub mod stage;
pub mod stages;

use stage::Stage;

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
            if cache::needs_run(stage.as_ref()) {
                println!("[run]  {:?}", stage.kind());
                stage.run()?;
            } else {
                println!("[skip] {:?}", stage.kind());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stage::StageKind;
    use std::cell::RefCell;
    use std::fs;
    use std::rc::Rc;
    use std::thread;
    use std::time::Duration;

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

    #[test]
    fn stages_run_in_order() {
        let calls = Rc::new(RefCell::new(vec![]));

        let pipeline = Pipeline::from_stages(vec![
            Box::new(MockStage {
                kind: stage::StageKind::Ingest,
                requires: vec![],
                produces: vec![],
                calls: calls.clone(),
            }),
            Box::new(MockStage {
                kind: stage::StageKind::Analysis,
                requires: vec![],
                produces: vec![],
                calls: calls.clone(),
            }),
            Box::new(MockStage {
                kind: stage::StageKind::Motion,
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
    fn stage_is_skipped_when_outputs_are_fresh() {
        let dir = "test-pipeline-skip";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        let inp = format!("{}/input", dir);
        let out = format!("{}/output", dir);

        fs::write(&inp, "x").unwrap();
        thread::sleep(Duration::from_millis(10));
        fs::write(&out, "x").unwrap();

        let calls = Rc::new(RefCell::new(vec![]));

        let pipeline = Pipeline::from_stages(vec![Box::new(MockStage {
            kind: stage::StageKind::Ingest,
            requires: vec![inp],
            produces: vec![out],
            calls: calls.clone(),
        })]);

        pipeline.run().unwrap();

        assert!(calls.borrow().is_empty());

        let _ = fs::remove_dir_all(dir);
    }
}
