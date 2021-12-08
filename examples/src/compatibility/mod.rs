/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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

//! Test files containing decision model definitions.
//! File names begin with the compliance level, followed by the number of the test.
//! Test examples are based on examples contained in
//! [DMN™ Technology Compatibility Kit](https://dmn-tck.github.io/tck/) project.
//! The content of each test file is modified and differs from the original.

// compliance level 2
pub const DMN_2_0001: &str = include_str!("level_2/2_0001.dmn");
pub const DMN_2_0002: &str = include_str!("level_2/2_0002.dmn");
pub const DMN_2_0003: &str = include_str!("level_2/2_0003.dmn");
pub const DMN_2_0004: &str = include_str!("level_2/2_0004.dmn");
pub const DMN_2_0005: &str = include_str!("level_2/2_0005.dmn");
pub const DMN_2_0006: &str = include_str!("level_2/2_0006.dmn");
pub const DMN_2_0007: &str = include_str!("level_2/2_0007.dmn");
pub const DMN_2_0008: &str = include_str!("level_2/2_0008.dmn");
pub const DMN_2_0009: &str = include_str!("level_2/2_0009.dmn");
pub const DMN_2_0010: &str = include_str!("level_2/2_0010.dmn");
pub const DMN_2_0100: &str = include_str!("level_2/2_0100.dmn");
pub const DMN_2_0101: &str = include_str!("level_2/2_0101.dmn");
pub const DMN_2_0102: &str = include_str!("level_2/2_0102.dmn");
pub const DMN_2_0105: &str = include_str!("level_2/2_0105.dmn");
pub const DMN_2_0106: &str = include_str!("level_2/2_0106.dmn");
pub const DMN_2_0107: &str = include_str!("level_2/2_0107.dmn");
pub const DMN_2_0108: &str = include_str!("level_2/2_0108.dmn");
pub const DMN_2_0109: &str = include_str!("level_2/2_0109.dmn");
pub const DMN_2_0110: &str = include_str!("level_2/2_0110.dmn");
pub const DMN_2_0111: &str = include_str!("level_2/2_0111.dmn");
pub const DMN_2_0112: &str = include_str!("level_2/2_0112.dmn");
pub const DMN_2_0113: &str = include_str!("level_2/2_0113.dmn");
pub const DMN_2_0114: &str = include_str!("level_2/2_0114.dmn");
pub const DMN_2_0115: &str = include_str!("level_2/2_0115.dmn");
pub const DMN_2_0116: &str = include_str!("level_2/2_0116.dmn");
pub const DMN_2_0117: &str = include_str!("level_2/2_0117.dmn");
pub const DMN_2_0118: &str = include_str!("level_2/2_0118.dmn");
pub const DMN_2_0119: &str = include_str!("level_2/2_0119.dmn");

