// ============================================================================
// VISUALIZATION ENGINE
// ============================================================================

pub mod registry;
pub mod selectors;
use std::{any::TypeId, marker::PhantomData};

use crate::{
    HasContextTag, RenderContext,
    algorithm::{AlgorithmInstance, Current, Metadata, PartInfo, PuzzleSource},
    core::{render::RendererProxy, state::StateInfo},
    error::{ParseError, VisualizationError},
};
use registry::Registry;
use selectors::{PartSelector, RendererSelector, StateSelector};
/// Manages renderers and coordinates visualization
pub struct VisualizationEngine {
    puzzle: Option<AlgorithmInstance>,
    active_renderer: Option<Box<dyn RendererProxy>>,
    registry: Registry,
    // registry: RendererRegistry,
    // domain_registry: DomainRegistry,
}

impl VisualizationEngine {
    pub fn new() -> Self {
        Self::with_registry(crate::domains::create_registry())
    }

    pub fn from_source(source: PuzzleSource) -> Result<Self, ParseError> {
        let mut engine = Self::new();
        engine.load_puzzle_from_source(source)?;
        Ok(engine)
    }
    pub fn with_registry(registry: Registry) -> Self {
        Self {
            registry,
            // registry: RendererRegistry::new(),
            puzzle: None,
            active_renderer: None,
        }
    }
    pub fn from_source_with_registry(
        source: PuzzleSource,
        registry: Registry,
    ) -> Result<Self, ParseError> {
        let mut engine = Self::with_registry(registry);
        engine.load_puzzle_from_source(source)?;
        Ok(engine)
    }
    fn load_puzzle_from_source(&mut self, source: PuzzleSource) -> Result<(), ParseError> {
        let puzzle = AlgorithmInstance::from_source(source, self.registry.domain_registry())?;
        self.puzzle = Some(puzzle);
        // TODO: Set up initial current state...
        Ok(())
    }

    pub fn configure_for_current_context<C: RenderContext + HasContextTag + 'static>(
        &'_ mut self,
    ) -> ContextConfiguration<'_, C> {
        ContextConfiguration {
            engine: self,
            context_type: TypeId::of::<C::Tag>(),
            _phantom: PhantomData,
        }
    }

    pub fn register_renderer<R>(&mut self, renderer: R)
    where
        R: crate::core::render::Renderer + Sync,
    {
        self.registry
            .renderer_registry_mut()
            .register_renderer(renderer)
    }

    pub fn get_parts(&self) -> Result<&[PartInfo], VisualizationError> {
        Ok(&self
            .puzzle
            .as_ref()
            .ok_or(VisualizationError::NoPuzzleLoaded)?
            .parts)
    }
    pub fn get_metadata(&self) -> Result<Metadata, VisualizationError> {
        let metadata = self
            .puzzle
            .as_ref()
            .ok_or(VisualizationError::NoPuzzleLoaded)?
            .metadata
            .clone();
        Ok(metadata)
    }
    pub fn select_part(
        &mut self,
        selection: fn(&mut PartSelector),
    ) -> Result<(), VisualizationError> {
        let mut selector = PartSelector::from_parts(
            &self
                .puzzle
                .as_ref()
                .ok_or(VisualizationError::NoPuzzleLoaded)?
                .parts,
        );
        selection(&mut selector);
        let select = selector
            .resolve_selection()
            .ok_or(VisualizationError::NoPartLoaded)?;
        self.puzzle.as_mut().unwrap().current = Some(Current {
            step: 0,
            part_id: select.id.clone(),
        });
        self.active_renderer = None;
        self.puzzle.as_mut().unwrap().state = None;
        Ok(())
    }
    // ============================================================================
    // STEPPING CONTROL METHODS
    // ============================================================================
    /// Execute the next step in the sequence
    pub fn next_step(&mut self) -> Result<(), VisualizationError> {
        let puzzle = self
            .puzzle
            .as_mut()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let current = puzzle
            .current
            .as_mut()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        let state = puzzle
            .state
            .as_mut()
            .ok_or(VisualizationError::MissingState)?;
        if current.step < part.steps.len() {
            let step = &part.steps[current.step];
            state.inner.apply_step_erased(step.as_ref())?;
            current.step += 1;
            Ok(())
        } else {
            Err(VisualizationError::AlreadyAtEnd)
        }
    }

    /// Go back one step (may require replaying from beginning)
    pub fn previous_step(&mut self) -> Result<(), VisualizationError> {
        let puzzle = self
            .puzzle
            .as_mut()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let current = puzzle
            .current
            .as_mut()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let state = puzzle
            .state
            .as_mut()
            .ok_or(VisualizationError::MissingState)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        if current.step > 0 {
            let target_step = current.step - 1;
            state.inner.seek_to_step_erased(target_step, &part.steps)?;
            current.step = target_step;
            Ok(())
        } else {
            Err(VisualizationError::AlreadyAtBeginning)
        }
    }

    /// Jump directly to a specific step
    pub fn goto_step(&mut self, step_index: usize) -> Result<(), VisualizationError> {
        let puzzle = self
            .puzzle
            .as_mut()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let state = puzzle
            .state
            .as_mut()
            .ok_or(VisualizationError::MissingState)?;
        let current = puzzle
            .current
            .as_mut()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        if step_index <= part.steps.len() {
            state.inner.seek_to_step_erased(step_index, &part.steps)?;
            current.step = step_index;
            Ok(())
        } else {
            Err(VisualizationError::InvalidStepIndex(step_index))
        }
    }

    /// Reset to the beginning
    pub fn reset(&mut self) -> Result<(), VisualizationError> {
        let puzzle = self
            .puzzle
            .as_mut()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let current = puzzle
            .current
            .as_mut()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let state = puzzle
            .state
            .as_mut()
            .ok_or(VisualizationError::MissingState)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        state
            .reset(&part.input_data, &part.configuration)
            .ok()
            .ok_or(VisualizationError::MissingState)?;
        current.step = 0;
        Ok(())
    }

    /// Play through all remaining steps automatically
    pub fn play_to_end(&mut self) -> Result<(), VisualizationError> {
        while self.can_step_forward()? {
            self.next_step()?;
        }
        Ok(())
    }

    // ============================================================================
    // STATE QUERY METHODS
    // ============================================================================

    /// Check if we can step forward
    pub fn can_step_forward(&self) -> Result<bool, VisualizationError> {
        let puzzle = self
            .puzzle
            .as_ref()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let current = puzzle
            .current
            .as_ref()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        Ok(current.step < part.steps.len())
    }

    /// Check if we can step backward
    pub fn can_step_backward(&self) -> Result<bool, VisualizationError> {
        let puzzle = self
            .puzzle
            .as_ref()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let current = puzzle
            .current
            .as_ref()
            .ok_or(VisualizationError::NoPartLoaded)?;

        Ok(current.step > 0)
    }

    /// Get current step information
    pub fn current_step_info(&self) -> Result<(usize, usize), VisualizationError> {
        let puzzle = self
            .puzzle
            .as_ref()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let current = puzzle
            .current
            .as_ref()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;

        Ok((current.step, part.steps.len()))
    }

    // ============================================================================
    // PUZZLE MANAGEMENT WITH RENDERER COMPATIBILITY
    // ============================================================================

    // Ended up unimplemented. I don't think we ever really need this.
    //
    // pub fn load_registry(&mut self, registry: RendererRegistry) {
    //     self.registry = registry;
    //     //FIXME: Error here :
    //     //TODO: we need to verify that we're not rendering something, and null the current renderer
    //     //or at least check if it's not in the new registry !
    // }
    /// Load a puzzle and ensure renderer compatibility
    pub fn load_puzzle(&mut self, puzzle: AlgorithmInstance) {
        self.active_renderer = None;
        self.puzzle = Some(puzzle);
    }

    // ============================================================================
    // RENDERING - THE CRITICAL BRIDGE
    // ============================================================================

    /// Render current step and state
    pub fn render(&mut self, context: &mut dyn RenderContext) -> Result<(), VisualizationError> {
        let puzzle = self
            .puzzle
            .as_ref()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let renderer = self
            .active_renderer
            .as_mut()
            .ok_or(VisualizationError::NoRendererSelected)?;
        let state = puzzle
            .state
            .as_ref()
            .ok_or(VisualizationError::MissingState)?;
        let snapshot = state.inner.create_snapshot_erased();
        renderer.render_state_erased(snapshot.as_ref(), context)?;
        Ok(())
    }
}

