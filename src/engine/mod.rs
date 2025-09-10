// ============================================================================
// VISUALIZATION ENGINE
// ============================================================================

pub mod registry;
use crate::{
    core::render::{context::RenderContext, RendererProxy},
    error::VisualizationError,
    puzzle::PuzzleInstance,
};
use registry::RendererRegistry;

/// Manages renderers and coordinates visualization
pub struct VisualizationEngine {
    puzzle: Option<PuzzleInstance>,
    active_renderer: Option<Box<dyn RendererProxy>>,
}

impl VisualizationEngine {
    pub fn new() -> Self {
        Self {
            puzzle: None,
            active_renderer: None,
        }
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
        if current.step < part.steps.len() {
            let step = &part.steps[current.step];
            current.state.apply_step(step.as_ref())?;
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
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        if current.step > 0 {
            let target_step = current.step - 1;
            current.state.seek_to_step(target_step, &part.steps)?;
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
        let current = puzzle
            .current
            .as_mut()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        if step_index <= part.steps.len() {
            current.state.seek_to_step(step_index, &part.steps)?;
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
        current.state.reset_to_initial();
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

    /// Get description of current step (if any)
    pub fn current_step_description(&self) -> Result<Option<String>, VisualizationError> {
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

        if current.step < part.steps.len() {
            let step = &part.steps[current.step];
            Ok(Some(step.description()))
        } else {
            Ok(None)
        }
    }

    // ============================================================================
    // PUZZLE MANAGEMENT WITH RENDERER COMPATIBILITY
    // ============================================================================

    /// Load a puzzle and ensure renderer compatibility
    pub fn load_puzzle(
        &mut self,
        puzzle: PuzzleInstance,
        _registry: &RendererRegistry, //TODO: fix this. The logic has changed ! This should be
                                      //handled somewhere else
                                      //
    ) -> Result<(), VisualizationError> {
        // Check if current renderer is compatible
        // if let Some(current_renderer) = &self.active_renderer {
        //     if current_renderer.step_type_id() != puzzle.step_type_id {
        //         // Auto-switch to compatible renderer
        //         if let Some(new_renderer) = registry.get_renderer_for_step_type(puzzle.step_type_id)
        //         {
        //             self.active_renderer = Some(new_renderer);
        //         } else {
        //             return Err(VisualizationError::NoCompatibleRenderer(
        //                 puzzle.step_type_id,
        //             ));
        //         }
        //     }
        // } else {
        //     // No renderer selected, try to auto-select
        //     if let Some(renderer) = registry.get_renderer_for_step_type(puzzle.step_type_id) {
        //         self.active_renderer = Some(renderer);
        //     } else {
        //         return Err(VisualizationError::NoCompatibleRenderer(
        //             puzzle.step_type_id,
        //         ));
        //     }
        // }
        //
        self.active_renderer = None;
        self.puzzle = Some(puzzle);
        Ok(())
    }

    /// Switch renderer (assumes registry provided compatible renderer)
    pub fn set_renderer(
        &mut self,
        renderer: Box<dyn RendererProxy>,
    ) -> Result<(), VisualizationError> {
        if let Some(puzzle) = &self.puzzle {
            let step_type = puzzle
                .current_part()
                .ok_or(VisualizationError::NoPartLoaded)?
                // .expect("A renderer was set without a part selected. This is an engine failure.")
                .step_type_id;
            assert_eq!(
                renderer.step_type_id(),
                step_type,
                "Registry provided incompatible renderer: expected {}, got {}",
                step_type,
                renderer.step_type_id()
            );
            self.active_renderer = Some(renderer);
            Ok(())
        } else {
            panic!("Cannot set renderer without loaded puzzle - this indicates a bug in the application logic");
        }
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
        let current = puzzle
            .current
            .as_ref()
            .ok_or(VisualizationError::NoPartLoaded)?;
        let part = current
            .current_part(&puzzle.parts)
            .ok_or(VisualizationError::NoPartLoaded)?;
        let snapshot = current.state.create_snapshot();

        if current.step < part.steps.len() {
            let current_step = &part.steps[current.step];
            renderer.render_step_erased(current_step.as_ref(), snapshot.as_ref(), context);
        } else {
            renderer.render_state_erased(snapshot.as_ref(), context);
        }

        Ok(())
    }
}

impl Default for VisualizationEngine {
    fn default() -> Self {
        Self::new()
    }
}
