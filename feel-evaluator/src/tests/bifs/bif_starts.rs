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
#[ignore]
fn _0001() {
  te_bool(false, &scope!(), r#"starts(20,[1..20])"#, true);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), r#"starts(20,[1..20])"#, "unimplemented");
}

#[test]
fn _0003() {
  te_null(false, &scope!(), r#"starts(point: 20,range: [1..20])"#, "unimplemented");
}

#[test]
fn _0004() {
  te_null(false, &scope!(), r#"starts(range: [1..20], point: 20)"#, "unimplemented");
}

#[test]
fn _0005() {
  te_null(false, &scope!(), r#"starts(range1: [1..20],range2: [5..20])"#, "unimplemented");
}

#[test]
fn _0006() {
  te_null(false, &scope!(), r#"starts()"#, "expected 2 parameters, actual number of parameters is 0");
}

#[test]
fn _0007() {
  te_null(
    false,
    &scope!(),
    r#"starts(20,[1..20],10)"#,
    "expected 2 parameters, actual number of parameters is 3",
  );
}

#[test]
fn _0008() {
  te_null(false, &scope!(), r#"starts(p: 20, range: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0009() {
  te_null(false, &scope!(), r#"starts(point: 20, r: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"starts(p: 20, r: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0011() {
  te_null(
    false,
    &scope!(),
    r#"starts(range1: [1..20], r2: [1..20])"#,
    "[named::starts] invalid named parameters",
  );
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#"starts(r1: [1..20], range2: [1..20])"#,
    "[named::starts] invalid named parameters",
  );
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"starts(r1: [1..20], r2: [1..20])"#,
    "[named::starts] invalid named parameters",
  );
}
