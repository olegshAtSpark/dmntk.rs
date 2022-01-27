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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0004);
}

const APPLICANT_DATA: &str = r#"
  {
    ApplicantData:  {
      Age:  35,
      EmploymentStatus:  "EMPLOYED",
      ExistingCustomer:  true,
      MaritalStatus:  "M",
      Monthly:  {
        Expenses:  2000,
        Income:  6000,
        Repayments:  0
      }
    },
    BureauData:  {
      Bankrupt:  false,
      CreditScore:  649
    },
    RequestedProduct:  {
      Amount:  350000,
      ProductType:  "STANDARD LOAN",
      Rate:  0.0395,
      Term:  360
    },
    SupportingDocuments:  "YES"
  }
"#;

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Adjudication", &ctx, r#""ACCEPT""#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "ApplicationRiskScore", &ctx, r#"130"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Pre-bureauRiskCategory", &ctx, r#""LOW""#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "BureauCallType", &ctx, r#""MINI""#);
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Post-bureauRiskCategory", &ctx, r#""LOW""#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "RequiredMonthlyInstallment", &ctx, r#"1680.8803256086347968"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Pre-bureauAffordability", &ctx, r#"true"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Eligibility", &ctx, r#""ELIGIBLE""#);
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Strategy", &ctx, r#""BUREAU""#);
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Post-bureauAffordability", &ctx, r#"true"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(APPLICANT_DATA);
  assert_decision(&MODEL_EVALUATOR, "Routing", &ctx, r#""ACCEPT""#);
}
