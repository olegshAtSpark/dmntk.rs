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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0070);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_001", &ctx, r#"false"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_002", &ctx, r#"false"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_003", &ctx, r#"false"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_004", &ctx, r#"false"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_005", &ctx, r#"false"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_006", &ctx, r#"false"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_007", &ctx, r#"false"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_009", &ctx, r#"false"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "null_010", &ctx, r#"false"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_001", &ctx, r#"true"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_002", &ctx, r#"true"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_003", &ctx, r#"false"#);
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_004", &ctx, r#"false"#);
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_005", &ctx, r#"false"#);
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_006", &ctx, r#"false"#);
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_007", &ctx, r#"false"#);
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_009", &ctx, r#"false"#);
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_010", &ctx, r#"false"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_001", &ctx, r#"true"#);
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_002", &ctx, r#"false"#);
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_003", &ctx, r#"true"#);
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_004", &ctx, r#"false"#);
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_005", &ctx, r#"false"#);
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_006", &ctx, r#"false"#);
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_007", &ctx, r#"false"#);
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_009", &ctx, r#"false"#);
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_010", &ctx, r#"false"#);
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_001", &ctx, r#"true"#);
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_002", &ctx, r#"false"#);
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_003", &ctx, r#"false"#);
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_004", &ctx, r#"true"#);
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_005", &ctx, r#"false"#);
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_006", &ctx, r#"false"#);
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_007", &ctx, r#"false"#);
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_009", &ctx, r#"false"#);
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_010", &ctx, r#"false"#);
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_001", &ctx, r#"true"#);
}

#[bench]
fn _0038(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_002", &ctx, r#"false"#);
}

#[bench]
fn _0039(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_003", &ctx, r#"false"#);
}

#[bench]
fn _0040(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_004", &ctx, r#"false"#);
}

#[bench]
fn _0041(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_005", &ctx, r#"true"#);
}

#[bench]
fn _0042(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_006", &ctx, r#"false"#);
}

#[bench]
fn _0043(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_007", &ctx, r#"false"#);
}

#[bench]
fn _0044(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_009", &ctx, r#"false"#);
}

#[bench]
fn _0045(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_010", &ctx, r#"false"#);
}

#[bench]
fn _0046(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_001", &ctx, r#"true"#);
}

#[bench]
fn _0047(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_002", &ctx, r#"false"#);
}

#[bench]
fn _0048(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_003", &ctx, r#"false"#);
}

#[bench]
fn _0049(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_004", &ctx, r#"false"#);
}

#[bench]
fn _0050(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_005", &ctx, r#"false"#);
}

#[bench]
fn _0051(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_006", &ctx, r#"true"#);
}

#[bench]
fn _0052(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_007", &ctx, r#"false"#);
}

#[bench]
fn _0053(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_009", &ctx, r#"false"#);
}

#[bench]
fn _0054(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_010", &ctx, r#"false"#);
}

#[bench]
fn _0055(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_001", &ctx, r#"true"#);
}

#[bench]
fn _0056(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_002", &ctx, r#"false"#);
}

#[bench]
fn _0057(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_003", &ctx, r#"false"#);
}

#[bench]
fn _0058(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_004", &ctx, r#"false"#);
}

#[bench]
fn _0059(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_005", &ctx, r#"false"#);
}

#[bench]
fn _0060(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_006", &ctx, r#"false"#);
}

#[bench]
fn _0061(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_007", &ctx, r#"false"#);
}

#[bench]
fn _0062(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_009", &ctx, r#"false"#);
}

#[bench]
fn _0063(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_010", &ctx, r#"false"#);
}

#[bench]
fn _0064(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_016", &ctx, r#"false"#);
}

#[bench]
fn _0065(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_001", &ctx, r#"true"#);
}

#[bench]
fn _0066(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_002", &ctx, r#"false"#);
}

#[bench]
fn _0067(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_003", &ctx, r#"false"#);
}

#[bench]
fn _0068(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_004", &ctx, r#"false"#);
}

#[bench]
fn _0069(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_005", &ctx, r#"false"#);
}

#[bench]
fn _0070(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_006", &ctx, r#"false"#);
}

#[bench]
fn _0071(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_007", &ctx, r#"false"#);
}

#[bench]
fn _0072(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_009", &ctx, r#"true"#);
}

#[bench]
fn _0073(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_010", &ctx, r#"false"#);
}

#[bench]
fn _0074(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_001", &ctx, r#"true"#);
}

#[bench]
fn _0075(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_002", &ctx, r#"false"#);
}

#[bench]
fn _0076(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_003", &ctx, r#"false"#);
}

#[bench]
fn _0077(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_004", &ctx, r#"false"#);
}

#[bench]
fn _0078(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_005", &ctx, r#"false"#);
}

#[bench]
fn _0079(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_006", &ctx, r#"false"#);
}

#[bench]
fn _0080(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_007", &ctx, r#"false"#);
}

#[bench]
fn _0081(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_009", &ctx, r#"false"#);
}

#[bench]
fn _0082(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_010", &ctx, r#"true"#);
}

#[bench]
fn _0083(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_001", &ctx, r#"true"#);
}

#[bench]
fn _0084(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_002", &ctx, r#"false"#);
}

#[bench]
fn _0085(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_003", &ctx, r#"false"#);
}

#[bench]
fn _0086(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_004", &ctx, r#"false"#);
}

#[bench]
fn _0087(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_005", &ctx, r#"false"#);
}

#[bench]
fn _0088(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_006", &ctx, r#"false"#);
}

#[bench]
fn _0089(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_007", &ctx, r#"false"#);
}

#[bench]
fn _0090(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_009", &ctx, r#"false"#);
}

#[bench]
fn _0091(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_010", &ctx, r#"false"#);
}

#[bench]
fn _0092(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_001", &ctx, r#"true"#);
}

#[bench]
fn _0093(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_002", &ctx, r#"false"#);
}

#[bench]
fn _0094(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_003", &ctx, r#"false"#);
}

#[bench]
fn _0095(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_004", &ctx, r#"false"#);
}

#[bench]
fn _0096(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_005", &ctx, r#"false"#);
}

#[bench]
fn _0097(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_006", &ctx, r#"false"#);
}

#[bench]
fn _0098(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_007", &ctx, r#"false"#);
}

#[bench]
fn _0099(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_009", &ctx, r#"false"#);
}

#[bench]
fn _0100(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "function_010", &ctx, r#"false"#);
}
