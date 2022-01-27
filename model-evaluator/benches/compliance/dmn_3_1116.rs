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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_1116);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_001_bdf26fdc72", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_002_9d2e399b96", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_003_d1f0ea5bb9", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_004_57aea91d1c", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_005_32ea20b34f", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_006_e266498180", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_007_ee82c7bf12", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_008_08078c6c29", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_009_804c21ed52", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_010_cc773bb44b", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_011_ad5b3a26b5", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_012_3c2f416fc9", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_013_7f22c0bda8", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_014_0dc13176e8", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_015_376d693a79", &ctx, r#"12:00:00"#);
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "feel-time-function_ErrorCase_016_c3cccff405",
    &ctx,
    r#"null(expected 1,3,4 parameters, actual number of parameters is 0)"#,
  );
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_017_f3683885f5", &ctx, r#"01:02:03"#);
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_018_35f1f2cce8", &ctx, r#"00:00:00"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_019_879be89d63", &ctx, r#"11:22:33.444"#);
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_020_72b421086e", &ctx, r#"11:22:33.123456789"#);
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_021_5c50fa1dff", &ctx, r#"23:59:00Z"#);
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_022_55e76d3595", &ctx, r#"11:00:00Z"#);
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_023_5cbbb85435", &ctx, r#"00:00:00Z"#);
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_024_5f7f735e8f", &ctx, r#"13:20:00+02:00"#);
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_025_139b25b795", &ctx, r#"13:20:00-05:00"#);
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_026_c5208af118", &ctx, r#"11:22:33Z"#);
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_027_45082fd26c", &ctx, r#"11:22:33Z"#);
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_028_eaea7a943c", &ctx, r#""00:01:00@Etc/UTC""#);
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_029_f0d5c2c16a", &ctx, r#""00:01:00@Europe/Paris""#);
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_030_390d4f4648", &ctx, r#"10:20:00"#);
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_031_4d086a3b59", &ctx, r#"10:20:00Z"#);
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_032_d9b0d7f931", &ctx, r#"10:20:00Z"#);
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_033_8420160da1", &ctx, r#"10:20:00+01:00"#);
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_034_13c312c376", &ctx, r#"10:20:00-01:00"#);
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_035_fbfce88ac4", &ctx, r#"10:20:00Z"#);
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_036_eb05fabc01", &ctx, r#""10:20:00@Europe/Paris""#);
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_037_eed195f693", &ctx, r#""11:20:00@Asia/Dhaka""#);
}

#[bench]
fn _0038(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_038_05b311131c", &ctx, r#"11:59:45"#);
}

#[bench]
fn _0039(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_039_5b65992f0d", &ctx, r#"11:59:45Z"#);
}

#[bench]
fn _0040(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_040_6c9d17b491", &ctx, r#"11:59:45+02:00"#);
}

#[bench]
fn _0041(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_041_29a448d57e", &ctx, r#"11:59:45-02:00"#);
}

#[bench]
fn _0042(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_042_00146f2977", &ctx, r#"11:59:00+02:01"#);
}

#[bench]
fn _0043(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_043_2edfae8414", &ctx, r#"11:59:00-02:01"#);
}

#[bench]
fn _0044(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_044_3073ffd026", &ctx, r#"11:59:00+02:01"#);
}

#[bench]
fn _0045(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_045_ad1339e858", &ctx, r#"11:59:00-02:01"#);
}

#[bench]
fn _0046(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_046_7b80221ec1", &ctx, r#""11:59:45+02:45:55""#);
}

#[bench]
fn _0047(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_047_33cd7b9b15", &ctx, r#""11:59:45-02:45:55""#);
}

#[bench]
fn _0048(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_048_9bedd52886", &ctx, r#"11:59:45Z"#);
}

#[bench]
fn _0049(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_049_617d9e09d6", &ctx, r#"23:59:01"#);
}

#[bench]
fn _0050(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_050_524d9a8146", &ctx, r#"23:59:01.987654321"#);
}

#[bench]
fn _0051(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_051_a71d2a08f7", &ctx, r#"09:15:30+02:00"#);
}

#[bench]
fn _0052(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_052_d825d58888", &ctx, r#"09:15:30Z"#);
}

#[bench]
fn _0053(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_053_3d956966c0", &ctx, r#"00:00:00Z"#);
}

#[bench]
fn _0054(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_054_fdc3094237", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0055(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_055_9b47db6ea4", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0056(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_056_a8e828d64d", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0057(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_057_d039115cce", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0058(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_058_81dd4b1639", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0059(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_059_c7e1705fe1", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0060(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_060_0cf4734fae", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0061(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_061_da2717f085", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0062(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_062_6cd1313fa9", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0063(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_063_e85c40b474", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0064(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_064_df74038c67", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0065(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_065_79eaef6fee", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0066(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_066_5116e12fd3", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0067(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_067_8285edad7b", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0068(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_068_ad528abb23", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0069(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_069_5096701e2e", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0070(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_070_8b2e39f570", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0071(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_071_cf9417648b", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0072(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_072_4c8c3835e4", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0073(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_073_a5fc245959", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0074(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_074_387d4411ea", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0075(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_075_1606dda03d", &ctx, r#"null(time_1)"#);
}

#[bench]
fn _0076(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_076_cb117ca612", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0077(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_077_a4daad060c", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0078(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_078_c2fe73418b", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0079(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_ErrorCase_079_d2d226c3cd", &ctx, r#"null(time_4)"#);
}

#[bench]
fn _0080(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_080_2bbb8c86af", &ctx, r#"23:59:00"#);
}

#[bench]
fn _0081(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_081_69f4e0231e", &ctx, r#"12:45:00"#);
}

#[bench]
fn _0082(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_082_36a78e5396", &ctx, r#"11:59:00+02:01"#);
}

#[bench]
fn _0083(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "feel-time-function_083_6b608254c7", &ctx, r#"11:59:00-02:00"#);
}
