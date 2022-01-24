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
  let scope = scope!();
  te_bool(false, &scope, "any(true)", true);
}

#[test]
fn _0002() {
  let scope = scope!();
  te_bool(false, &scope, "any(false)", false);
}

#[test]
fn _0003() {
  let scope = scope!();
  te_bool(false, &scope, "any(true,true,true)", true);
}

#[test]
fn _0004() {
  let scope = scope!();
  te_bool(false, &scope, "any(false,false,false)", false);
}

#[test]
fn _0005() {
  let scope = scope!();
  te_bool(false, &scope, "any(false,true,false,false)", true);
}

#[test]
fn _0006() {
  let scope = scope!();
  te_bool(false, &scope, "any(false,true,false,true)", true);
}

#[test]
fn _0007() {
  let scope = scope!();
  te_bool(false, &scope, "any([true])", true);
}

#[test]
fn _0008() {
  let scope = scope!();
  te_bool(false, &scope, "any([false])", false);
}

#[test]
fn _0009() {
  let scope = scope!();
  te_bool(false, &scope, "any([true,true,true])", true);
}

#[test]
fn _0010() {
  let scope = scope!();
  te_bool(false, &scope, "any([false,false,false])", false);
}

#[test]
fn _0011() {
  let scope = scope!();
  te_bool(false, &scope, "any([false,true,false,false])", true);
}

#[test]
fn _0012() {
  let scope = scope!();
  te_bool(false, &scope, "any([false,true,false,true])", true);
}

#[test]
fn _0013() {
  let scope = scope!();
  te_bool(false, &scope, "any([false,null,true])", true);
}

#[test]
fn _0014() {
  let scope = scope!();
  te_bool(false, &scope, "any([])", false);
}

#[test]
fn _0015() {
  let scope = scope!();
  te_null(false, &scope, "any(1)", "");
}

#[test]
fn _0016() {
  let scope = scope!();
  te_null(false, &scope, "any(1,2,3)", "");
}

#[test]
fn _0017() {
  let scope = scope!();
  te_null(false, &scope, "any([123,false])", "");
}

#[test]
fn _0018() {
  let scope = scope!();
  te_null(false, &scope, "any([true,8,false])", "");
}
