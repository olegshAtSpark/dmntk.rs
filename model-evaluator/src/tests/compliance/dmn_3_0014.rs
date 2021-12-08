/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * DMN model evaluator
 *
 * Copyright 2018-2021 Dariusz Depta Engos Software <dariusz.depta@engos.software>
 *
 * THE SOFTWARE IS PROVIDED "AS IS",  WITHOUT WARRANTY OF ANY KIND,  EXPRESS OR
 * IMPLIED,  INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,  DAMAGES OR OTHER
 * LIABILITY,  WHETHER IN AN ACTION OF CONTRACT,  TORT OR OTHERWISE,  ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::tests::{assert_business_knowledge_model, assert_decision, context};

lazy_static! {
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0014, "file: ///3_0014.dmn").unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{p: 1, r: 1, n: 1, pmt: 1}"#);
  assert_business_knowledge_model(&DEFINITIONS, "equity36Mo", &ctx, r#"1.0000000000000000000000000000"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{p: 2, r: 1, n: 1, pmt: 1}"#);
  assert_business_knowledge_model(&DEFINITIONS, "equity36Mo", &ctx, r#"2.0833333333333333333333333333"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{p: 1, r: 1, n: 1}"#);
  assert_business_knowledge_model(&DEFINITIONS, "monthlyPayment", &ctx, r#"1.0833333333333333333333333332"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{p: 2, r: 1, n: 1}"#);
  assert_business_knowledge_model(&DEFINITIONS, "monthlyPayment", &ctx, r#"2.1666666666666666666666666678"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{requestedAmt: 330000, product: {fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}}"#);
  assert_business_knowledge_model(
    &DEFINITIONS,
    "FinancialMetrics",
    &ctx,
    r#"{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782027, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.8474697691209029114153224, points: 0, rate: 0.03500}"#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &DEFINITIONS,
    "Bankrates",
    &ctx,
    r#"[{fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}, {fee: 2700, lenderName: "eClick Lending", points: 1.1, rate: 0.03200}, {fee: 1200, lenderName: "eClickLending", points: 0.1, rate: 0.03375}, {fee: 3966, lenderName: "AimLoan", points: 1.1, rate: 0.03000}, {fee: 285, lenderName: "Home Loans Today", points: 1.1, rate: 0.03125}, {fee: 4028, lenderName: "Sebonic", points: 0.1, rate: 0.03125}, {fee: 4317, lenderName: "AimLoan", points: 0.1, rate: 0.03125}, {fee: 2518, lenderName: "eRates Mortgage", points: 1.1, rate: 0.03125}, {fee: 822, lenderName: "Home Loans Today", points: 0.1, rate: 0.03250}, {fee: 1995, lenderName: "AimLoan", points: 0, rate: 0.03250}]"#,
  );
}

#[test]
fn _0007() {
  let ctx = context(r#"{RequestedAmt: 330000}"#);
  assert_decision(
    &DEFINITIONS,
    "RankedProducts",
    &ctx,
    r#"{metricsTable: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782027, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.8474697691209029114153224, points: 0, rate: 0.03500}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846155, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585135, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322264, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.6815658998635108192647055, points: 0.1, rate: 0.03375}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616326, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.3183537092755045448901731, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604212, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.4098900057388069237672151, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529226, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489151, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755701, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.5455983131948984640340423, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233002, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.9755138452872391770675227, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170882, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940629, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560612, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442951, points: 0, rate: 0.03250}], rankByDownPmt: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782027, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.8474697691209029114153224, points: 0, rate: 0.03500}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170882, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940629, points: 0.1, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322264, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.6815658998635108192647055, points: 0.1, rate: 0.03375}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560612, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442951, points: 0, rate: 0.03250}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604212, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.4098900057388069237672151, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529226, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489151, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755701, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.5455983131948984640340423, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233002, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.9755138452872391770675227, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846155, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585135, points: 1.1, rate: 0.03200}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616326, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.3183537092755045448901731, points: 1.1, rate: 0.03000}], rankByEquityPct: [{downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170882, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940629, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560612, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442951, points: 0, rate: 0.03250}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782027, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.8474697691209029114153224, points: 0, rate: 0.03500}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322264, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.6815658998635108192647055, points: 0.1, rate: 0.03375}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604212, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.4098900057388069237672151, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529226, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489151, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755701, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.5455983131948984640340423, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233002, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.9755138452872391770675227, points: 1.1, rate: 0.03125}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616326, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.3183537092755045448901731, points: 1.1, rate: 0.03000}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846155, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585135, points: 1.1, rate: 0.03200}], rankByMonthlyPmt: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616326, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.3183537092755045448901731, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604212, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.4098900057388069237672151, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529226, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489151, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755701, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.5455983131948984640340423, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233002, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.9755138452872391770675227, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170882, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940629, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560612, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442951, points: 0, rate: 0.03250}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846155, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585135, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322264, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.6815658998635108192647055, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782027, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.8474697691209029114153224, points: 0, rate: 0.03500}], rankByRate: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616326, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.3183537092755045448901731, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604212, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.4098900057388069237672151, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529226, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489151, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755701, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.5455983131948984640340423, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233002, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.9755138452872391770675227, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846155, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585135, points: 1.1, rate: 0.03200}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170882, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940629, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560612, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442951, points: 0, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322264, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.6815658998635108192647055, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782027, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.8474697691209029114153224, points: 0, rate: 0.03500}]}"#,
  );
}
