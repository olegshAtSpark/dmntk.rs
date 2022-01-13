/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * DMN model evaluator
 *
 * Copyright 2018-2022 Dariusz Depta Engos Software <dariusz.depta@engos.software>
 *
 * THE SOFTWARE IS PROVIDED "AS IS",  WITHOUT WARRANTY OF ANY KIND,  EXPRESS OR
 * IMPLIED,  INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,  DAMAGES OR OTHER
 * LIABILITY,  WHETHER IN AN ACTION OF CONTRACT,  TORT OR OTHERWISE,  ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::super::*;
use crate::model_evaluator::ModelEvaluator;
use std::sync::Arc;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0011);
}

#[test]
fn _0001() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, "literalNestedList", &ctx, r#"[["a", "b"], ["b", "c"]]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, "remove1", &ctx, r#"["a", "c"]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, "insert3", &ctx, r#"[["o"], ["a", "b", "c"], ["p", "q"]]"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, "insert2", &ctx, r#"[["a", "b"], ["a", "b", "c"], ["b", "c"]]"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, "remove2", &ctx, r#"[["a", "b"]]"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&MODEL_EVALUATOR, "insert1", &ctx, r#"["a", "x", "b", "c"]"#);
}
