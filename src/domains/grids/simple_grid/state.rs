use crate::{
    core::{
        configuration::Configuration,
        state::{VisualizationState, snapshot::StateSnapshot},
    },
    error::ParseError,
};
use std::{num::ParseFloatError, ops::Deref};

#[derive(Debug, Clone)]
pub struct SimpleGridCell {
    pub content: f32,
    pub color: Option<String>,
}

#[derive(Debug)]
pub struct SimpleGridState {
    content: Vec<SimpleGridCell>,
    col: usize,
    row: usize,
}

impl StateSnapshot for SimpleGridSnapshot {
    fn snapshot_type_id() -> &'static str
    where
        Self: Sized,
    {
        "text_snapshot"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
pub struct SimpleGridSnapshot {
    content: Vec<SimpleGridCell>,
    col: usize,
    row: usize,
}

impl Deref for SimpleGridSnapshot {
    type Target = Vec<SimpleGridCell>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl From<&SimpleGridState> for SimpleGridSnapshot {
    fn from(value: &SimpleGridState) -> Self {
        Self {
            content: value.content.clone(),
            col: value.col,
            row: value.row,
        }
    }
}

impl VisualizationState for SimpleGridState {
    type Step = super::SimpleF32GridStep;
    type Snapshot = SimpleGridSnapshot;
    // TODO :
    // Do we want the type of StepAction to be embedded in VisualizationState ?
    // This is reasonable... Creating duplicate states isn't really an issue.
    // But I like the cross-referencing...
    fn apply_step(&mut self, step: &Self::Step) -> Result<(), crate::error::StepError> {
        let [x, y] = step.position;
        if x > self.row {
            return Err(crate::error::StepError::InvalidPosition(step.position[0]));
        }
        if y > self.row {
            return Err(crate::error::StepError::InvalidPosition(step.position[1]));
        }
        let representation = &mut self.content[x * y];
        if let Some(content) = step.content {
            representation.content = content;
        };
        if let Some(color) = step.color.clone() {
            representation.color = Some(color);
        }
        Ok(())
    }

    fn seek_to_step(
        &mut self,
        step_index: usize,
        all_steps: &[&Self::Step],
    ) -> Result<(), crate::error::StepError> {
        todo!()
    }
    fn create_snapshot(&self) -> Box<Self::Snapshot> {
        Box::new(SimpleGridSnapshot::from(self))
    }
    fn state_type_id() -> &'static str
    where
        Self: Sized,
    {
        "simple_grid_state"
    }

    fn parse(input: &str, configuration: &Configuration) -> Result<Self, crate::error::ParseError>
    where
        Self: Sized,
    {
        //TODO : Make the separators an option to parse
        let separator = ":";
        let content: Result<Vec<SimpleGridCell>, ParseFloatError> = input
            .split(separator)
            .map(|txt| {
                Ok(SimpleGridCell {
                    content: txt.parse()?,
                    color: None,
                })
            })
            .collect();
        let col = configuration
            .get("columns")
            .and_then(|f| f.as_leaf())
            .ok_or_else(|| ParseError::RequiredConfigField("columns".to_string()))?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Columns field malformed".to_string()))?;

        let row = configuration
            .get("rows")
            .and_then(|f| f.as_leaf())
            .ok_or_else(|| ParseError::RequiredConfigField("rows".to_string()))?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Row field malformed".to_string()))?;

        // FIXME: I need to start implementing the err types From std to avoid these map_err
        Ok(Self {
            content: content.map_err(|_| ParseError::InvalidFormat(input.to_string()))?,
            col,
            row,
        })
    }
}
