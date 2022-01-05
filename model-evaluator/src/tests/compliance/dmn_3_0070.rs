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

use crate::tests::{assert_decision, context};

lazy_static! {
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0070).unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_001", &ctx, r#"false"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_002", &ctx, r#"false"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_003", &ctx, r#"false"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_004", &ctx, r#"false"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_005", &ctx, r#"false"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_006", &ctx, r#"false"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_007", &ctx, r#"false"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_009", &ctx, r#"false"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "null_010", &ctx, r#"false"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_001", &ctx, r#"true"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_002", &ctx, r#"true"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_003", &ctx, r#"false"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_004", &ctx, r#"false"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_005", &ctx, r#"false"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_006", &ctx, r#"false"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_007", &ctx, r#"false"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_009", &ctx, r#"false"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_010", &ctx, r#"false"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_001", &ctx, r#"true"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_002", &ctx, r#"false"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_003", &ctx, r#"true"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_004", &ctx, r#"false"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_005", &ctx, r#"false"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_006", &ctx, r#"false"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_007", &ctx, r#"false"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_009", &ctx, r#"false"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_010", &ctx, r#"false"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_001", &ctx, r#"true"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_002", &ctx, r#"false"#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_003", &ctx, r#"false"#);
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_004", &ctx, r#"true"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_005", &ctx, r#"false"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_006", &ctx, r#"false"#);
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_007", &ctx, r#"false"#);
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_009", &ctx, r#"false"#);
}

#[test]
fn _0036() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "boolean_010", &ctx, r#"false"#);
}

#[test]
fn _0037() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_001", &ctx, r#"true"#);
}

#[test]
fn _0038() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_002", &ctx, r#"false"#);
}

#[test]
fn _0039() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_003", &ctx, r#"false"#);
}

#[test]
fn _0040() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_004", &ctx, r#"false"#);
}

#[test]
fn _0041() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_005", &ctx, r#"true"#);
}

#[test]
fn _0042() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_006", &ctx, r#"false"#);
}

#[test]
fn _0043() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_007", &ctx, r#"false"#);
}

#[test]
fn _0044() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_009", &ctx, r#"false"#);
}

#[test]
fn _0045() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_010", &ctx, r#"false"#);
}

#[test]
fn _0046() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_001", &ctx, r#"true"#);
}

#[test]
fn _0047() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_002", &ctx, r#"false"#);
}

#[test]
fn _0048() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_003", &ctx, r#"false"#);
}

#[test]
fn _0049() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_004", &ctx, r#"false"#);
}

#[test]
fn _0050() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_005", &ctx, r#"false"#);
}

#[test]
fn _0051() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_006", &ctx, r#"true"#);
}

#[test]
fn _0052() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_007", &ctx, r#"false"#);
}

#[test]
fn _0053() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_009", &ctx, r#"false"#);
}

#[test]
fn _0054() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_010", &ctx, r#"false"#);
}

#[test]
fn _0055() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_001", &ctx, r#"true"#);
}

#[test]
fn _0056() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_002", &ctx, r#"false"#);
}

#[test]
fn _0057() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_003", &ctx, r#"false"#);
}

#[test]
fn _0058() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_004", &ctx, r#"false"#);
}

#[test]
fn _0059() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_005", &ctx, r#"false"#);
}

#[test]
fn _0060() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_006", &ctx, r#"false"#);
}

#[test]
fn _0061() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_007", &ctx, r#"false"#);
}

#[test]
fn _0062() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_009", &ctx, r#"false"#);
}

#[test]
fn _0063() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_010", &ctx, r#"false"#);
}

#[test]
fn _0064() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "list_016", &ctx, r#"false"#);
}

#[test]
fn _0065() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_001", &ctx, r#"true"#);
}

#[test]
fn _0066() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_002", &ctx, r#"false"#);
}

#[test]
fn _0067() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_003", &ctx, r#"false"#);
}

#[test]
fn _0068() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_004", &ctx, r#"false"#);
}

#[test]
fn _0069() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_005", &ctx, r#"false"#);
}

#[test]
fn _0070() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_006", &ctx, r#"false"#);
}

#[test]
fn _0071() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_007", &ctx, r#"false"#);
}

#[test]
fn _0072() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_009", &ctx, r#"true"#);
}

#[test]
fn _0073() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_010", &ctx, r#"false"#);
}

#[test]
fn _0074() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_001", &ctx, r#"true"#);
}

#[test]
fn _0075() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_002", &ctx, r#"false"#);
}

#[test]
fn _0076() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_003", &ctx, r#"false"#);
}

#[test]
fn _0077() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_004", &ctx, r#"false"#);
}

#[test]
fn _0078() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_005", &ctx, r#"false"#);
}

#[test]
fn _0079() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_006", &ctx, r#"false"#);
}

#[test]
fn _0080() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_007", &ctx, r#"false"#);
}

#[test]
fn _0081() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_009", &ctx, r#"false"#);
}

#[test]
fn _0082() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_010", &ctx, r#"true"#);
}

#[test]
fn _0083() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_001", &ctx, r#"true"#);
}

#[test]
fn _0084() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_002", &ctx, r#"false"#);
}

#[test]
fn _0085() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_003", &ctx, r#"false"#);
}

#[test]
fn _0086() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_004", &ctx, r#"false"#);
}

#[test]
fn _0087() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_005", &ctx, r#"false"#);
}

#[test]
fn _0088() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_006", &ctx, r#"false"#);
}

#[test]
fn _0089() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_007", &ctx, r#"false"#);
}

#[test]
fn _0090() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_009", &ctx, r#"false"#);
}

#[test]
fn _0091() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "context_010", &ctx, r#"false"#);
}

#[test]
fn _0092() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_001", &ctx, r#"true"#);
}

#[test]
fn _0093() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_002", &ctx, r#"false"#);
}

#[test]
fn _0094() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_003", &ctx, r#"false"#);
}

#[test]
fn _0095() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_004", &ctx, r#"false"#);
}

#[test]
fn _0096() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_005", &ctx, r#"false"#);
}

#[test]
fn _0097() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_006", &ctx, r#"false"#);
}

#[test]
fn _0098() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_007", &ctx, r#"false"#);
}

#[test]
fn _0099() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_009", &ctx, r#"false"#);
}

#[test]
fn _0100() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "function_010", &ctx, r#"false"#);
}
