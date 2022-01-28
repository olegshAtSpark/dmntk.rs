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

use super::build_model_evaluator;
use crate::compliance::{assert_decision, context};
use dmntk_model_evaluator::ModelEvaluator;
use std::sync::Arc;
use test::Bencher;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0033);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{heights: [10, 20, 30]}"#);
  let invocable_name = "increase1";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"[11, 21, 31]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{heights: [10, 20, 30], widths: [2, 3]}"#);
  let invocable_name = "areas";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"[20, 30, 40, 60, 60, 90]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{factors: [2, 3, 5, 7, 11], value: 35}"#);
  let invocable_name = "check factors";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"[false, false, true, true, false]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{value: 10}"#);
  let invocable_name = "multiples";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"[20, 30, 40, 50]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}
