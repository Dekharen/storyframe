use crate::{StepAction, core::split::SplitArray, error::ParseError};

pub mod state;

// TODO: Kind of a clash : I'd assume we'd use position as a f32, but also that we likely register
// it as a usize position.
// We kinda need a heatmap implementation-type that uses a f32 position for this to make more sense as a separation.

#[derive(Debug)]
pub struct SimpleF32GridStep {
    position: [usize; 2],
    content: Option<f32>,
    color: Option<String>,
}

impl StepAction for SimpleF32GridStep {
    fn type_id() -> &'static str
    where
        Self: Sized,
    {
        "simple_f32_grid_step"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn from_str(string: &str) -> Result<Self, crate::error::ParseError>
    where
        Self: Sized,
    {
        //FIXME: use some provided separators
        // let mut list = string.split("__");
        let [positions, content, color] = string
            .split_array("__")
            .ok_or(ParseError::InvalidFormat(string.to_string()))?;
        // let positions = list
        //     .next()
        //     .ok_or(ParseError::InvalidFormat(string.to_string()))?;
        //
        // let content = list
        //     .next()
        //     .ok_or(ParseError::InvalidFormat(string.to_string()))?
        //     .to_string();
        //
        // let color = list
        //     .next()
        //     .ok_or(ParseError::InvalidFormat(string.to_string()))?
        //     .to_string();
        //
        // TODO: Separators... This is a simple dash
        let [x, y] = positions
            .split_array("_")
            .ok_or(ParseError::InvalidFormat(string.to_string()))?;
        // let mut positions = positions.split("_");
        //
        // let x = positions
        //     .next()
        //     .ok_or(ParseError::InvalidFormat(string.to_string()))?;
        // let y = positions
        //     .next()
        //     .ok_or(ParseError::InvalidFormat(string.to_string()))?;
        //
        let position: [usize; 2] = [
            x.parse()
                .map_err(|_| ParseError::InvalidFormat(x.to_string()))?,
            y.parse()
                .map_err(|_| ParseError::InvalidFormat(y.to_string()))?,
        ];

        let content: Option<f32> = if content.trim().is_empty() {
            None
        } else {
            Some(
                content
                    .trim()
                    .parse()
                    .map_err(|_| ParseError::InvalidFormat(content.to_string()))?,
            )
        };
        let color = if color.trim().is_empty() {
            None
        } else {
            Some(color.trim().to_string())
        };
        Ok(Self {
            position,
            content,
            color,
        })
    }
}
//TODO : Implement text grid
//
// #[derive(Debug)]
// pub struct TextGridStep {
//     position: [usize; 2],
//     content: Option<String>,
//     background_color: Option<String>,
//     foreground_color: Option<String>,
// }
//
// impl StepAction for TextGridStep {
//     fn type_id() -> &'static str
//     where
//         Self: Sized,
//     {
//         todo!()
//     }
//
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
//
//     fn from_str(string: &str) -> Result<Self, crate::error::ParseError>
//     where
//         Self: Sized,
//     {
//         todo!()
//     }
// }
//
//
#[cfg(test)]
mod test {

    #[cfg(test)]
    #[test]
    fn test_simple_grid() {
        let content = r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: simple_f32_grid_step
        part.tokenize.input: hello world
        part.tokenize.steps: 0__goodbye__ __
    "#;
        let s = crate::engine::VisualizationEngine::from_source(
            crate::algorithm::PuzzleSource::String(content.to_string()),
        );
        match s {
            Ok(_) => todo!(),
            Err(e) => {
                dbg!(e);
                panic!();
            }
        }
    }
}
