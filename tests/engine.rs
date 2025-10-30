use storyframe::{
    StepAction,
    algorithm::PuzzleSource,
    core::{
        configuration::Configuration,
        state::{VisualizationState, snapshot::StateSnapshot},
    },
    engine::VisualizationEngine,
    error::ParseError,
    impl_render_context, register_domain_types,
};

struct Ctx;

impl_render_context!(Ctx => CtxTag);

#[test]
fn test_engine_basic_puzzle_parsing() {
    let content = r#"
        title: Test Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.step_type: text_step
        part.tokenize.input: hello__world
        part.tokenize.steps: 0__"goodbye"______
    "#;

    let result = VisualizationEngine::from_source(PuzzleSource::String(content.to_string()));
    match &result {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    };
    assert!(result.is_ok());
    let mut engine = result.unwrap();
    engine
        .select_part(|selector| {
            for opt in selector.options_mut() {
                opt.select();
            }
        })
        .unwrap();
    let mut config = engine.configure_for_current_context::<Ctx>();
    config
        .set_state(|x| {
            for opt in x.options_mut() {
                opt.select();
            }
        })
        .unwrap();

    // config
    //        .set_renderer(|x| {
    //            for opt in x.options_mut() {
    //                opt.select();
    //            }
    //        });
}

#[test]
fn test_engine_invalid_step_type() {
    let content = r#"
        title: Bad Puzzle
        part.tokenize.name: Tokenization
        part.tokenize.input: hello world
        part.tokenize.steps: highlight_char 0
        part.tokenize.step_type: unknown_type
    "#;

    let result = VisualizationEngine::from_source(PuzzleSource::String(content.to_string()));
    assert!(matches!(
        result,
        Err(ParseError::UnknownStepType {
            step_type: _,
            supported_step_types: _
        })
    ));
}
#[test]
fn test_engine_missing_puzzle_fields() {
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
        test_engine_missing_field(content);
    }
}

fn test_engine_missing_field(content: &str) {
    let result = VisualizationEngine::from_source(PuzzleSource::String(content.to_string()));
    // TODO: we could also test for correct fmt
    assert!(matches!(result, Err(ParseError::MissingPartField(_, _))));
}

#[test]
fn test_engine_invalid_part_structure() {
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
        test_engine_invalid_part(content);
    }
}

fn test_engine_invalid_part(content: &str) {
    let result = VisualizationEngine::from_source(PuzzleSource::String(content.to_string()));
    // let result = PuzzleInstance::from_source(PuzzleSource::String(content.to_string()));
    // TODO: we could also test for correct fmt
    assert!(matches!(result, Err(ParseError::InvalidPartStructure(_))));
}
// register_domain_types!();

#[derive(Debug)]
struct TestStep;

impl StepAction for TestStep {
    fn type_id() -> &'static str
    where
        Self: Sized,
    {
        "test"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn from_str(_string: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}

struct TestSnapshot;
impl StateSnapshot for TestSnapshot {
    fn snapshot_type_id() -> &'static str
    where
        Self: Sized,
    {
        "test_snapshot"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
struct TestState;
impl VisualizationState for TestState {
    type Step = TestStep;

    type Snapshot = TestSnapshot;

    fn apply_step(&mut self, _step: &Self::Step) -> Result<(), storyframe::error::StepError> {
        Ok(())
    }

    fn seek_to_step(
        &mut self,
        _step_index: usize,
        _all_steps: &[&Self::Step],
    ) -> Result<(), storyframe::error::StepError> {
        Ok(())
    }

    fn create_snapshot(&self) -> Box<Self::Snapshot> {
        Box::new(TestSnapshot)
    }

    fn state_type_id() -> &'static str
    where
        Self: Sized,
    {
        "test_state"
    }

    fn parse(_input: &str, _configuration: &Configuration) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}

register_domain_types!(TestStep {
    aliases: ["t"],
    states: [TestState]
});
