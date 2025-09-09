use std::any::Any;

/// Snapshot of state that can be rendered (immutable)
pub trait StateSnapshot: Send {
    fn as_any(&self) -> &dyn Any;
}
