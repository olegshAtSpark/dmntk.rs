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
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_2_0008, "file: ///2_0008.dmn").unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{loan: {principal: 600000,rate: 0.0375,termMonths: 360}}"#);
  assert_decision(&DEFINITIONS, "payment", &ctx, r#"2778.693549432766768088520383236299"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{loan: {principal: 30000,rate: 0.0475,termMonths: 60}}"#);
  assert_decision(&DEFINITIONS, "payment", &ctx, r#"562.7073593732659271562143285576524"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{loan: {principal: 600000,rate: 0.0399,termMonths: 360}}"#);
  assert_decision(&DEFINITIONS, "payment", &ctx, r#"2861.033777003901636716262779605767"#);
}
