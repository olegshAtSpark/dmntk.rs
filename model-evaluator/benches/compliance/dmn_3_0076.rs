/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::build_model_evaluator;
use crate::compliance::{assert_decision, context};
use dmntk_model_evaluator::ModelEvaluator;
use std::sync::Arc;
use test::Bencher;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0076);
}

#[bench]
#[ignore]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boxed_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "incorrect_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "incorrect_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "incorrect_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_005";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_006";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_007";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_007_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_008";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_009";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_010";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_011";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "literal_012";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
#[ignore]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "varargs_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}
