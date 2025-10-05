use std::any::Any;

/// Snapshot of state that can be rendered (immutable)
pub trait StateSnapshot: Send {
    fn snapshot_type_id() -> &'static str
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
}
