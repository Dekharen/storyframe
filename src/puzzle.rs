use crate::{
    core::input::RawInput,
    core::state::VisualizationState,
    core::step::StepAction,
    error::{ParseError, PuzzleError, SolveError},
};
use std::path::PathBuf;

#[derive(Default)]
/// Metadata about a puzzle and its parts
pub struct PuzzleMetadata {
    pub title: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub difficulty: Option<u8>,
}

/// Information about a solvable part of a puzzle
pub struct PartInfo {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
}

/// A complete puzzle instance ready for visualization
pub struct PuzzleInstance {
    pub metadata: PuzzleMetadata,
    pub parts: Vec<PartInfo>,
    pub state: Box<dyn VisualizationState>,
    pub steps: Vec<Box<dyn StepAction>>,
    pub step_type_id: &'static str,
    //TODO: Current Step could (should ?) be an enum or a more complex type probably, if we ever
    //implement something like parallel stepping. TBD
    pub current_step: usize,
}

impl PuzzleInstance {
    /// Create a puzzle instance from a source
    fn from_source(source: PuzzleSource) -> Result<PuzzleInstance, ParseError>
    where
        Self: Sized,
    {
        let raw_content = match source {
            PuzzleSource::File(path) => std::fs::read_to_string(path)?,
            PuzzleSource::Executable(path, args) => {
                // Execute and capture output
                let output = std::process::Command::new(path).args(args).output()?;
                String::from_utf8(output.stdout)?
            }
            PuzzleSource::InlineCode(content) => content,
            PuzzleSource::Network(url) => {
                // HTTP request implementation
                todo!("Network source")
            }
            PuzzleSource::Interactive => {
                // Interactive input
                todo!("Interactive source")
            }
        };
        let metadata = PuzzleMetadata::default();
        PuzzleInstance {
            metadata,
            parts: Vec::new(),
        }
    }

    /// Shortand of `from_source()` when handling specifically file paths.
    pub fn from_file(path: PathBuf) -> Result<Self, ParseError> {
        todo!()
    }
    // pub fn from_executable(path: PathBuf, args: Vec<String>) -> Result<Self, ParseError>;
    // pub fn from_network(url: String) -> Result<Self, ParseError>;
    //
    //
    pub fn solve_part(&mut self, part_id: &str) -> Result<(), SolveError> {
        todo!()
    }
}

/// Raw input data from various sources
pub enum PuzzleSource {
    File(PathBuf),
    Executable(PathBuf, Vec<String>), // path + args
    InlineCode(String),
    Network(String),
    Interactive,
}
// ============================================================================
// PUZZLE DEFINITION
// ============================================================================

/// Creates puzzle instances from various sources
pub trait Puzzle {
    type InputProcessor: RawInput;

    /// Create a puzzle instance from a source
    fn from_source(source: PuzzleSource) -> Result<PuzzleInstance, PuzzleError>
    where
        Self: Sized;

    /// Solve a specific part and generate steps
    fn solve_part(
        state: &mut <Self::InputProcessor as RawInput>::VisualizationState,
        part_id: &str,
    ) -> Result<Vec<<Self::InputProcessor as RawInput>::StepAction>, SolveError>;
}
