use context::RenderContext;

use crate::{core::state::snapshot::StateSnapshot, core::step::StepAction};

pub mod context;
// pub mod registry;
// ============================================================================
// RENDERING SYSTEM
// ============================================================================

/// Generic renderer for a specific step type
pub trait Renderer: Send + Clone + 'static {
    type Step: StepAction;

    /// Render a step application with the current state
    fn render_step(
        &mut self,
        step: &Self::Step,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    );

    /// Render the current state without any step highlight
    fn render_state(&mut self, snapshot: &dyn StateSnapshot, context: &mut dyn RenderContext);

    /// Get human-readable name for UI selection
    fn renderer_name(&self) -> &'static str;

    /// Check if this renderer can work with the given visualization state type
    fn supports_state_type(&self, state_type: &str) -> bool;
}
/// Type-erased wrapper for storing different renderer types
pub trait RendererProxy: Send {
    fn step_type_id(&self) -> &'static str;
    fn renderer_name(&self) -> &'static str;
    fn supports_state_type(&self, state_type: &str) -> bool;

    fn render_step_erased(
        &mut self,
        step: &dyn StepAction,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    );

    fn render_state_erased(
        &mut self,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    );

    fn clone_boxed(&self) -> Box<dyn RendererProxy>;
}

/// Blanket implementation to convert any Renderer of T into RendererProxy
impl<R: Renderer> RendererProxy for R {
    fn step_type_id(&self) -> &'static str {
        R::Step::step_type_id()
    }

    fn renderer_name(&self) -> &'static str {
        self.renderer_name()
    }

    fn supports_state_type(&self, state_type: &str) -> bool {
        self.supports_state_type(state_type)
    }

    fn render_step_erased(
        &mut self,
        step: &dyn StepAction,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    ) {
        let typed_step = step.as_any().downcast_ref::<R::Step>().unwrap_or_else(|| {
            panic!(
                "Registry provided incompatible step: expected {}, got {}",
                R::Step::step_type_id(),
                step.get_type_id()
            )
        });
        self.render_step(typed_step, snapshot, context);
    }

    fn render_state_erased(
        &mut self,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    ) {
        self.render_state(snapshot, context);
    }

    fn clone_boxed(&self) -> Box<dyn RendererProxy> {
        Box::new(self.clone())
    }
}
