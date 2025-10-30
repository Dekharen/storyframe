// use std::str::pattern::Pattern;

use std::{collections::HashMap, sync::Arc};

use crate::{
    algorithm::{Metadata, RawPartMetadata},
    // domains::step_type_to_id,
    core::{configuration::Configuration, step::StepAction},
    error::ParseError,
};

pub type StepParserFn = fn(&str) -> Result<Box<dyn StepAction>, ParseError>;
#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    Leaf(String),                 // Terminal value
    Node(HashMap<String, Field>), // Nested
}

impl Field {
    /// Parse a path like part.tokenize.name under the form "[part, tokenize, name]"
    pub fn get_path(&self, path: &[&str]) -> Option<&Field> {
        match (self, path) {
            (Field::Leaf(_), []) => Some(self),
            (Field::Leaf(_), _) => None, // Can't traverse into leaf
            (Field::Node(_), []) => Some(self),
            (Field::Node(map), [head, tail @ ..]) => map.get(*head)?.get_path(tail),
        }
    }

    /// Get leaf value if this is a leaf
    pub fn as_leaf(&self) -> Option<&str> {
        match self {
            Field::Leaf(value) => Some(value),
            Field::Node(_) => None,
        }
    }

    /// Set a value at a dotted path
    pub fn set_path(&mut self, path: &[&str], value: String) {
        match (self, path) {
            (_, []) => (), // Invalid: empty path
            (field @ Field::Leaf(_), [_single]) => {
                *field = Field::Leaf(value);
            }
            (Field::Node(map), [single]) => {
                map.insert(single.to_string(), Field::Leaf(value));
            }
            (field @ Field::Leaf(_), path) => {
                // Convert leaf to node to accommodate nested path
                *field = Field::Node(HashMap::new());
                field.set_path(path, value);
            }
            (Field::Node(map), [head, tail @ ..]) => {
                let entry = map
                    .entry(head.to_string())
                    .or_insert_with(|| Field::Node(HashMap::new()));
                entry.set_path(tail, value);
            }
        }
    }
}
const COMMENT_SIGN: char = '#';
/// Entry point for the parser, once transformed into string.
pub fn parse_puzzle_format(content: &str) -> Result<(Metadata, Vec<RawPartMetadata>), ParseError> {
    let mut root = Field::Node(HashMap::new());

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(COMMENT_SIGN) {
            continue;
        }

        if let Some((key_path, value)) = line.split_once(':') {
            let path: Vec<&str> = key_path.split('.').map(|s| s.trim()).collect();
            root.set_path(&path, value.trim().to_string());
        }
    }

    // Extract structured data
    let metadata = extract_metadata(&root)?;
    let configuration = Arc::new(extract_config(&root, None));
    let parts = extract_parts_with_steps(&root, &configuration)?;
    // TODO: store the config, maybe on the metadata, or add it as return
    Ok((metadata, parts))
}

fn extract_metadata(root: &Field) -> Result<Metadata, ParseError> {
    let mut metadata = Metadata::default();

    if let Some(title) = root.get_path(&["title"]).and_then(|f| f.as_leaf()) {
        metadata.title = title.to_string();
    }

    if let Some(author) = root.get_path(&["author"]).and_then(|f| f.as_leaf()) {
        metadata.author = Some(author.to_string());
    }

    if let Some(difficulty_str) = root.get_path(&["difficulty"]).and_then(|f| f.as_leaf()) {
        metadata.difficulty = difficulty_str.parse().ok();
    }

    Ok(metadata)
}
fn extract_parts_with_steps(
    root: &Field,
    parent_config: &Arc<Configuration>,
) -> Result<Vec<RawPartMetadata>, ParseError> {
    let mut parts = Vec::new();

    if let Some(Field::Node(part_map)) = root.get_path(&["part"]) {
        for (part_id, part_field) in part_map {
            let part_metadata =
                extract_single_part_with_steps(part_id, part_field, &parent_config)?;

            parts.push(part_metadata);
        }
    }

    Ok(parts)
}

fn extract_config(root: &Field, parent: Option<Arc<Configuration>>) -> Configuration {
    if let Some(Field::Node(configuration)) = root.get_path(&["config"]) {
        return Configuration::with_parent(configuration.clone(), parent);
    }
    Configuration::with_parent(HashMap::new(), parent)
}

fn extract_single_part_with_steps(
    part_id: &str,
    part_field: &Field,
    parent_config: &Arc<Configuration>,
) -> Result<RawPartMetadata, ParseError> {
    let Field::Node(fields) = part_field else {
        return Err(ParseError::InvalidPartStructure(part_id.to_string()));
    };

    let name = fields
        .get("name")
        .and_then(|f| f.as_leaf())
        .ok_or_else(|| ParseError::MissingPartField(part_id.to_string(), "name"))?;

    let step_type = fields
        .get("step_type")
        .and_then(|f| f.as_leaf())
        .ok_or_else(|| ParseError::MissingPartField(part_id.to_string(), "step_type"))?;

    let input_data = fields
        .get("input")
        .and_then(|f| f.as_leaf())
        .ok_or_else(|| ParseError::MissingPartField(part_id.to_string(), "input"))?
        .to_string();

    let steps_str = fields
        .get("steps")
        .and_then(|f| f.as_leaf())
        .ok_or_else(|| ParseError::MissingPartField(part_id.to_string(), "steps"))?;
    let configuration = extract_config(part_field, Some(parent_config.clone()));

    // Create PartInfo without steps (they'll be parsed later with proper input context)
    let part_metadata = RawPartMetadata {
        id: part_id.to_string(),
        display_name: name.to_string(),
        configuration,
        description: fields
            .get("description")
            .and_then(|f| f.as_leaf())
            .map(String::from),
        input_data,                              // Store in PartInfo too for easy access
        raw_steps_string: steps_str.to_string(), // Will be populated later
        raw_step_type_id: step_type.to_string(),
    };

    Ok(part_metadata)
}
