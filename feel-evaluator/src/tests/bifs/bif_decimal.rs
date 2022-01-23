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

#[test]
fn _0001() {
  let scope = &te_scope("{}");
  te_number(false, scope, "decimal(1,2)", 100, 2);
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_number(false, scope, "decimal(1/3,2)", 330, 3);
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_number(false, scope, "decimal(0.505,2)", 50, 2);
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_number(false, scope, "decimal(0.515,2)", 52, 2);
}

#[test]
fn _0005() {
  let scope = &te_scope("{}");
  te_number(false, scope, "decimal(1/3, 2.5)", 33, 2);
}

#[test]
fn _0006() {
  let scope = &te_scope("{}");
  te_null(false, scope, "decimal(1/3, 6177)", "[core::decimal] scale is out of range: 6177");
}

#[test]
fn _0007() {
  let scope = &te_scope("{}");
  te_null(false, scope, "decimal(1/3, -6112)", "[core::decimal] scale is out of range: -6112");
}

#[test]
fn _0008() {
  let scope = &te_scope("{}");
  te_null(
    false,
    scope,
    r#"decimal(1/3, "scale")"#,
    r#"[core::decimal] scale value is not a number: "scale""#,
  );
}

#[test]
fn _0009() {
  let scope = &te_scope("{}");
  te_null(
    false,
    scope,
    r#"decimal("number", 6)"#,
    r#"[core::decimal] number value is not a number: "number""#,
  );
}
