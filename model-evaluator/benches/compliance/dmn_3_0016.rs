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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0016);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "priceTable1",
    &ctx,
    r#"[{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]"#,
  );
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, "everyGtTen1", &ctx, r#"false"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, "everyGtTen2", &ctx, r#"false"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, "someGtTen1", &ctx, r#"true"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, "someGtTen2", &ctx, r#"true"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{priceTable2: [{itemName: "widget", price: 25}, {itemName: "sprocket", price: 15}, {itemName: "trinket", price: 1.5}]}"#);
  assert_decision(&MODEL_EVALUATOR, "everyGtTen3", &ctx, r#"false"#);
}
