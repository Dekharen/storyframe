pub mod core {
    pub mod id;
    pub mod input;
    pub mod render;
    pub mod state;
    pub mod step;
}
pub mod domains;
pub mod engine;
pub mod error;
pub mod macros;
pub mod puzzle;
pub use core::{
    render::{
        context::{HasContextTag, RenderContext},
        Renderer,
    },
    step::StepAction,
};
pub use engine::registry::Registry;
