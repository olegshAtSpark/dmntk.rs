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

use super::*;

#[test]
fn test_0001() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "1*1", 1, 0);
}

#[test]
fn test_0002() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, " 1 * 2 ", 2, 0);
}

#[test]
fn test_0003() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, " 5 *2 *3", 30, 0);
}

#[test]
fn test_0004() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "10*2*5", 100, 0);
}

#[test]
fn test_0005() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "( 1 * 2 ) * ( 3 * 4 )", 24, 0);
}

#[test]
fn test_0006() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "( ( ( 4 * 3 ) ) )", 12, 0);
}

#[test]
fn test_0007() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "(3*2)+(4*5)", 26, 0);
}

#[test]
fn test_0008() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "3*2+4*5", 26, 0);
}

#[test]
fn test_0009() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "3*(2+4)*5", 90, 0);
}

#[test]
fn test_0010() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, ".10 * 30.00", 3, 0);
}

#[test]
fn test_0011() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "1.0*10**3", 1000, 0);
}

#[test]
fn test_0012() {
  let scope = &te_scope("{Monthly Salary:10000}");
  te_number(false, scope, "12 * Monthly Salary", 120000, 0);
}
