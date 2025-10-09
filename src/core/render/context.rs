use std::any::TypeId;
// ============================================================================
// RENDERING CONTEXT
// ============================================================================

/// Abstraction over different UI frameworks (egui, console, etc.)
pub trait RenderContext {
    fn tag_id(&self) -> TypeId;

    fn as_ptr(&mut self) -> *mut () {
        (self as *mut _) as *mut ()
    }
    // fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A required Context Tag that allows us to identify which context we're currently using.
/// This should not be implemented directly : It should be done through [` e`]
pub trait HasContextTag {
    type Tag: 'static;
}
