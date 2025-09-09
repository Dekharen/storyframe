// ============================================================================
// RENDERER REGISTRY
// ============================================================================

use std::collections::HashMap;

use crate::step::StepAction;

use super::{Renderer, RendererProxy};

/// Registry for managing available renderers
pub struct RendererRegistry {
    renderers: HashMap<&'static str, Vec<Box<dyn RendererProxy>>>,
}

impl RendererRegistry {
    /// Creates a new [`RendererRegistry`].
    pub fn new() -> Self {
        Self {
            renderers: HashMap::new(),
        }
    }

    /// Register a renderer for a specific step type
    pub fn register_renderer<TStep, R>(&mut self, renderer: R)
    where
        TStep: StepAction + 'static,
        R: Renderer + Clone + 'static,
    {
        let step_type = TStep::step_type_id();
        self.renderers
            .entry(step_type)
            .or_default()
            .push(Box::new(renderer));
    }

    /// Get all registered renderers for a step type
    pub fn renderers_for_step_type(&self, step_type: &str) -> Option<&Vec<Box<dyn RendererProxy>>> {
        self.renderers.get(step_type)
    }

    /// Get first compatible renderer for a step type
    pub fn get_renderer_for_step_type(&self, step_type: &str) -> Option<Box<dyn RendererProxy>> {
        self.renderers
            .get(step_type)?
            .first()
            .map(|r| r.clone_boxed())
    }
}

impl Default for RendererRegistry {
    fn default() -> Self {
        Self::new()
    }
}
