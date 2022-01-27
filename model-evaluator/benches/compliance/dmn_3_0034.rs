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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0034);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(&MODEL_EVALUATOR, "decision A 1", &ctx, r#"{resolve A: "A"}"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(&MODEL_EVALUATOR, "decision A 2.1", &ctx, r#"{resolve A 1: {resolve A: "A"}}"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(&MODEL_EVALUATOR, "decision A 2.2", &ctx, r#"{resolve A 1: {resolve A: "A"}}"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision A 3",
    &ctx,
    r#"{resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}"#,
  );
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(&MODEL_EVALUATOR, "decision B 1", &ctx, r#"{resolve A: "A", resolve B: "B"}"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(&MODEL_EVALUATOR, "decision B 2.1", &ctx, r#"{resolve B 1: {resolve A: "A", resolve B: "B"}}"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(&MODEL_EVALUATOR, "decision B 2.2", &ctx, r#"{resolve B 1: {resolve A: "A", resolve B: "B"}}"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision B 3",
    &ctx,
    r#"{resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 2.1: {resolve B 1: {resolve A: "A", resolve B: "B"}}, resolve B 2.2: {resolve B 1: {resolve A: "A", resolve B: "B"}}}"#,
  );
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision C 1",
    &ctx,
    r#"{resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 3: {resolve A 3: {resolve A 2.1: {resolve A 1: {resolve A: "A"}}, resolve A 2.2: {resolve A 1: {resolve A: "A"}}}, resolve B 2.1: {resolve B 1: {resolve A: "A", resolve B: "B"}}, resolve B 2.2: {resolve B 1: {resolve A: "A", resolve B: "B"}}}, resolve C: "C"}"#,
  );
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision C 2",
    &ctx,
    r#""BKM I # BKM II # BKM III # decision C 2 # BKM IV # BKM III # decision C 2""#,
  );
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision C 3",
    &ctx,
    r#""BKM II # BKM III # decision C 3 # BKM IV # BKM III # decision C 3""#,
  );
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{A: "A",B: "B",C: "C"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision C 4",
    &ctx,
    r#"{resolve C 3: "BKM II # BKM III # decision C 3 # BKM IV # BKM III # decision C 3"}"#,
  );
}
