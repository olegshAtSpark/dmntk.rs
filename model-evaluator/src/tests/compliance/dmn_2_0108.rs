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
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_2_0108, "file: ///2_0108.dmn").unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{Age: 19,RiskCategory: "Medium",isAffordable: true}"#);
  assert_decision(&DEFINITIONS, "Approval", &ctx, r#"{Rate: "Best", Status: "Approved"}"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{Age: 13,RiskCategory: "Medium",isAffordable: true}"#);
  assert_decision(&DEFINITIONS, "Approval", &ctx, r#"{Rate: "Standard", Status: "Approved"}"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{Age: 10,RiskCategory: "Low",isAffordable: true}"#);
  assert_decision(&DEFINITIONS, "Approval", &ctx, r#"{Rate: "Standard", Status: "Declined"}"#);
}