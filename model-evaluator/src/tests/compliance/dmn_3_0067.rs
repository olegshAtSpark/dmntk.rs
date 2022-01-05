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
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0067).unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision001", &ctx, r#"["John", "Doe"]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision001_a", &ctx, r#"["a", "b", "c", "", ""]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "decision004",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 0)"#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "decision005",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 1)"#,
  );
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision006", &ctx, r#"["foo", "bar"]"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision007", &ctx, r#"null(parameter 'string' not found)"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision008", &ctx, r#"null(split)"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision008_a", &ctx, r#"null(split)"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&DEFINITIONS, "decision008_b", &ctx, r#"null(split)"#);
}
