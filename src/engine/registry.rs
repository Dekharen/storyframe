// ============================================================================
// RENDERER REGISTRY
// ============================================================================

use std::any::TypeId;
use std::collections::HashMap;

use super::RendererProxy;
use crate::core::state::snapshot::StateSnapshot;
use crate::core::state::StateInfo;
pub use crate::domains::DomainRegistry;
use crate::{HasContextTag, Renderer};
// Registry for managing available renderers
pub struct RendererRegistry {
    renderers: HashMap<RendererKey, Vec<Box<dyn RendererProxy>>>,
}
impl RendererRegistry {
    /// Creates a new [`RendererRegistry`].
    pub fn new() -> Self {
        Self {
            renderers: HashMap::new(),
        }
    }
    // TODO: if possibly, simplify this type syntax. TypeId::of<type> is very wordy
    pub fn register_renderer<R>(&mut self, renderer: R)
    where
        R: Renderer + Sync + 'static,
    {
        let key = RendererKey::new(
            R::StateSnapshot::snapshot_type_id(),
            TypeId::of::<<<R as Renderer>::Context<'_> as HasContextTag>::Tag>(),
        );
        self.renderers
            .entry(key)
            .or_default()
            .push(Box::new(renderer));
    }

    pub fn get_renderers(
        &self,
        snapshot_type: &str,
        context_type: TypeId,
    ) -> Option<&Vec<Box<dyn RendererProxy>>> {
        let key = RendererKey::new(snapshot_type, context_type);
        self.renderers.get(&key)
    }
    pub fn get_renderers_id(
        &self,
        state_type: &str,
        context_type: TypeId,
    ) -> Option<Vec<&'static str>> {
        let renderers = self.get_renderers(state_type, context_type)?;
        Some(
            renderers
                .iter()
                .map(|renderer| renderer.renderer_name())
                .collect(),
        )
    }
    pub fn get_first_renderer(
        &self,
        state_type: &str,
        context_type: TypeId,
    ) -> Option<&dyn RendererProxy> {
        self.get_renderers(state_type, context_type)?
            .first()
            .map(|boxed| boxed.as_ref())
    }
}

impl Default for RendererRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RendererKey {
    pub snapshot_type: String,
    pub context_type: TypeId,
}

impl RendererKey {
    pub fn new(snapshot_type: &str, context_type: TypeId) -> Self {
        Self {
            snapshot_type: snapshot_type.to_string(),
            context_type,
        }
    }
}

// // Optional: if I want tuple-like access
// impl From<(&str, &str)> for RendererKey {
//     fn from((state_type, context_type): (&str, &str)) -> Self {
//         Self::new(state_type, context_type)
//     }
// }

pub struct StateRegistry {
    states: HashMap<&'static str, Vec<StateInfo>>,
}

impl StateRegistry {
    pub fn get(&self, step_type: &str) -> Option<&Vec<StateInfo>> {
        self.states.get(step_type)
    }
    pub fn from_mappings(mapping: &HashMap<&'static str, Vec<StateInfo>>) -> Self {
        Self {
            states: mapping.clone(),
        }
    }
}

pub struct Registry {
    renderer_registry: RendererRegistry,
    state_registry: StateRegistry,
    domain_registry: DomainRegistry,
}

impl Registry {
    pub fn new(
        renderer_registry: RendererRegistry,
        state_registry: StateRegistry,
        domain_registry: DomainRegistry,
    ) -> Self {
        Self {
            renderer_registry,
            state_registry,
            domain_registry,
        }
    }

    pub fn renderer_registry_mut(&mut self) -> &mut RendererRegistry {
        &mut self.renderer_registry
    }
    pub fn state_registry_mut(&mut self) -> &mut StateRegistry {
        &mut self.state_registry
    }
    pub fn domain_registry_mut(&mut self) -> &mut DomainRegistry {
        &mut self.domain_registry
    }

    pub fn renderer_registry(&self) -> &RendererRegistry {
        &self.renderer_registry
    }
    pub fn state_registry(&self) -> &StateRegistry {
        &self.state_registry
    }
    pub fn domain_registry(&self) -> &DomainRegistry {
        &self.domain_registry
    }
}
