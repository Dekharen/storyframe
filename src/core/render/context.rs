use std::any::Any;

use crate::core::id::ContextId;

// ============================================================================
// RENDERING CONTEXT
// ============================================================================

/// Abstraction over different UI frameworks (egui, console, etc.)
pub trait RenderContext {
    fn context_type_id() -> ContextId
    where
        Self: Sized;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
