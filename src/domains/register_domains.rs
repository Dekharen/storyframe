//TODO: Change these &'static str types to at least equivalent IDs

#[macro_export]
macro_rules! register_domain_types {
    ($(
        $step_type:ty {

            aliases: [$($alias:literal),*],
            states: [$($state_type:ty ),*]
        }
    ),*) =>{
        // Compile-time trait bound checks
        const _: () = {
            $(
                // Force compile error if $step_type doesn't implement StepAction
                fn _assert_step_action<T: $crate::core::step::StepAction>() {}
                let _ = _assert_step_action::<$step_type>;

                $(
                    // Force compile error if $state_type doesn't implement VisualizationState
                    fn _assert_state<T: $crate::core::state::VisualizationState>() {}
                    let _ = _assert_state::<$state_type>;
                )*
            )*
        };
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
                    factory: |input| Ok(Box::new(<$state_type>::parse(input)?)),
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
