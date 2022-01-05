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
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0071).unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_001", &ctx, r#"false"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_002", &ctx, r#"true"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_003", &ctx, r#"true"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_004", &ctx, r#"true"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "number_005", &ctx, r#"false"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_001", &ctx, r#"false"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_002", &ctx, r#"true"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_003", &ctx, r#"true"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_004", &ctx, r#"true"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "string_005", &ctx, r#"false"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_001", &ctx, r#"false"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_002", &ctx, r#"true"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_003", &ctx, r#"true"#);
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_004", &ctx, r#"true"#);
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "date_005", &ctx, r#"false"#);
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_001", &ctx, r#"false"#);
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_002", &ctx, r#"true"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_003", &ctx, r#"true"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_004", &ctx, r#"true"#);
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "time_005", &ctx, r#"false"#);
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "datetime_001", &ctx, r#"false"#);
}

#[test]
fn _0022() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "datetime_002", &ctx, r#"true"#);
}

#[test]
fn _0023() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "datetime_003", &ctx, r#"true"#);
}

#[test]
fn _0024() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "datetime_004", &ctx, r#"true"#);
}

#[test]
fn _0025() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "datetime_005", &ctx, r#"false"#);
}

#[test]
fn _0026() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_001", &ctx, r#"false"#);
}

#[test]
fn _0027() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0028() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_003", &ctx, r#"true"#);
}

#[test]
fn _0029() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_004", &ctx, r#"true"#);
}

#[test]
fn _0030() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "ym_duration_005", &ctx, r#"false"#);
}

#[test]
fn _0031() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_001", &ctx, r#"false"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_002", &ctx, r#"true"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_003", &ctx, r#"true"#);
}

#[test]
fn _0034() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_004", &ctx, r#"true"#);
}

#[test]
fn _0035() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "dt_duration_005", &ctx, r#"false"#);
}
