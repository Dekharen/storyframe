// use std::str::pattern::Pattern;

use std::string::ParseError;

use crate::puzzle::PuzzleMetadata;

// pub description: Option<String>,
// pub author: Option<String>,
// pub difficulty: Option<u8>,
//
const COMMENT_SIGN: &str = "#";
//let (metadata, step_type_id, input_data) =
pub fn parse_puzzle_format(
    raw_content: &str,
) -> Result<(PuzzleMetadata, &'static str, &str), ParseError> {
    let mut metadata = PuzzleMetadata::default();
    for input_line in raw_content.lines() {
        match input_line {
            "\n" | "\r\n" => {
                continue;
            }
            mut line => {
                line = line.trim();
                if line.starts_with(COMMENT_SIGN) {
                    continue;
                };
                if let Some(description) = parse_line_var(line, "description:") {
                    if metadata.description.is_some() {
                        panic!("Duplicate input (should be error)");
                    }
                    metadata.description = Some(description.to_string());
                }
                if let Some(author) = parse_line_var(line, "author:") {
                    if metadata.author.is_some() {
                        panic!("Duplicate input (should be error)");
                    }
                    metadata.author = Some(author.to_string());
                }

                if let Some(difficulty) = parse_line_var(line, "difficulty:") {
                    if metadata.difficulty.is_some() {
                        panic!("Duplicate input (should be error)");
                    }

                    metadata.difficulty = Some(
                        difficulty
                            .parse::<u8>()
                            .expect("failed parsing for difficulty"),
                    );
                }
            }
        };
    }
    todo!()
}

fn parse_line_var<'a>(line: &'a str, tag: &'static str) -> Option<&'a str> {
    return line.strip_prefix(tag);
}
