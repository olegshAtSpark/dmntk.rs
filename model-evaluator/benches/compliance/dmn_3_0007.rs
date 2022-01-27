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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0007);
}

const INPUT_DATA: &str = r#"{Day: 22, Hours: 12, Minutes: 59, Month: 11, Seconds: 1.3, Timezone: @"-PT1H", Year: 1999, dateString: "2015-12-24", dateTimeString: "2016-12-24T23:59:00-08:00", durationString: "P13DT2H14S", oneHour: PT1H, timeString: "00:00:01-01:00"}"#;

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Date-Time", &ctx, r#"2016-12-24T23:59:00-08:00"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(
    &MODEL_EVALUATOR,
    "Date",
    &ctx,
    r#"{fromDateTime: 2016-12-24, fromString: 2015-12-24, fromYearMonthDay: 1999-11-22}"#,
  );
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Time", &ctx, r#"00:00:01-01:00"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Date-Time2", &ctx, r#"2015-12-24T00:00:01-01:00"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Time2", &ctx, r#"00:00:01-01:00"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Time3", &ctx, r#"12:59:01.3-01:00"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "dtDuration1", &ctx, r#"P13DT2H14S"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "dtDuration2", &ctx, r#"P367DT6H58M59S"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "sumDurations", &ctx, r#"P380DT8H59M13S"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "ymDuration2", &ctx, r#"P1Y"#);
}

#[bench]
#[ignore]
fn _0011(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "cDay", &ctx, r#"24"#);
}

#[bench]
#[ignore]
fn _0012(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "cYear", &ctx, r#"2015"#);
}

#[bench]
#[ignore]
fn _0013(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "cMonth", &ctx, r#"12"#);
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "cHour", &ctx, r#"0"#);
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "cMinute", &ctx, r#"0"#);
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "cSecond", &ctx, r#"1"#);
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "cOffset", &ctx, r#"-PT1H"#);
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "years", &ctx, r#"1"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(INPUT_DATA);
  assert_decision(&MODEL_EVALUATOR, "seconds", &ctx, r#"14"#);
}
