pub mod state;

use crate::{core::step::StepAction, error::ParseError};
#[derive(Debug)]
pub struct TextStep {
    position: usize,
    content: Option<String>,
    background_color: Option<String>,
    foreground_color: Option<String>,
    // effect : italic/bold ?
}

impl StepAction for TextStep {
    fn type_id() -> &'static str
    where
        Self: Sized,
    {
        "text_step"
    }

    // fn get_type_id(&self) -> &'static str {
    //     Self::type_id()
    // }
    //
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn from_str(string: &str) -> Result<Self, crate::error::ParseError>
    where
        Self: Sized,
    {
        //FIXME: use some provided separators
        let mut list = string.split("__");
        let position = list
            .next()
            .ok_or(ParseError::InvalidFormat(string.to_string()))?;
        let content = list
            .next()
            .ok_or(ParseError::InvalidFormat(string.to_string()))?
            .to_string();
        let bg = list
            .next()
            .ok_or(ParseError::InvalidFormat(string.to_string()))?
            .to_string();
        let fg = list
            .next()
            .ok_or(ParseError::InvalidFormat(string.to_string()))?
            .to_string();
        let position: usize = position
            .parse()
            .map_err(|_| ParseError::InvalidFormat(position.to_string()))?;
        let content = if content.trim().is_empty() {
            None
        } else {
            Some(content.trim().to_string())
        };
        let bg = if bg.trim().is_empty() {
            None
        } else {
            Some(bg.trim().to_string())
        };
        let fg = if fg.trim().is_empty() {
            None
        } else {
            Some(fg.trim().to_string())
        };
        Ok(Self {
            position,
            content,
            background_color: bg,
            foreground_color: fg,
        })
    }
}