// compliance level 3
pub const DMN_3_0001: &str = include_str!("level_3/3_0001.dmn");
pub const DMN_3_0002: &str = include_str!("level_3/3_0002.dmn");
pub const DMN_3_0003: &str = include_str!("level_3/3_0003.dmn");
pub const DMN_3_0004: &str = include_str!("level_3/3_0004.dmn");
pub const DMN_3_0005: &str = include_str!("level_3/3_0005.dmn");
pub const DMN_3_0006: &str = include_str!("level_3/3_0006.dmn");
pub const DMN_3_0007: &str = include_str!("level_3/3_0007.dmn");
pub const DMN_3_0008: &str = include_str!("level_3/3_0008.dmn");
pub const DMN_3_0009: &str = include_str!("level_3/3_0009.dmn");
pub const DMN_3_0010: &str = include_str!("level_3/3_0010.dmn");
pub const DMN_3_0011: &str = include_str!("level_3/3_0011.dmn");
pub const DMN_3_0012: &str = include_str!("level_3/3_0012.dmn");
pub const DMN_3_0013: &str = include_str!("level_3/3_0013.dmn");
pub const DMN_3_0014: &str = include_str!("level_3/3_0014.dmn");
pub const DMN_3_0016: &str = include_str!("level_3/3_0016.dmn");
pub const DMN_3_0017: &str = include_str!("level_3/3_0017.dmn");
pub const DMN_3_0020: &str = include_str!("level_3/3_0020.dmn");
pub const DMN_3_0021: &str = include_str!("level_3/3_0021.dmn");
pub const DMN_3_0030: &str = include_str!("level_3/3_0030.dmn");
pub const DMN_3_0031: &str = include_str!("level_3/3_0031.dmn");
pub const DMN_3_0032: &str = include_str!("level_3/3_0032.dmn");
pub const DMN_3_0033: &str = include_str!("level_3/3_0033.dmn");
pub const DMN_3_0034: &str = include_str!("level_3/3_0034.dmn");
pub const DMN_3_0035: &str = include_str!("level_3/3_0035.dmn");
pub const DMN_3_0036: &str = include_str!("level_3/3_0036.dmn");
pub const DMN_3_0037: &str = include_str!("level_3/3_0037.dmn");
pub const DMN_3_0038: &str = include_str!("level_3/3_0038.dmn");
pub const DMN_3_0039: &str = include_str!("level_3/3_0039.dmn");
pub const DMN_3_0040: &str = include_str!("level_3/3_0040.dmn");
pub const DMN_3_0041: &str = include_str!("level_3/3_0041.dmn");
pub const DMN_3_0050: &str = include_str!("level_3/3_0050.dmn");
pub const DMN_3_0051: &str = include_str!("level_3/3_0051.dmn");
pub const DMN_3_0052: &str = include_str!("level_3/3_0052.dmn");
pub const DMN_3_0053: &str = include_str!("level_3/3_0053.dmn");
pub const DMN_3_0054: &str = include_str!("level_3/3_0054.dmn");
pub const DMN_3_0055: &str = include_str!("level_3/3_0055.dmn");
pub const DMN_3_0056: &str = include_str!("level_3/3_0056.dmn");
pub const DMN_3_0057: &str = include_str!("level_3/3_0057.dmn");
pub const DMN_3_0058: &str = include_str!("level_3/3_0058.dmn");
pub const DMN_3_0059: &str = include_str!("level_3/3_0059.dmn");
pub const DMN_3_0060: &str = include_str!("level_3/3_0060.dmn");
pub const DMN_3_0061: &str = include_str!("level_3/3_0061.dmn");
pub const DMN_3_0062: &str = include_str!("level_3/3_0062.dmn");
pub const DMN_3_0063: &str = include_str!("level_3/3_0063.dmn");
pub const DMN_3_0064: &str = include_str!("level_3/3_0064.dmn");
pub const DMN_3_0065: &str = include_str!("level_3/3_0065.dmn");
pub const DMN_3_0066: &str = include_str!("level_3/3_0066.dmn");
pub const DMN_3_0067: &str = include_str!("level_3/3_0067.dmn");
pub const DMN_3_0068: &str = include_str!("level_3/3_0068.dmn");
pub const DMN_3_0069: &str = include_str!("level_3/3_0069.dmn");
pub const DMN_3_0070: &str = include_str!("level_3/3_0070.dmn");
pub const DMN_3_0071: &str = include_str!("level_3/3_0071.dmn");
pub const DMN_3_0072: &str = include_str!("level_3/3_0072.dmn");
pub const DMN_3_0073: &str = include_str!("level_3/3_0073.dmn");
pub const DMN_3_0074: &str = include_str!("level_3/3_0074.dmn");
pub const DMN_3_0075: &str = include_str!("level_3/3_0075.dmn");
pub const DMN_3_0076: &str = include_str!("level_3/3_0076.dmn");
pub const DMN_3_0077: &str = include_str!("level_3/3_0077.dmn");
pub const DMN_3_0078: &str = include_str!("level_3/3_0078.dmn");
pub const DMN_3_0080: &str = include_str!("level_3/3_0080.dmn");
pub const DMN_3_0081: &str = include_str!("level_3/3_0081.dmn");
pub const DMN_3_0082: &str = include_str!("level_3/3_0082.dmn");
pub const DMN_3_0083: &str = include_str!("level_3/3_0083.dmn");
pub const DMN_3_0084: &str = include_str!("level_3/3_0084.dmn");
pub const DMN_3_0085: &str = include_str!("level_3/3_0085.dmn");
pub const DMN_3_0086: &str = include_str!("level_3/3_0086.dmn");
pub const DMN_3_0087: &str = include_str!("level_3/3_0087.dmn");
pub const DMN_3_0088: &str = include_str!("level_3/3_0088.dmn");
pub const DMN_3_0089: &str = include_str!("level_3/3_0089.dmn");
pub const DMN_3_0090: &str = include_str!("level_3/3_0090.dmn");
pub const DMN_3_1100: &str = include_str!("level_3/3_1100.dmn");
pub const DMN_3_1101: &str = include_str!("level_3/3_1101.dmn");
pub const DMN_3_1102: &str = include_str!("level_3/3_1102.dmn");
pub const DMN_3_1103: &str = include_str!("level_3/3_1103.dmn");
pub const DMN_3_1104: &str = include_str!("level_3/3_1104.dmn");
pub const DMN_3_1105: &str = include_str!("level_3/3_1105.dmn");
pub const DMN_3_1106: &str = include_str!("level_3/3_1106.dmn");
pub const DMN_3_1107: &str = include_str!("level_3/3_1107.dmn");
pub const DMN_3_1108: &str = include_str!("level_3/3_1108.dmn");
pub const DMN_3_1109: &str = include_str!("level_3/3_1109.dmn");
pub const DMN_3_1110: &str = include_str!("level_3/3_1110.dmn");
pub const DMN_3_1115: &str = include_str!("level_3/3_1115.dmn");
pub const DMN_3_1116: &str = include_str!("level_3/3_1116.dmn");
pub const DMN_3_1117: &str = include_str!("level_3/3_1117.dmn");
pub const DMN_3_1120: &str = include_str!("level_3/3_1120.dmn");
pub const DMN_3_1121: &str = include_str!("level_3/3_1121.dmn");
pub const DMN_3_2891: &str = include_str!("level_3/3_2891.dmn");
pub const DMN_3_2892: &str = include_str!("level_3/3_2892.dmn");
pub const DMN_3_2893: &str = include_str!("level_3/3_2893.dmn");
pub const DMN_3_2894: &str = include_str!("level_3/3_2894.dmn");
pub const DMN_3_2895: &str = include_str!("level_3/3_2895.dmn");

// non compliant
pub const DMN_N_0015: &str = include_str!("non_compliant/N_0015.dmn");
pub const DMN_N_0019: &str = include_str!("non_compliant/N_0019.dmn");
pub const DMN_N_0079: &str = include_str!("non_compliant/N_0079.dmn");
pub const DMN_N_0088: &str = include_str!("non_compliant/N_0088.dmn");
