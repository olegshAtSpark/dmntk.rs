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
use crate::compliance::{assert_decision, assert_decision_service, context};
use dmntk_model_evaluator::ModelEvaluator;
use std::sync::Arc;
use test::Bencher;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0085);
}

#[bench]
fn _0001(b: &mut Bencher) {
  assert_decision_service(&MODEL_EVALUATOR, "decisionService_001", r#"{}"#, r#""foo""#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  assert_decision_service(&MODEL_EVALUATOR, "decisionService_002", r#"{decision_002_input: "baz"}"#, r#""foo baz""#);
}

#[bench]
fn _0002_a(b: &mut Bencher) {
  assert_decision_service(&MODEL_EVALUATOR, "decisionService_002", r#"{}"#, r#"null(addition err 2)"#);
}

#[bench]
fn _0002_b(b: &mut Bencher) {
  assert_decision_service(
    &MODEL_EVALUATOR,
    "decisionService_002",
    r#"{decision_002_input: null}"#,
    r#"null(addition err 2)"#,
  );
}

#[bench]
fn _0002_c(b: &mut Bencher) {
  assert_decision_service(
    &MODEL_EVALUATOR,
    "decisionService_002",
    r#"{decision_002_input: 1234}"#,
    r#"null(addition err 2)"#,
  );
}

#[bench]
fn _0003(b: &mut Bencher) {
  assert_decision_service(
    &MODEL_EVALUATOR,
    "decisionService_003",
    r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
    r#""A B C D""#,
  );
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "decision_004_1", &ctx, r#""foo""#);
}

#[bench]
#[ignore]
fn _0005(b: &mut Bencher) {
  assert_decision_service(&MODEL_EVALUATOR, "decisionService_005", r#"{}"#, r#"null"#);
}
