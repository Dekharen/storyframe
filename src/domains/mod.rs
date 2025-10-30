use crate::{
    core::{input::processors::StepParserFn, state::VisualizationState, step::StepAction},
    error::ParseError,
};

pub mod grids;
pub mod text;

#[derive(Debug, Clone)]
pub struct DomainRegistry {
    step_type_to_id: fn(&str) -> Result<&'static str, ParseError>,
    get_parser: fn(&str) -> Option<StepParserFn>,
    pub step_types: Vec<String>,
}

impl DomainRegistry {
    pub fn new(
        step_types: Vec<String>,
        get_parser: fn(&str) -> Option<StepParserFn>,
        step_type_to_id: fn(&str) -> Result<&'static str, ParseError>,
    ) -> Self {
        Self {
            step_types,
            get_parser,
            step_type_to_id,
        }
    }
    pub fn get_supported_types(&self) -> Vec<String> {
        self.step_types.clone()
    }
    pub fn step_type_to_id(&self, step_type: &str) -> Result<&'static str, ParseError> {
        (self.step_type_to_id)(step_type)
    }
    pub fn parse_steps(
        &self,
        step_type_id: &str,
        steps_str: &str,
    ) -> Result<Vec<Box<dyn StepAction>>, ParseError> {
        let parser =
            (self.get_parser)(step_type_id).ok_or_else(|| ParseError::UnknownStepType {
                step_type: step_type_id.to_string(),
                supported_step_types: self.step_types.clone(),
            })?;

        parse_steps_with_parser(parser, steps_str)
    }
}

fn parse_steps_with_parser(
    parser: StepParserFn,
    steps_str: &str,
) -> Result<Vec<Box<dyn StepAction>>, ParseError> {
    let step_strings: Vec<&str> = steps_str.split('|').map(|s| s.trim()).collect();
    let mut steps = Vec::new();

    for step_str in step_strings {
        let step = parser(step_str)?;
        steps.push(step);
    }

    Ok(steps)
}

crate::register_domain_types!(
    text::TextStep {
        aliases: ["text", "text_step"],
        states: [text::state::TextState] // default_state: TextState
    },
    grids::simple_grid::SimpleF32GridStep {
        aliases: ["grid", "matrix"],
        states: [grids::simple_grid::state::SimpleGridState {
            required: ["columns", "rows"]
        }]
    } // GraphStep { aliases: ["graph", "tree"] },
      // GridStep { aliases: ["grid", "matrix"] }
);
