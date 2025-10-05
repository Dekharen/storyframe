use crate::{core::state::snapshot::StateSnapshot, error::RenderError};
use context::RenderContext;

use super::id::{RendererId, StateId};

pub mod context;
// pub mod registry;
// ============================================================================
// RENDERING SYSTEM
// ============================================================================

/// Generic renderer for a specific step type
pub trait Renderer: Send + Clone + 'static {
    type StateSnapshot: StateSnapshot;
    type Context: RenderContext;

    /// Render a step application with the current state
    fn render_state(&mut self, snapshot: &Self::StateSnapshot, context: &mut Self::Context);

    /// Get human-readable name for UI selection
    fn renderer_name(&self) -> RendererId;

    // TODO: either remove or change this
    //
    //
    //    /// Check if this renderer can work with the given visualization state type
    //    fn supports_state_type(&self, state_type: &str) -> bool;
}
/// Type-erased wrapper for storing different renderer types
pub trait RendererProxy: Send {
    fn state_type_id(&self) -> StateId;
    fn renderer_name(&self) -> RendererId;
    // fn supports_state_type(&self, state_type: &str) -> bool;

    fn render_state_erased(
        &mut self,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    ) -> Result<(), RenderError>;

    fn clone_boxed(&self) -> Box<dyn RendererProxy>;
}

/// Blanket implementation to convert any Renderer of T into RendererProxy
impl<R: Renderer> RendererProxy for R {
    fn state_type_id(&self) -> StateId {
        R::StateSnapshot::snapshot_type_id()
    }

    fn renderer_name(&self) -> RendererId {
        self.renderer_name()
    }

    // fn supports_state_type(&self, state_type: &str) -> bool {
    //     self.supports_state_type(state_type)
    // }

    fn render_state_erased(
        &mut self,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    ) -> Result<(), RenderError> {
        let typed_snapshot = snapshot
            .as_any()
            .downcast_ref::<R::StateSnapshot>()
            .ok_or_else(|| RenderError::IncompatibleState(R::StateSnapshot::snapshot_type_id()))?;

        let typed_context = context
            .as_any_mut()
            .downcast_mut::<R::Context>()
            .ok_or_else(|| RenderError::IncompatibleContext(R::Context::context_type_id()))?;
        self.render_state(typed_snapshot, typed_context);
        Ok(())
    }
    fn clone_boxed(&self) -> Box<dyn RendererProxy> {
        Box::new(self.clone())
    }
}
