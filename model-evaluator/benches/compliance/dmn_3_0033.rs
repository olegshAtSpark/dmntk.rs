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
  assert_decision(&MODEL_EVALUATOR, "increase1", &ctx, r#"[11, 21, 31]"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{heights: [10, 20, 30], widths: [2, 3]}"#);
  assert_decision(&MODEL_EVALUATOR, "areas", &ctx, r#"[20, 30, 40, 60, 60, 90]"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{factors: [2, 3, 5, 7, 11], value: 35}"#);
  assert_decision(&MODEL_EVALUATOR, "check factors", &ctx, r#"[false, false, true, true, false]"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{value: 10}"#);
  assert_decision(&MODEL_EVALUATOR, "multiples", &ctx, r#"[20, 30, 40, 50]"#);
}
