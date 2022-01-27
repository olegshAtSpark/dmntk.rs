use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::Scope;
use dmntk_model_evaluator::ModelEvaluator;
use std::sync::Arc;

mod dmn_2_0001;

/// Utility function that builds a model evaluator from XML model definitions.
fn build_model_evaluator(model_content: &str) -> Arc<ModelEvaluator> {
  ModelEvaluator::new(&dmntk_model::parse(model_content).unwrap()).unwrap()
}

/// Utility function that creates a `FEEL` context from specified input expression.
pub fn context(input: &str) -> FeelContext {
  let scope = Scope::default();
  match dmntk_feel_parser::parse_context(&scope, input, false) {
    Ok(node) => match dmntk_feel_evaluator::prepare(&node) {
      Ok(evaluator) => match evaluator(&scope) {
        Value::Context(ctx) => ctx,
        other => panic!("ERROR: expected context value, actual value is: {}", other as Value),
      },
      Err(reason) => panic!("ERROR: building evaluator failed with reason: {}", reason),
    },
    Err(reason) => panic!("ERROR: parsing context failed with reason: {}", reason),
  }
}

/// Utility function that evaluates a `Decision` specified by name and compares the result.
fn assert_decision(model_evaluator: &ModelEvaluator, name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision does not match the expected value:\n  expected: {}\n    actual: {}\n",
    expected, actual
  );
}
