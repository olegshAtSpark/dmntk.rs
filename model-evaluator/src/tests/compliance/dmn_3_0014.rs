/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * DMN model evaluator
 *
 * Copyright 2018-2022 Dariusz Depta Engos Software <dariusz.depta@engos.software>
 *
 * THE SOFTWARE IS PROVIDED "AS IS",  WITHOUT WARRANTY OF ANY KIND,  EXPRESS OR
 * IMPLIED,  INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,  DAMAGES OR OTHER
 * LIABILITY,  WHETHER IN AN ACTION OF CONTRACT,  TORT OR OTHERWISE,  ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::super::*;
use crate::model_evaluator::ModelEvaluator;
use std::sync::Arc;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0014);
}

#[test]
fn _0001() {
  let ctx = context(r#"{p: 1, r: 1, n: 1, pmt: 1}"#);
  assert_business_knowledge_model(&MODEL_EVALUATOR, "equity36Mo", &ctx, r#"1"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{p: 2, r: 1, n: 1, pmt: 1}"#);
  assert_business_knowledge_model(&MODEL_EVALUATOR, "equity36Mo", &ctx, r#"2.083333333333333333333333333333333"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{p: 1, r: 1, n: 1}"#);
  assert_business_knowledge_model(&MODEL_EVALUATOR, "monthlyPayment", &ctx, r#"1.083333333333333333333333333333338"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{p: 2, r: 1, n: 1}"#);
  assert_business_knowledge_model(&MODEL_EVALUATOR, "monthlyPayment", &ctx, r#"2.166666666666666666666666666666676"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{requestedAmt: 330000, product: {fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}}"#);
  assert_business_knowledge_model(
    &MODEL_EVALUATOR,
    "FinancialMetrics",
    &ctx,
    r#"{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}"#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "Bankrates",
    &ctx,
    r#"[{fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}, {fee: 2700, lenderName: "eClick Lending", points: 1.1, rate: 0.03200}, {fee: 1200, lenderName: "eClickLending", points: 0.1, rate: 0.03375}, {fee: 3966, lenderName: "AimLoan", points: 1.1, rate: 0.03000}, {fee: 285, lenderName: "Home Loans Today", points: 1.1, rate: 0.03125}, {fee: 4028, lenderName: "Sebonic", points: 0.1, rate: 0.03125}, {fee: 4317, lenderName: "AimLoan", points: 0.1, rate: 0.03125}, {fee: 2518, lenderName: "eRates Mortgage", points: 1.1, rate: 0.03125}, {fee: 822, lenderName: "Home Loans Today", points: 0.1, rate: 0.03250}, {fee: 1995, lenderName: "AimLoan", points: 0, rate: 0.03250}]"#,
  );
}

#[test]
fn _0007() {
  let ctx = context(r#"{RequestedAmt: 330000}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "RankedProducts",
    &ctx,
    r#"{metricsTable: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}], rankByDownPmt: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}], rankByEquityPct: [{downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}], rankByMonthlyPmt: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}], rankByRate: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}]}"#,
  );
}
