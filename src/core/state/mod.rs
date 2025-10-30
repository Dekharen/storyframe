use crate::{
    core::{configuration::Configuration, step::StepAction},
    error::{ParseError, StepError},
};
use snapshot::StateSnapshot;
use std::fmt::Debug;

pub mod snapshot;
// ============================================================================
// VISUALIZATION STATE MANAGEMENT
// ============================================================================

/// Manages the current state of a visualization that can be modified by steps
pub trait VisualizationState: Send + 'static
where
    Self: Debug,
{
    type Step: StepAction;
    type Snapshot: StateSnapshot;
    /// Apply a single step to modify the state
    fn apply_step(&mut self, step: &Self::Step) -> Result<(), StepError>;

    /// Jump directly to a specific step index (may reset + replay)
    fn seek_to_step(
        &mut self,
        step_index: usize,
        all_steps: &[&Self::Step],
    ) -> Result<(), StepError>;

    /// Create a snapshot of current state for rendering
    fn create_snapshot(&self) -> Box<Self::Snapshot>;

    /// Returns the static type id of the state
    fn state_type_id() -> &'static str
    where
        Self: Sized;
    /// Returns the static type id of the snapshot associated
    fn snapshot_type_id() -> &'static str
    where
        Self: Sized,
    {
        Self::Snapshot::snapshot_type_id()
    }
    fn parse(input: &str, configuration: &Configuration) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait StateProxy: Send + Sync + 'static
where
    Self: Debug,
{
    /// Apply a single step to modify the state
    fn apply_step_erased(&mut self, step: &dyn StepAction) -> Result<(), StepError>;

    /// Jump directly to a specific step index (may reset + replay)
    fn seek_to_step_erased(
        &mut self,
        step_index: usize,
        all_steps: &[Box<dyn StepAction>],
    ) -> Result<(), StepError>;

    /// Create a snapshot of current state for rendering
    fn create_snapshot_erased(&self) -> Box<dyn StateSnapshot>;
}

impl<S: VisualizationState + Sync> StateProxy for S {
    fn apply_step_erased(&mut self, step: &dyn StepAction) -> Result<(), StepError> {
        let typed_step = step
            .as_any()
            .downcast_ref::<S::Step>()
            // TODO: Could add more info to this error type !
            .ok_or(StepError::IncompatibleStepType)?;

        self.apply_step(typed_step)
    }

    fn seek_to_step_erased(
        &mut self,
        step_index: usize,
        all_steps: &[Box<dyn StepAction>],
    ) -> Result<(), StepError> {
        let downcasted_steps: Result<Vec<&S::Step>, StepError> = all_steps
            .iter()
            .map(|step| {
                step.as_any()
                    .downcast_ref::<S::Step>()
                    .ok_or(StepError::IncompatibleStepType)
            })
            .collect();

        self.seek_to_step(step_index, &downcasted_steps?)
    }

    fn create_snapshot_erased(&self) -> Box<dyn StateSnapshot> {
        self.create_snapshot()
    }
}

pub type StateFactoryFn = fn(&str, &Configuration) -> Result<Box<dyn StateProxy>, ParseError>;
#[derive(Clone, Debug)]
pub struct StateInfo {
    pub type_id: &'static str,
    pub display_name: &'static str,
    pub snapshot_type_id: &'static str,
    pub factory: StateFactoryFn,
    pub required_config_fields: &'static [&'static str],
    pub is_default: bool,
}