impl Default for VisualizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ContextConfiguration<'a, C> {
    engine: &'a mut VisualizationEngine,
    context_type: TypeId,
    _phantom: PhantomData<C>,
}

impl<C: RenderContext + 'static> ContextConfiguration<'_, C> {
    pub fn set_renderer(
        &mut self,
        selector: impl Fn(&mut RendererSelector),
    ) -> Result<(), VisualizationError> {
        let puzzle = self
            .engine
            .puzzle
            .as_ref()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        //FIXME: Wrong error type
        let snapshot = puzzle
            .state
            .as_ref()
            .ok_or(VisualizationError::IncompatibleRenderer)?
            .info
            .snapshot_type_id;

        let renderers = self
            .engine
            .registry
            .renderer_registry()
            //FIXME : Wrong error type
            .get_renderers(snapshot, self.context_type)
            .ok_or(VisualizationError::IncompatibleRenderer)?;
        let mut selection = RendererSelector::from_renderers(renderers)
            .ok_or(VisualizationError::IncompatibleRenderer)?;

        selector(&mut selection);
        let new_renderer = selection
            .resolve_selection()
            .ok_or(VisualizationError::IncompatibleRenderer)?;
        self.engine.active_renderer = Some(new_renderer);
        Ok(())
    }

    pub fn set_state(
        &mut self,
        selector: impl Fn(&mut StateSelector),
    ) -> Result<(), VisualizationError> {
        let puzzle = self
            .engine
            .puzzle
            .as_mut()
            .ok_or(VisualizationError::NoPuzzleLoaded)?;
        let current = puzzle
            .current
            .as_mut()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let part = current
            .current_part(&puzzle.parts)
            //FIXME: These are not be the right
            // error types, but the logic is sound. Bugs would be confusing
            .ok_or(VisualizationError::NoPartLoaded)?;

        let step_type = part.step_type_id;
        let mut selection =
            StateSelector::from_step_id(step_type, self.engine.registry.state_registry())
                .ok_or(VisualizationError::IncompatibleRenderer)?;
        selector(&mut selection);
        let selected_state: StateInfo = selection
            .resolve_selection()
            .ok_or(VisualizationError::NoRendererSelected)?;
        let state = (selected_state.factory)(&part.input_data, &part.configuration)
            .ok()
            .ok_or(VisualizationError::NoRendererSelected)?; //FIXME:
        puzzle.state = Some(crate::algorithm::State {
            inner: state,
            info: selected_state,
        });
        Ok(())
    }
}
