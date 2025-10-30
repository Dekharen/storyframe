/// Implements [`RenderContext`](crate::RenderContext) and [`HasContextTag`](crate::HasContextTag)
/// for a given context type, generating a unique tag type in the process.
///
/// # Example
/// ```
/// use storyframe::impl_render_context;
///
///struct Ui;
///
/// struct EguiContext<'a> {
///     ui: &'a mut Ui,
/// }
///
/// impl_render_context!(EguiContext<'_> => EguiContextTag);
/// ```
///
/// This generates:
/// ```ignore
/// pub struct EguiContextTag;
///
/// impl mylib::RenderContext for EguiContext<'_> {
///     fn tag_id(&self) -> std::any::TypeId {
///         std::any::TypeId::of::<EguiContextTag>()
///     }
/// }
///
/// impl mylib::HasContextTag for EguiContext<'_> {
///     type Tag = EguiContextTag;
/// }
/// ```
#[macro_export]
macro_rules! impl_render_context {
    ($ctx:ty => $tag:ident) => {
        /// Unique zero-sized tag type for the given render context.
        #[allow(non_camel_case_types)]
        pub struct $tag;

        impl $crate::RenderContext for $ctx {
            #[inline]
            fn tag_id(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<$tag>()
            }
        }

        impl $crate::HasContextTag for $ctx {
            type Tag = $tag;
        }
    };
}

#[macro_export]
macro_rules! register_domain_types {
    ($(
        $step_type:ty {

            aliases: [$($alias:literal),*],
            // states: [$($state_type:ty ),*]
            states: [$(
                $state_type:ty $({ required: [$($req:literal),*] })?
            ),*]
        }
    ),*) =>{
        // Compile-time trait bound checks
$(
    const _: () = {
        fn _assert_step_action<T: $crate::core::step::StepAction>() {}
        let _ = _assert_step_action::<$step_type>;
    };

    $(
        const _: () = {
            fn _assert_state<T: $crate::core::state::VisualizationState>() {}
            let _ = _assert_state::<$state_type>;
        };
    )*
)*
  lazy_static::lazy_static! {
            static ref STEP_TYPE_MAPPINGS: std::collections::HashMap<&'static str, (&'static str, $crate::core::input::processors::StepParserFn )> = {
                let mut map = std::collections::HashMap::new();

                $(
                    let type_id = <$step_type>::type_id();
                    let parser: fn(&str) -> Result<Box<dyn $crate::core::step::StepAction>, ParseError> =
                        |s| Ok(Box::new(<$step_type>::from_str(s)?));

                    // Insert canonical type_id
                    map.insert(type_id, (type_id, parser));

                    // Insert aliases
                    $(
                        map.insert($alias, (type_id, parser));
                    )*
                )*

                map
            };
        }
  lazy_static::lazy_static! {
    // step_type_id -> Vec<state_type_id>
 static ref STEP_TO_STATES: std::collections::HashMap<&'static str, Vec<$crate::core::state::StateInfo>> = {
    let mut map: std::collections::HashMap<&'static str, Vec<$crate::core::state::StateInfo>> = std::collections::HashMap::new();
    $(
        let step_type_id = <$step_type>::type_id();
        let states = vec![
            $(
                $crate::core::state::StateInfo {
                    type_id: <$state_type>::state_type_id(),
                    display_name: stringify!($state_type),
                    snapshot_type_id: <$state_type>::snapshot_type_id(),
                    factory: |input, configuration| Ok(Box::new(<$state_type>::parse(input, configuration)?)),
                    required_config_fields: &[$($($req),*)?],
                    is_default: false, // Will be set below
                },
            )*
        ];

        // Set first state as default
        let mut states = states;
        if let Some(first) = states.first_mut() {
            first.is_default = true;
        }

        map.insert(step_type_id, states);
    )*
    map
};

        }
        fn get_step_parser(step_type_id: &str) -> Option<$crate::core::input::processors::StepParserFn> {
            STEP_TYPE_MAPPINGS.get(step_type_id).map(|(_, parser)| *parser)
        }

        pub fn step_type_to_id(step_type: &str) -> Result<&'static str, ParseError> {
            STEP_TYPE_MAPPINGS.get(step_type)
                .map(|(canonical_id, _)| *canonical_id)
                .ok_or_else(|| ParseError::UnknownStepType {
                    step_type: step_type.to_string(),
                    supported_step_types: get_supported_step_types(),
                })
        }

        pub fn get_supported_step_types() -> Vec<String> {
            STEP_TYPE_MAPPINGS.keys().map(|s| s.to_string()).collect()
        }
/// Function creating the registry that will hold all information on how to handle steps and
        /// states.
pub fn create_registry() -> $crate::engine::registry::Registry {
    $crate::engine::registry::Registry::new(
         $crate::engine::registry::RendererRegistry::new(), // Empty, user populates
         $crate::engine::registry::StateRegistry::from_mappings(&STEP_TO_STATES),
        $crate::engine::registry::DomainRegistry::new(
             get_supported_step_types(),
             get_step_parser,
            step_type_to_id,
        )
        )
}
    };
}
