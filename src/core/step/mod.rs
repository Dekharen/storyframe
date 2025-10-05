// ============================================================================
// STEP ACTIONS
// ============================================================================

use std::any::Any;
use std::fmt::Debug;

use crate::error::ParseError;
/// Represents a single modification that can be applied to visualization state
pub trait StepAction: Send + 'static
where
    Self: Debug,
{
    /// Type identifier for renderer compatibility (static method)
    fn type_id() -> &'static str
    where
        Self: Sized;

    /// Instance method to get type_id for trait objects
    fn get_type_id(&self) -> &'static str;

    /// For downcasting in renderers
    fn as_any(&self) -> &dyn Any;

    /// Parse a raw string into a step
    fn from_str(string: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}
