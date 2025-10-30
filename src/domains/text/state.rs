use crate::core::{
    configuration::Configuration,
    state::{VisualizationState, snapshot::StateSnapshot},
};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct TextRepresentation {
    pub content: String,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>,
}

#[derive(Debug)]
pub struct TextState {
    content: Vec<TextRepresentation>,
}
impl StateSnapshot for TextSnapshot {
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
pub struct TextSnapshot(pub Vec<TextRepresentation>);

impl Deref for TextSnapshot {
    type Target = Vec<TextRepresentation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl VisualizationState for TextState {
    type Step = super::TextStep;
    type Snapshot = TextSnapshot;
    // TODO :
    // Do we want the type of StepAction to be embedded in VisualizationState ?
    // This is reasonable... Creating duplicate states isn't really an issue.
    // But I like the cross-referencing...
    fn apply_step(&mut self, step: &Self::Step) -> Result<(), crate::error::StepError> {
        if step.position >= self.content.len() {
            //TODO: this could maybe be done right after parsing for each step.
            //But some step types might only be able to validate based on the state - so
            //We either trust the input entirely, or we should make InvalidPosition a more
            //general error.

            return Err(crate::error::StepError::InvalidPosition(step.position));
        }
        let representation = &mut self.content[step.position];
        if let Some(content) = step.content.clone() {
            representation.content = content;
        }
        if let Some(color) = step.background_color.clone() {
            representation.background_color = Some(color);
        }
        if let Some(color) = step.foreground_color.clone() {
            representation.foreground_color = Some(color);
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
        Box::new(TextSnapshot(self.content.clone()))
    }
    fn state_type_id() -> &'static str
    where
        Self: Sized,
    {
        "text_state"
    }

    //TODO: Could implement a configuration for bg/fg color
    fn parse(input: &str, _configuration: &Configuration) -> Result<Self, crate::error::ParseError>
    where
        Self: Sized,
    {
        //TODO : Make the separators an option to parse
        let separator = "_";
        let content = input
            .split(separator)
            .map(|txt| TextRepresentation {
                content: txt.to_string(),
                background_color: None,
                foreground_color: None,
            })
            .collect();
        Ok(Self { content })
    }
}
