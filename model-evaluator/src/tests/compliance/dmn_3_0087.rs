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
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0087, "file: ///3_0087.dmn").unwrap();
}

#[test]
#[ignore]
fn _0001() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 10000,Income: 100000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate:  .08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Strategy", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0002() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 10000,Income: 100000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Routing", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0003() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 10000,Income: 100000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Application risk score", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0004() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 10000,Income: 100000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Pre-bureau risk category", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0005() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 10000,Income: 100000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Bureau call type", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0006() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 10000,Income: 100000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Eligibility", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0007() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 10000,Income: 100000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Post-bureau affordability", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0008() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 3000,Income: 10000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Strategy", &ctx, r#"null"#);
}

#[test]
#[ignore]
fn _0009() {
  let ctx = context(
    r#"{Applicant data: {Age: 51,EmploymentStatus: "EMPLOYED",ExistingCustomer: false,MartitalStatus: "M",Monthly: {Expenses: 3000,Income: 10000,Repayments: 2500}},Bureau data: {Bankrupt: false,CreditScore: 600},Requested product: {Amount: 100000,ProductType: "STANDARD LOAN",Rate: 0.08,Term: 36}}"#,
  );
  assert_decision(&DEFINITIONS, "Routing", &ctx, r#"null"#);
}
