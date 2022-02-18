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

//! Examples of horizontal decision tables in text format.
//!
//! Naming convention:
//!
//! ```text
//! ┌────────────── table orientation: h - horizontal (rules as rows)
//! │ ┌──────────── flag indicating if information item name is: absent (0) or present (1)
//! │ │┌─────────── flag indicating if output label is: absent (0) or present (1)
//! │ ││┌────────── flag indicating if allowed values are: absent (0) or present (1)
//! │ │││┌───────── number of input clauses: 0,1,2...
//! │ ││││┌──────── number of output clauses: 1,2...
//! │ │││││┌─────── number of annotation clauses: 0,1,2...  
//! h_000010.dtb
//! ```

/// Horizontal, no information item name, no output label, no allowed values, no inputs, single output, no annotations.
///
/// ```text
/// PREFERRED ORIENTATION: horizontal
/// INFORMATION ITEM NAME: 0 - absent
///          OUTPUT LABEL: 0 - absent
///        ALLOWED VALUES: 0 - absent
///                INPUTS: 0 - absent
///               OUTPUTS: 1 - single
///           ANNOTATIONS: 0 - absent
/// ```
pub const H_000010: &str = H_000010_DTB;
const H_000010_DTB: &str = include_str!("h_000010.dtb");

/// Horizontal, no information item name, no output label, no allowed values, double inputs, single output, no annotations.
///
/// ```text
/// PREFERRED ORIENTATION: horizontal
/// INFORMATION ITEM NAME: absent
///          OUTPUT LABEL: absent
///        ALLOWED VALUES: absent
///                INPUTS: double
///               OUTPUTS: single
///           ANNOTATIONS: absent
/// ```
pub const H_000210: &str = H_000210_DTB;
const H_000210_DTB: &str = include_str!("h_000210.dtb");

/// Horizontal, no information item name, output label, no allowed values, no inputs, single output, no annotations.
///
/// ```text
/// PREFERRED ORIENTATION: horizontal
/// INFORMATION ITEM NAME: 0 - absent
///          OUTPUT LABEL: 1 - present
///        ALLOWED VALUES: 0 - absent
///                INPUTS: 0 - absent
///               OUTPUTS: 1 - single
///           ANNOTATIONS: 0 - absent
/// ```
pub const H_010010: &str = H_010010_DTB;
const H_010010_DTB: &str = include_str!("h_010010.dtb");

/// Horizontal, no information item name, output label, no allowed values, double inputs, single output, no annotations.
///
/// ```text
/// PREFERRED ORIENTATION: horizontal
/// INFORMATION ITEM NAME: 0 - absent
///          OUTPUT LABEL: 1 - present
///        ALLOWED VALUES: 0 - absent
///                INPUTS: 2 - double
///               OUTPUTS: 1 - single
///           ANNOTATIONS: 0 - absent
/// ```
pub const H_010210: &str = H_010210_DTB;
const H_010210_DTB: &str = include_str!("h_010210.dtb");

/// Horizontal, no information item name, output label, allowed values, double inputs, double outputs, double annotations.
///
/// ```text
/// PREFERRED ORIENTATION: horizontal
/// INFORMATION ITEM NAME: 0 - absent
///          OUTPUT LABEL: 1 - present
///        ALLOWED VALUES: 1 - present
///                INPUTS: 2 - double
///               OUTPUTS: 2 - double
///           ANNOTATIONS: 2 - double
/// ```
pub const H_011222: &str = H_011222_DTB;
const H_011222_DTB: &str = include_str!("h_011222.dtb");

/// Horizontal, information item name, output label, no allowed values, no inputs, single output, no annotations.
///
/// ```text
/// PREFERRED ORIENTATION: horizontal
/// INFORMATION ITEM NAME: 1 - present
///          OUTPUT LABEL: 1 - present
///        ALLOWED VALUES: 0 - absent
///                INPUTS: 0 - absent
///               OUTPUTS: 1 - single
///           ANNOTATIONS: 0 - absent
/// ```
pub const H_110010: &str = H_110010_DTB;
const H_110010_DTB: &str = include_str!("h_110010.dtb");
