// ============================================================================
// STEP ACTIONS
// ============================================================================

use std::any::Any;
use std::fmt::Debug;
/// Represents a single modification that can be applied to visualization state
pub trait StepAction: Send + 'static
where
    Self: Debug,
{
    /// For debugging and logging
    fn description(&self) -> String;

    /// Type identifier for renderer compatibility (static method)
    fn step_type_id() -> &'static str
    where
        Self: Sized;

    /// Instance method to get type_id for trait objects
    fn get_type_id(&self) -> &'static str;

    /// For downcasting in renderers
    fn as_any(&self) -> &dyn Any;
}
