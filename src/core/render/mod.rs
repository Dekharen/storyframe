use std::any::TypeId;

use super::id::{RendererId, StateId};
use crate::{
    core::state::snapshot::StateSnapshot, error::RenderError, HasContextTag, RenderContext,
};

pub mod context;
// ============================================================================
// RENDERING SYSTEM
// ============================================================================

/// Generic renderer for a specific step type
pub trait Renderer: Send + Clone + 'static {
    type StateSnapshot: StateSnapshot;
    type Context<'a>: RenderContext + HasContextTag
    where
        Self: 'a;

    /// Render a step application with the current state
    fn render_state(&mut self, snapshot: &Self::StateSnapshot, context: &mut Self::Context<'_>);

    /// Get human-readable name for UI selection
    fn renderer_name(&self) -> RendererId;
}
/// Type-erased wrapper for storing different renderer types
pub trait RendererProxy: Send + Sync {
    fn state_type_id(&self) -> StateId;
    fn renderer_name(&self) -> RendererId;
    // fn supports_state_type(&self, state_type: &str) -> bool;

    fn render_state_erased(
        &mut self,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    ) -> Result<(), RenderError>;

    fn clone_boxed(&self) -> Box<dyn RendererProxy>;
}

/// Blanket implementation to convert any Renderer of T into RendererProxy
impl<R: Renderer + Sync> RendererProxy for R {
    fn state_type_id(&self) -> StateId {
        R::StateSnapshot::snapshot_type_id()
    }

    fn renderer_name(&self) -> RendererId {
        self.renderer_name()
    }

    // fn supports_state_type(&self, state_type: &str) -> bool {
    //     self.supports_state_type(state_type)
    // }

    fn render_state_erased(
        &mut self,
        snapshot: &dyn StateSnapshot,
        context: &mut dyn RenderContext,
    ) -> Result<(), RenderError> {
        // render_state_erased(self, snapshot, context)
        let typed_snapshot = snapshot
            .as_any()
            .downcast_ref::<R::StateSnapshot>()
            .ok_or_else(|| RenderError::IncompatibleState(R::StateSnapshot::snapshot_type_id()))?;
        //FIXME: get a debug repr of context type tag
        let expected = TypeId::of::<<R::Context<'_> as HasContextTag>::Tag>();
        if context.tag_id() != expected {
            return Err(RenderError::IncompatibleContext("context"));
        }
        let ptr = context.as_ptr() as *mut R::Context<'_>;
        let typed_context: &mut R::Context<'_> = unsafe { &mut *ptr };
        // let typed_context = context
        // .as_any_mut()
        // .downcast_mut::<R::Context>()
        // .ok_or_else(|| RenderError::IncompatibleContext(R::Context::context_type_id()))?;
        self.render_state(typed_snapshot, typed_context);
        Ok(())
    }
    fn clone_boxed(&self) -> Box<dyn RendererProxy> {
        Box::new(self.clone())
    }
}
// he
// // //
// /// Implements the [`RenderContext`] trait, and links a context type
// /// with a unique `'static` tag type used for safe downcasting.
// ///
// /// # Example
// /// ```
// /// use mycrate::{RenderContext, impl_render_context};
// ///
// /// pub struct MyUi;
// ///
// /// pub struct MyContext<'a> {
// ///     pub ui: &'a mut MyUi,
// /// }
// ///
// /// // This generates:
// /// // - a zero-sized type `MyContextTag`
// /// // - `impl RenderContext for MyContext<'a>`
// /// // - a private link for type-based downcasting
// /// impl_render_context!(MyContext<'a> => MyContextTag);
// /// ```
// #[macro_export]
// macro_rules! impl_render_context {
//     ($ctx:ty => $tag:ident) => {
//         pub struct $tag;
//
//         impl $crate::core::render::context::RenderContext for $ctx {
//             fn tag_id(&self) -> ::std::any::TypeId {
//                 ::std::any::TypeId::of::<$tag>()
//             }
//         }
//
//         impl $crate::core::render::HasContextTag for $ctx {
//             type Tag = $tag;
//         }
//     };
// }

// struct A;
// impl_render_context!(A => Atype);
