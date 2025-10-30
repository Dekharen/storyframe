pub mod core {
    pub mod configuration;
    pub mod id;
    pub mod input;
    pub mod render;
    pub mod split;
    pub mod state;
    pub mod step;
}
pub mod algorithm;
pub mod domains;
pub mod engine;
pub mod error;
pub mod macros;
pub use core::{
    render::{
        Renderer,
        context::{HasContextTag, RenderContext},
    },
    step::StepAction,
};
pub use engine::registry::Registry;
