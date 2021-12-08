/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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

use crate::tests::{assert_decision, context};

lazy_static! {
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0082, "file: ///3_0082.dmn").unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_001", &ctx, r#"null"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_003", &ctx, r#"null"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_004", &ctx, r#"{age: 10, name: "foo", surname: "bar"}"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_005", &ctx, r#"null"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_006", &ctx, r#"["foo"]"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_006_a", &ctx, r#"null"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_007", &ctx, r#""foo""#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_007_a", &ctx, r#"null"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_008", &ctx, r#"null"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_bkm_001", &ctx, r#"true"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_bkm_002", &ctx, r#"false"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_bkm_003", &ctx, r#"null"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_bkm_004_a", &ctx, r#"null"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_bkm_004_b", &ctx, r#"null"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_bkm_005", &ctx, r#"10"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision_bkm_005_a", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "invoke_001", &ctx, r#"false"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "invoke_002", &ctx, r#"null"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "invoke_004", &ctx, r#"null"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "invoke_005", &ctx, r#"null"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "invoke_006", &ctx, r#"null"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "fd_001", &ctx, r#"10"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "fd_002", &ctx, r#"null"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "literal_001", &ctx, r#"10"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "literal_002", &ctx, r#"null"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "literal_004", &ctx, r#"null"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "literal_005", &ctx, r#"10"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "literal_006", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "ds_invoke_002_with_number",
    &ctx,
    r#"null(feel-evaluator: expected built-in function name or function definition, actual value is null(context has no value for key '["decisionService_002"]'))"#,
  );
}

#[test]
#[ignore]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "ds_invoke_002_with_singleton_list",
    &ctx,
    r#"null(feel-evaluator: expected built-in function name or function definition, actual value is null(context has no value for key '["decisionService_002"]'))"#,
  );
}
