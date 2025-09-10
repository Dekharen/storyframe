use storyframe::{
    error::ParseError,
    puzzle::{PuzzleInstance, PuzzleSource},
}; // Import your library

#[test]
fn test_basic_puzzle_parsing() {
    let content = r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        part.tokenize.input: hello world
        part.tokenize.steps: highlight_char 0
    "#;

    let result = PuzzleInstance::from_source(PuzzleSource::String(content.to_string()));
    assert!(result.is_ok());
}

#[test]
fn test_invalid_step_type() {
    let content = r#"
        title: Bad Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.input: hello world
        part.tokenize.steps: highlight_char 0
        part.tokenize.step_type: unknown_type
    "#;

    let result = PuzzleInstance::from_source(PuzzleSource::String(content.to_string()));
    println!("{result:?}");
    assert!(matches!(result, Err(ParseError::UnknownStepType(_))));
}
#[test]
fn test_missing_puzzle_fields() {
    let content_list = vec![
        r#"
        title: Test Puzzle
        #part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        part.tokenize.input: hello world
        part.tokenize.steps: highlight_char 0
    "#,
        r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        #part.tokenize.step_type: text_step
        part.tokenize.input: hello world
        part.tokenize.steps: highlight_char 0
    "#,
        r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        #part.tokenize.input: hello world
        part.tokenize.steps: highlight_char 0
    "#,
        r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        part.tokenize.input: hello world
        #part.tokenize.steps: highlight_char 0
    "#,
    ];
    for content in content_list {
        test_missing_field(content);
    }
}
fn test_missing_field(content: &str) {
    let result = PuzzleInstance::from_source(PuzzleSource::String(content.to_string()));
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
        #part.tokenize.input: hello world
        #part.tokenize.steps: highlight_char 0
    "#,
    ];
    for content in content_list {
        test_invalid_part(content);
    }
}

fn test_invalid_part(content: &str) {
    let result = PuzzleInstance::from_source(PuzzleSource::String(content.to_string()));
    println!("{result:?}");
    // TODO: we could also test for correct fmt
    assert!(matches!(result, Err(ParseError::InvalidPartStructure(_))));
}
