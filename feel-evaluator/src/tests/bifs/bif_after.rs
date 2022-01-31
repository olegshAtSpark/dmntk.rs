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

use super::super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_null(false, &scope!(), "after(10,1)", r#"[core::after] under construction: 10 | 1"#);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), "after(point1: 10, point2: 1)", r#"[core::after] under construction: 10 | 1"#);
}

#[test]
fn _0003() {
  te_null(false, &scope!(), "after(point2: 1, point1: 10)", r#"[core::after] under construction: 10 | 1"#);
}

#[test]
fn _0004() {
  te_null(
    false,
    &scope!(),
    "after(point: 11, range: [1..10])",
    r#"[core::after] under construction: 11 | [1..10]"#,
  );
}

#[test]
fn _0005() {
  te_null(
    false,
    &scope!(),
    "after(range: [1..10], point: 11)",
    r#"[core::after] under construction: 11 | [1..10]"#,
  );
}

#[test]
fn _0006() {
  te_null(false, &scope!(), "after(p1: 10, point2: 1)", r#"[named::after] invalid named parameters"#);
}

#[test]
fn _0007() {
  te_null(
    false,
    &scope!(),
    "after(range1: [1..10], range2: [11..20])",
    r#"[core::after] under construction: [1..10] | [11..20]"#,
  );
}

#[test]
fn _0008() {
  te_null(
    false,
    &scope!(),
    "after(range2: [11..20], range1: [1..10])",
    r#"[core::after] under construction: [1..10] | [11..20]"#,
  );
}

#[test]
fn _0009() {
  te_null(false, &scope!(), "after()", r#"expected 2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0010() {
  te_null(false, &scope!(), "after(1,2,3)", r#"expected 2 parameters, actual number of parameters is 3"#);
}
