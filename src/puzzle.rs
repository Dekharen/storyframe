use crate::{
    core::input::processors::parse_puzzle_format,
    core::input::RawInput,
    core::state::VisualizationState,
    core::step::StepAction,
    error::{ParseError, PuzzleError, SolveError},
};
use std::path::PathBuf;

#[derive(Default, Debug)]
/// Metadata about a puzzle and its parts
pub struct PuzzleMetadata {
    pub title: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub difficulty: Option<u8>,
}

#[derive(Debug)]
/// Information about a solvable part of a puzzle
pub struct PartInfo {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub steps: Vec<Box<dyn StepAction>>,
    // this is maybe the wrong type, as in it should already be
    // transformed into a step-related state but we'll see. Like, maybe a Box<dyn VisualizationState>
    // that is the initial state
    pub input_data: String,
    pub step_type_id: &'static str,
}
#[derive(Debug)]
/// The current, active state of the puzzle. This is decided by which part is selected.
pub struct Current {
    //TODO: Current Step could (should ?) be an enum or a more complex type probably, if we ever
    //implement something like parallel stepping. TBD
    pub step: usize,
    pub part_id: String,
    pub state: Box<dyn VisualizationState>,
}

impl<'a> Current {
    pub fn current_part(&self, parts: &'a [PartInfo]) -> Option<&'a PartInfo> {
        let current_part_id = &self.part_id;
        parts.iter().find(|p| &p.id == current_part_id)
    }
}

#[derive(Debug)]
/// A complete puzzle instance ready for visualization
pub struct PuzzleInstance {
    pub metadata: PuzzleMetadata,
    pub parts: Vec<PartInfo>,
    pub current: Option<Current>,
    // pub state: Option<Box<dyn VisualizationState>>,
    // // pub steps: Vec<Box<dyn StepAction>>,
    // // pub step_type_id: &'static str,
    // pub current_step: usize,
    // pub current_part_id: String,
}

impl PuzzleInstance {
    /// Create a puzzle instance from a source
    pub fn from_source(source: PuzzleSource) -> Result<PuzzleInstance, ParseError>
    where
        Self: Sized,
    {
        let raw_content = match source {
            PuzzleSource::String(str) => str,
            PuzzleSource::File(path) => std::fs::read_to_string(path)?,
            PuzzleSource::Executable(path, args) => {
                // Execute and capture output
                let output = std::process::Command::new(path).args(args).output()?;
                String::from_utf8(output.stdout)?
            }
            PuzzleSource::InlineCode(content) => content,
            PuzzleSource::Network(_url) => {
                // HTTP request implementation
                todo!("Network source")
            }
            PuzzleSource::Interactive => {
                // Interactive input
                todo!("Interactive source")
            }
        };
        let (metadata, parts) = parse_puzzle_format(&raw_content)?;

        // let metadata = PuzzleMetadata::default();
        Ok(PuzzleInstance {
            metadata,
            // state: None,
            current: None,
            parts: parts.into_iter().map(|t| t.0).collect(),
            // parts: Vec::new(),
            // steps: Vec::new(),
        })
    }
    /// Get currently active part
    pub fn current_part(&self) -> Option<&PartInfo> {
        let current_part_id = self.current.as_ref().map(|current| &current.part_id)?;
        self.parts.iter().find(|p| &p.id == current_part_id)
    }

    pub fn get_part(&self, part_id: &str) -> Option<&PartInfo> {
        self.parts.iter().find(|p| p.id == part_id)
    }

    /// Shortand of `from_source()` when handling specifically file paths.
    pub fn from_file(_path: PathBuf) -> Result<Self, ParseError> {
        todo!()
    }
    // pub fn from_executable(path: PathBuf, args: Vec<String>) -> Result<Self, ParseError>;
    // pub fn from_network(url: String) -> Result<Self, ParseError>;
    //
    //
    pub fn solve_part(&mut self, _part_id: &str) -> Result<(), SolveError> {
        todo!()
    }
}
/// Raw input data from various sources
pub enum PuzzleSource {
    File(PathBuf),
    String(String),
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
