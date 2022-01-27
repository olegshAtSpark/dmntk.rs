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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_1120);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_001_f2c6cd6866", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "feel-duration-function_ErrorCase_002_ddca5756ca",
    &ctx,
    r#"null(expected 1 parameters, actual number of parameters is 0)"#,
  );
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_003_951e1d1c31", &ctx, r#"P1D"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_004_6b31e7cde7", &ctx, r#"PT2H"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_005_202d863d07", &ctx, r#"PT3M"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_006_a885f926d9", &ctx, r#"PT4S"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_007_2f0ad399f3", &ctx, r#"PT0.999S"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_008_747f56743d", &ctx, r#"P1DT2H3M4.123456789S"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_009_cef3c1ed26", &ctx, r#"PT0S"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_010_5b452a4975", &ctx, r#"PT0S"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_011_2169615b94", &ctx, r#"PT0S"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_012_2affe6d169", &ctx, r#"PT0S"#);
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_013_0e8e26513c", &ctx, r#"PT0S"#);
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_014_598ba6fabd", &ctx, r#"PT0S"#);
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_015_ce2cb09830", &ctx, r#"-PT1H2M"#);
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_016_af3e37fdbd", &ctx, r#"PT16H40M"#);
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_017_4f4549fda4", &ctx, r#"PT16H40M0.999999999S"#);
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_018_f5ec776811", &ctx, r#"PT9H15M"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_019_2e6885755a", &ctx, r#"PT1H1M"#);
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_020_af58b3766e", &ctx, r#"P1D"#);
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_021_e48e70ad4e", &ctx, r#"P10D"#);
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_022_668f24bed7", &ctx, r#"P2DT1H40M"#);
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_023_6fc32087db", &ctx, r#"PT1H"#);
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_024_fd7000d72f", &ctx, r#"P2DT4H34M"#);
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_025_f8ffbd8658", &ctx, r#"P1Y2M"#);
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_026_e6c47f0cae", &ctx, r#"P1Y"#);
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_027_33b7fb8704", &ctx, r#"P0M"#);
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_028_971b94f16d", &ctx, r#"P0M"#);
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_029_1a12a226cc", &ctx, r#"-P1Y"#);
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_030_afac0f2062", &ctx, r#"P2Y2M"#);
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_031_1ddad718b9", &ctx, r#"P3Y3M"#);
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_032_72c46a9ec9", &ctx, r#"P8Y4M"#);
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_033_5d1540abaf", &ctx, r#"-P8Y4M"#);
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_034_aa9cbb21a6", &ctx, r#"P83333333Y3M"#);
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_035_93eef01ae7", &ctx, r#"-P83333333Y3M"#);
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_036_5f2775875e", &ctx, r#"P99999999Y"#);
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_037_8c9ea9c0e6", &ctx, r#"-P99999999Y"#);
}

#[bench]
fn _0038(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_038_67dc4c254c", &ctx, r#"P1Y"#);
}

#[bench]
fn _0039(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_039_4aa0b67804", &ctx, r#"P1D"#);
}

#[bench]
fn _0040(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_040_7d8eae461f", &ctx, r#"P2Y2M"#);
}

#[bench]
fn _0041(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_041_264bc9d682", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0042(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_042_59a0000245", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0043(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_043_253815dc6c", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0044(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_044_f3b338d877", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0045(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_045_2ffcc37801", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0046(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_046_eb637de5f6", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0047(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_047_3210c46a5a", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0048(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_048_ab6244f767", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0049(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_049_2225b503a0", &ctx, r#"null(duration)"#);
}

#[bench]
fn _0050(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-duration-function_ErrorCase_050_dd2ef33bbd", &ctx, r#"null(duration)"#);
}
