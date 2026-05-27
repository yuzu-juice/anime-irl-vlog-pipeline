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

    pub fn run(&self) {
        for stage in &self.stages {
            println!("[run]  {:?}", stage.kind());
            stage.run();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stage::StageKind;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct MockStage {
        kind: StageKind,
        calls: Rc<RefCell<Vec<StageKind>>>,
    }

    impl Stage for MockStage {
        fn kind(&self) -> StageKind {
            self.kind
        }

        fn run(&self) {
            self.calls.borrow_mut().push(self.kind);
        }
    }

    #[test]
    fn stages_run_in_order() {
        let calls = Rc::new(RefCell::new(vec![]));

        let pipeline = Pipeline::from_stages(vec![
            Box::new(MockStage {
                kind: StageKind::Ingest,
                calls: calls.clone(),
            }),
            Box::new(MockStage {
                kind: StageKind::Analysis,
                calls: calls.clone(),
            }),
            Box::new(MockStage {
                kind: StageKind::Motion,
                calls: calls.clone(),
            }),
        ]);

        pipeline.run();

        assert_eq!(
            *calls.borrow(),
            vec![StageKind::Ingest, StageKind::Analysis, StageKind::Motion]
        );
    }
}
