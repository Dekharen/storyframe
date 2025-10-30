use std::{fmt, string::FromUtf8Error};

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug)]
pub enum ParseError {
    RequiredConfigField(String),
    InvalidFormat(String),
    IoError(std::io::Error),
    FromUtf8Error(FromUtf8Error),
    InvalidPartStructure(String),
    MissingPartField(String, &'static str),
    ConflictingFieldTypes {
        path: String,
        existing_type: &'static str,
        attempted_type: &'static str,
        existing_value: String,
    },
    UnknownStepType {
        step_type: String,
        supported_step_types: Vec<String>,
    },
    EmptyPath,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::RequiredConfigField(field) => {
                write!(f, "Required configuration field '{field}' is missing")
            }
            ParseError::ConflictingFieldTypes {
                path,
                existing_type,
                attempted_type,
                existing_value,
            } => {
                write!(
                    f,
                    "Field '{}' is already set as {} with value '{}', cannot use as {}",
                    path, existing_type, existing_value, attempted_type
                )
            }
            ParseError::EmptyPath => write!(f, "Cannot set value with empty path"),
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::IoError(err) => write!(f, "IO error: {}", err),
            ParseError::FromUtf8Error(err) => {
                write!(f, "Parse error, non-valid utf8 passed : {}", err)
            }
            ParseError::InvalidPartStructure(part) => {
                write!(f, "Part is not a properly structured node : {}", part)
            }
            ParseError::MissingPartField(part_name, missing_key) => {
                write!(f, "Part {part_name} is missing the key : {missing_key}")
            }
            ParseError::UnknownStepType {
                step_type,
                supported_step_types,
            } => {
                write!(
                    f,
                    "Unknown step type '{}'. Supported types: {:?}",
                    step_type, supported_step_types,
                )
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::IoError(err) => Some(err),
            ParseError::FromUtf8Error(err) => Some(err),
            _ => None,
        }
    }
}
impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> Self {
        ParseError::FromUtf8Error(err)
    }
}
impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err)
    }
}

// ============================================================================

#[derive(Debug)]
pub enum StepError {
    InvalidIndex(usize),
    IncompatibleStepType,
    InvalidPosition(usize),
}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepError::InvalidIndex(idx) => write!(f, "Invalid step index: {}", idx),
            StepError::InvalidPosition(idx) => {
                write!(f, "Invalid step position within state: {}", idx)
            }
            StepError::IncompatibleStepType => write!(f, "Incompatible step type"),
        }
    }
}

impl std::error::Error for StepError {}

// ============================================================================

#[derive(Debug)]
pub enum SolveError {
    UnknownPart(String),
    InvalidState(String),
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolveError::UnknownPart(part) => write!(f, "Unknown puzzle part: {}", part),
            SolveError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
        }
    }
}

impl std::error::Error for SolveError {}

// ============================================================================

#[derive(Debug)]
pub enum RenderError {
    IncompatibleStepType {
        expected: &'static str,
        received: &'static str,
    },
    IncompatibleContext(&'static str),
    IncompatibleState(&'static str),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::IncompatibleStepType { expected, received } => {
                write!(
                    f,
                    "Incompatible step type: expected '{}', received '{}'",
                    expected, received
                )
            }
            RenderError::IncompatibleState(msg) => {
                write!(f, "Render context was expecting this state type : {}.", msg)
            }
            RenderError::IncompatibleContext(msg) => {
                write!(
                    f,
                    "Render context was expecting this context type : {}.",
                    msg
                )
            }
        }
    }
}

impl std::error::Error for RenderError {}

// ============================================================================

#[derive(Debug)]
pub enum VisualizationError {
    NoPuzzleLoaded,
    NoPartLoaded,
    NoRendererSelected,
    IncompatibleRenderer,
    RenderError(RenderError),
    StepError(StepError),
    NoCompatibleRenderer(&'static str),
    AlreadyAtEnd,
    AlreadyAtBeginning,
    InvalidStepIndex(usize),
    MissingState,
}

impl fmt::Display for VisualizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VisualizationError::NoPartLoaded => write!(f, "No part loaded"),
            VisualizationError::NoPuzzleLoaded => write!(f, "No puzzle loaded"),
            VisualizationError::NoRendererSelected => write!(f, "No renderer selected"),
            VisualizationError::IncompatibleRenderer => {
                write!(f, "Incompatible renderer for current puzzle")
            }

            VisualizationError::StepError(err) => write!(f, "Step error: {}", err),
            VisualizationError::RenderError(err) => write!(f, "Render error: {}", err),
            VisualizationError::AlreadyAtEnd => write!(f, "Already at the end of the step array"),
            VisualizationError::AlreadyAtBeginning => {
                write!(f, "Already at the beginning of the step array")
            }
            VisualizationError::InvalidStepIndex(index) => {
                write!(f, "Invalid step index provided : {index}")
            }
            VisualizationError::NoCompatibleRenderer(step) => write!(
                f,
                "State type is implemented, but does not have any loaded renderer : {step}"
            ),
            VisualizationError::MissingState => todo!(),
        }
    }
}

impl std::error::Error for VisualizationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            VisualizationError::RenderError(err) => Some(err),
            VisualizationError::StepError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<StepError> for VisualizationError {
    fn from(err: StepError) -> Self {
        VisualizationError::StepError(err)
    }
}
impl From<RenderError> for VisualizationError {
    fn from(err: RenderError) -> Self {
        VisualizationError::RenderError(err)
    }
}

// ============================================================================

#[derive(Debug)]
pub enum PuzzleError {
    ParseError(ParseError),
    UnsupportedSource,
    IoError(std::io::Error),
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuzzleError::ParseError(err) => write!(f, "Parse error: {}", err),
            PuzzleError::UnsupportedSource => write!(f, "Unsupported puzzle source"),
            PuzzleError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for PuzzleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PuzzleError::ParseError(err) => Some(err),
            PuzzleError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ParseError> for PuzzleError {
    fn from(err: ParseError) -> Self {
        PuzzleError::ParseError(err)
    }
}

impl From<std::io::Error> for PuzzleError {
    fn from(err: std::io::Error) -> Self {
        PuzzleError::IoError(err)
    }
}
