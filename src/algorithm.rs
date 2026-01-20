use crate::{
    core::{
        configuration::Configuration,
        input::{processors::parse_puzzle_format, read_source_content},
        state::{StateInfo, StateProxy},
        step::StepAction,
    },
    domains::DomainRegistry,
    error::{ParseError, SolveError},
};
use std::path::PathBuf;

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// Metadata about a puzzle and its parts
pub struct Metadata {
    pub title: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub difficulty: Option<u8>,
    // pub configuration: Configuration,
}

#[derive(Debug)]
/// Information about a solvable part of a puzzle
pub struct RawPartMetadata {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub raw_steps_string: String,
    pub configuration: Configuration,
    // this is maybe the wrong type, as in it should already be
    // transformed into a step-related state but we'll see. Like, maybe a Box<dyn VisualizationState>
    // that is the initial state
    pub input_data: String,
    pub raw_step_type_id: String,
}
pub fn parse_part_info(
    part: RawPartMetadata,
    registry: &DomainRegistry,
) -> Result<PartInfo, ParseError> {
    let step_type_id: &'static str = registry.step_type_to_id(&part.raw_step_type_id)?;
    Ok(PartInfo {
        step_type_id,
        steps: registry.parse_steps(step_type_id, &part.raw_steps_string)?,
        id: part.id,
        display_name: part.display_name,
        description: part.description,
        input_data: part.input_data,
        configuration: part.configuration,
    })
}

#[derive(Debug)]
/// Information about a solvable part of a puzzle
pub struct PartInfo {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub configuration: Configuration,
    pub(crate) steps: Vec<Box<dyn StepAction>>,
    // this is maybe the wrong type, but for different reasons than before, because we might need
    // some type markings to make sure a single string type can transfer to multiple types of
    // states. Oh well
    pub(crate) input_data: String,
    pub step_type_id: &'static str,
}
#[derive(Debug)]
/// The current, active state of the puzzle. This is decided by which part is selected.
pub struct Current {
    //TODO: Current Step could (should ?) be an enum or a more complex type probably, if we ever
    //implement something like parallel stepping. TBD
    pub step: usize,
    pub part_id: String,
}
#[derive(Debug)]
pub(crate) struct State {
    pub inner: Box<dyn StateProxy>,
    pub info: StateInfo,
}
impl State {
    pub(crate) fn reset(
        &mut self,
        raw_state_input: &str,
        configuration: &Configuration,
    ) -> Result<(), ParseError> {
        self.inner = (self.info.factory)(raw_state_input, configuration)?;
        Ok(())
    }
}

impl<'a> Current {
    pub fn current_part(&self, parts: &'a [PartInfo]) -> Option<&'a PartInfo> {
        let current_part_id = &self.part_id;
        parts.iter().find(|p| &p.id == current_part_id)
    }
}

#[derive(Debug)]
/// A complete puzzle instance ready for visualization
pub struct AlgorithmInstance {
    pub(crate) metadata: Metadata,
    pub(crate) parts: Vec<PartInfo>,
    pub(crate) current: Option<Current>,
    pub(crate) state: Option<State>,
    // pub state: Option<Box<dyn VisualizationState>>,
    // // pub steps: Vec<Box<dyn StepAction>>,
    // // pub step_type_id: &'static str,
    // pub current_step: usize,
    // pub current_part_id: String,
}

impl AlgorithmInstance {
    /// Create a puzzle instance from a source
    pub fn from_source(
        source: PuzzleSource,
        registry: &DomainRegistry,
    ) -> Result<AlgorithmInstance, ParseError>
    where
        Self: Sized,
    {
        let raw_content = read_source_content(source)?;
        let (metadata, parts) = parse_puzzle_format(&raw_content)?;
        let mut parsed_parts = Vec::with_capacity(parts.len());
        for part in parts.into_iter() {
            parsed_parts.push(parse_part_info(part, registry)?);
        }
        // let metadata = PuzzleMetadata::default();
        Ok(AlgorithmInstance {
            metadata,
            // state: None,
            current: None,
            parts: parsed_parts,
            state: None,
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

#[cfg(test)]
mod test {
    use crate::domains::create_registry;

    use super::*;

    #[test]
    fn test_basic_puzzle_parsing() {
        let content = r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        part.tokenize.input: hello world
        part.tokenize.steps: 0__goodbye__ __
    "#;

        let result = AlgorithmInstance::from_source(
            PuzzleSource::String(content.to_string()),
            create_registry().domain_registry(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_puzzle_invalid_step_type() {
        let content = r#"
        title: Bad Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.input: hello world
        part.tokenize.steps: 0__goodbye__ __
        part.tokenize.step_type: unknown_type
    "#;

        let result = AlgorithmInstance::from_source(
            PuzzleSource::String(content.to_string()),
            create_registry().domain_registry(),
        );
        println!("{result:?}");
        assert!(matches!(result, Err(ParseError::UnknownStepType { .. })));
    }
    #[test]
    fn test_missing_puzzle_fields() {
        let content_list = vec![
            r#"
        title: Test Puzzle
        #part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        part.tokenize.input: hello world
        part.tokenize.steps: 0__goodbye__ __
    "#,
            r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        #part.tokenize.step_type: text_step
        part.tokenize.input: hello world
        part.tokenize.steps: 0__goodbye__ __
    "#,
            r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        #part.tokenize.input: hello world
        part.tokenize.steps: 0__goodbye__ __
    "#,
            r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        part.tokenize.input: hello__world
        #part.tokenize.steps: 0__goodbye__ __
    "#,
        ];
        for content in content_list {
            test_missing_field(content);
        }
    }
    // #[cfg(test)]
    fn test_missing_field(content: &str) {
        let result = AlgorithmInstance::from_source(
            PuzzleSource::String(content.to_string()),
            create_registry().domain_registry(),
        );
        println!("{result:?}");
        // TODO: we could also test for correct fmt
        assert!(matches!(result, Err(ParseError::MissingPartField(_, _))));
    }

    #[test]
    fn test_invalid_part_structure() {
        let content_list = vec![
            r#"
        title: Test Puzzle
        part.tokenize: Tokenization
        #part.tokenize.step_type: text_step
        #part.tokenize.input: hello__world
        #part.tokenize.steps: 0__goodbye__ __
    "#,
        ];
        for content in content_list {
            test_invalid_part(content);
        }
    }

    #[cfg(test)]
    fn test_invalid_part(content: &str) {
        let result = AlgorithmInstance::from_source(
            PuzzleSource::String(content.to_string()),
            create_registry().domain_registry(),
        );
        println!("{result:?}");
        // TODO: we could also test for correct fmt
        assert!(matches!(result, Err(ParseError::InvalidPartStructure(_))));
    }
}
