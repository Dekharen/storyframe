use std::collections::HashMap;

use crate::error::ParseError;

lazy_static::lazy_static! {
    static ref STEP_TYPE_REGISTRY: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("text_step", "text_step");
        m.insert("text", "text_step");           // Alias
        m.insert("string", "text_step");         // Alias

        m.insert("graph_step", "graph_step");
        m.insert("graph", "graph_step");         // Alias
        m.insert("tree", "graph_step");          // Alias

        m.insert("grid_step", "grid_step");
        m.insert("grid", "grid_step");           // Alias
        m.insert("matrix", "grid_step");         // Alias

        m
    };
}

pub fn step_type_to_id(step_type: &str) -> Result<&'static str, ParseError> {
    STEP_TYPE_REGISTRY
        .get(step_type)
        .copied()
        .ok_or_else(|| ParseError::UnknownStepType(step_type.to_string()))
}

pub fn supported_step_types() -> Vec<&'static str> {
    STEP_TYPE_REGISTRY.keys().copied().collect()
}
