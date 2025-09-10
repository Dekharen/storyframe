use crate::core::state::VisualizationState;
use crate::core::step::StepAction;
use crate::error::ParseError;

pub mod processors;
// ============================================================================
// INPUT PROCESSING
// ============================================================================

/// Processes raw input data into visualization-ready structures
pub trait RawInput {
    type VisualizationState: VisualizationState;
    type StepAction: StepAction;

    /// Parse raw input text into a visualization state
    fn parse(raw_data: &str) -> Result<Self::VisualizationState, ParseError>;

    /// Get the step type identifier for renderer compatibility
    fn step_type_id() -> &'static str
    where
        Self: Sized;
}

// ============================================================================
// EXAMPLE IMPLEMENTATIONS
// ============================================================================
//
// // Example step type
// #[derive(Clone)]
// pub struct TextStep {
//     pub target_index: usize,
//     pub modification: String, // Simplified
// }
//
// impl StepAction for TextStep {
//     fn description(&self) -> String {
//         format!("Modify character at index {}", self.target_index)
//     }
//
//     fn step_type_id() -> &'static str
//     where
//         Self: Sized,
//     {
//         "text_step"
//     }
//
//     fn get_type_id(&self) -> &'static str {
//         Self::step_type_id()
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
//
// // Example input processor
// pub struct TextInput;
// impl RawInput for TextInput {
//     type VisualizationState = TextState;
//     type StepAction = TextStep;
//
//     fn parse(raw_data: &str) -> Result<TextState, ParseError> {
//         Ok(TextState {
//             content: raw_data.to_string(),
//         })
//     }
//
//     fn step_type_id() -> &'static str {
//         "text_step"
//     }
// }
//
// // Example state
// pub struct TextState {
//     pub content: String,
// }
//
// impl VisualizationState for TextState {
//     fn apply_step(&mut self, step: &dyn StepAction) -> Result<(), StepError> {
//         if let Some(text_step) = step.as_any().downcast_ref::<TextStep>() {
//             // Apply the step
//             self.current_step += 1;
//             Ok(())
//         } else {
//             panic!("Incompatible step type provided to TextState: expected TextStep");
//         }
//     }
//
//     fn reset_to_initial(&mut self) {
//         // self.current_step = 0;
//     }
//
//     fn seek_to_step(
//         &mut self,
//         step_index: usize,
//         all_steps: &[Box<dyn StepAction>],
//     ) -> Result<(), StepError> {
//         // Reset and replay steps
//         self.reset_to_initial();
//         for step in &all_steps[..step_index] {
//             self.apply_step(step.as_ref())?;
//         }
//         Ok(())
//     }
//
//     fn current_step_index(&self) -> usize {
//         self.current_step
//     }
//
//     fn create_snapshot(&self) -> Box<dyn StateSnapshot> {
//         Box::new(TextStateSnapshot {
//             content: self.content.clone(),
//         })
//     }
// }
//
// // Example render snapshot
// pub struct TextStateSnapshot {
//     pub content: String,
// }
//
// impl StateSnapshot for TextStateSnapshot {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
//
// // Example renderer
// #[derive(Clone)]
// pub struct TextRenderer;
//
// impl Renderer for TextRenderer {
//     type Step = TextStep;
//     fn render_step(
//         &mut self,
//         step: &TextStep,
//         snapshot: &dyn StateSnapshot,
//         context: &mut dyn RenderContext,
//     ) {
//         // Render text with step highlight
//     }
//
//     fn render_state(&mut self, snapshot: &dyn StateSnapshot, context: &mut dyn RenderContext) {
//         // Render text without step highlight
//     }
//
//     fn renderer_name(&self) -> &'static str {
//         "Text Renderer"
//     }
//
//     fn supports_state_type(&self, state_type: &str) -> bool {
//         state_type == "text"
//     }
// }
//
// // Example puzzle
// pub struct GenericPuzzle;
//
// impl Puzzle for GenericPuzzle {
//     type InputProcessor = TextInput;
//
//     fn from_source(source: PuzzleSource) -> Result<PuzzleInstance, PuzzleError> {
//         let raw_data = match source {
//             PuzzleSource::File(path) => std::fs::read_to_string(path)?,
//             PuzzleSource::InlineCode(code) => code,
//             // ... other sources
//             _ => return Err(PuzzleError::UnsupportedSource),
//         };
//
//         // Parse metadata and input from the raw data
//         // Format could be: metadata_json\n---\ninput_type\n---\nactual_input\n---\nstep_type\n---\nsteps
//         let state = TextInput::parse(&raw_data)?;
//
//         Ok(PuzzleInstance {
//             metadata: PuzzleMetadata {
//                 title: "Generic Puzzle".to_string(),
//                 description: None,
//                 author: None,
//                 difficulty: None,
//             },
//             parts: vec![PartInfo {
//                 id: "part1".to_string(),
//                 display_name: "Part 1".to_string(),
//                 description: None,
//             }],
//             state: Box::new(state),
//             steps: Vec::new(),
//             step_type_id: TextInput::step_type_id(),
//             current_step: 0,
//         })
//     }
//
//     fn solve_part(state: &mut TextState, part_id: &str) -> Result<Vec<TextStep>, SolveError> {
//         match part_id {
//             "part1" => {
//                 // Generate steps for this part
//                 let steps = vec![TextStep {
//                     target_index: 0,
//                     modification: "highlight".to_string(),
//                 }];
//                 Ok(steps)
//             }
//             _ => Err(SolveError::UnknownPart(part_id.to_string())),
//         }
//     }
// }
