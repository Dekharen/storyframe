use crate::{core::step::StepAction, error::StepError};
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
    /// Apply a single step to modify the state
    fn apply_step(&mut self, step: &dyn StepAction) -> Result<(), StepError>;

    /// Reset to the initial state
    fn reset_to_initial(&mut self);

    /// Jump directly to a specific step index (may reset + replay)
    fn seek_to_step(
        &mut self,
        step_index: usize,
        all_steps: &[Box<dyn StepAction>],
    ) -> Result<(), StepError>;

    /// Get the current step index being displayed
    fn current_step_index(&self) -> usize;

    /// Create a snapshot of current state for rendering
    fn create_snapshot(&self) -> Box<dyn StateSnapshot>;
}
