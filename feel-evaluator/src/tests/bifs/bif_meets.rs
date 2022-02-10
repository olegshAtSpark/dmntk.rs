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
  te_bool(false, &scope!(), r#"meets([1..10],[10..20])"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"meets([1..10),[10..20])"#, false);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"meets([1..10],(10..20])"#, false);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"meets([1..10],[9..20])"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"meets([1..10],[11..20])"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"meets(range1: [1..10], range2: [10..20])"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"meets(range2: [1..10], range1: [10..20])"#, false);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"meets(range2: [10..20], range1: [1..10])"#, true);
}

#[test]
fn _0009() {
  te_null(false, &scope!(), r#"meets()"#, "expected 2 parameters, actual number of parameters is 0");
}

#[test]
fn _0010() {
  te_null(
    false,
    &scope!(),
    r#"meets(20,[1..20],10)"#,
    "expected 2 parameters, actual number of parameters is 3",
  );
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"meets(range1: [1..5],r2: [3..8])"#, "parameter 'range2' not found");
}

#[test]
fn _0012() {
  te_null(false, &scope!(), r#"meets(r1: [1..5],range2: [3..8])"#, "parameter 'range1' not found");
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"meets(r1: [1..5], r2: [3..8])"#, "parameter 'range1' not found");
}
