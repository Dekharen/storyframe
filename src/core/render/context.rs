use std::any::Any;

// ============================================================================
// RENDERING CONTEXT
// ============================================================================

/// Abstraction over different UI frameworks (egui, console, etc.)
pub trait RenderContext {
    /// Framework-specific rendering area
    fn render_area(&mut self) -> &mut dyn Any;

    /// Request a repaint/refresh
    fn request_repaint(&self);
}
