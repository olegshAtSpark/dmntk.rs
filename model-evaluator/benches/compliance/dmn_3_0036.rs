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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0036);
}

#[bench]
#[ignore]
fn _0001(b: &mut Bencher) {
  let ctx = context(
    r#"
      {
        "Another Date":  @"2018-07-31",
        "Another Date and Time":  @"2018-07-31T17: 13: 00Z",
        "Another Days and Time Duration":  @"PT12H",
        "Another String":  "Hello",
        "Another Time":  @"17: 13: 00",
        "Another Years and Months Duration":  @"P8M",
        "Another boolean":  false,
        "Another number":  15,
        Complex:  {
          aBoolean:  true,
          aDate:  @"2018-07-30",
          aDateTime:  @"2018-07-30T16: 12: 00Z",
          aDaysAndTimeDuration:  @"PT10H",
          aNumber:  10,
          aString:  "Hi",
          aTime:  @"16: 11: 00",
          aYearsAndMonthsDuration:  @"P5M"
        }
      }
    "#,
  );
  let invocable_name = "Compare Boolean";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

/*

#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare String", &ctx, r#""Different String""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time:  7: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Date", &ctx, r#""Future Date""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Number", &ctx, r#""Bigger""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Date and Time", &ctx, r#""Future date time""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Days and Time Duration", &ctx, r#""Longer duration""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Years and Months Duration", &ctx, r#""Longer duration""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Time", &ctx, r#""Future Time""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Boolean", &ctx, r#"null(no rules matched, no output value defined)"#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare String", &ctx, r#""Different String""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Date", &ctx, r#""Past Date""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Number", &ctx, r#""Smaller""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Date and Time", &ctx, r#""Past date time""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Days and Time Duration", &ctx, r#""Shorter duration""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Years and Months Duration", &ctx, r#""Shorter duration""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Time", &ctx, r#""Past Time""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Boolean", &ctx, r#"null(no rules matched, no output value defined)"#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare String", &ctx, r#""Same String""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Date", &ctx, r#""Same Date""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Number", &ctx, r#""Equals""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Date and Time", &ctx, r#""Same date time""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Days and Time Duration", &ctx, r#""Same duration""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Years and Months Duration", &ctx, r#""Same duration""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Time", &ctx, r#""Same Time""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}


#[bench]
fn _000() {
let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
   assert_decision(&MODEL_EVALUATOR, "Compare Boolean", &ctx, r#""Same boolean""#); b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

 */
