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
  te_number(false, &scope!(), r#"number("1",",",".")"#, 1, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), r#"number("1,000.21",",",".")"#, 100021, 2);
}

#[test]
fn _00021() {
  te_number(false, &scope!(), r#"number("1 000.21"," ",".")"#, 100021, 2);
}

#[test]
fn _00022() {
  te_number(false, &scope!(), r#"number("1.000,21",".",",")"#, 100021, 2);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), r#"number("12345",null,null)"#, 12345, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), r#"number("12,345",",",null)"#, 12345, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), r#"number("123,45",null,",")"#, 12345, 2);
}

#[test]
fn _000102() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000.21",".",".")"#,
    r#"[core::number] decimal separator must be different from grouping separator"#,
  );
}

#[test]
fn _000100() {
  te_null(
    false,
    &scope!(),
    r#"number("1$000.21","$",".")"#,
    r#"[core::number] grouping separator must be space, period, comma or null"#,
  );
}

#[test]
fn _000101() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000$21",",","$")"#,
    r#"[core::number] decimal separator must be period, comma or null"#,
  );
}

#[test]
fn _000103() {
  te_null(
    false,
    &scope!(),
    r#"number("123a56",null,null)"#,
    r#"[core::number] FeelNumberError: invalid number literal '123a56'"#,
  );
}

#[test]
fn _000104() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000.21",2,".")"#,
    r#"[core::number] grouping separator must be space, period, comma or null"#,
  );
}

#[test]
fn _000105() {
  te_null(
    false,
    &scope!(),
    r#"number("1,000.21",",",true)"#,
    r#"[core::number] decimal separator must be period, comma or null"#,
  );
}

#[test]
fn _000106() {
  te_null(
    false,
    &scope!(),
    r#"number(1000,null,null)"#,
    r#"[core::number] invalid argument type, expected string, actual type is number"#,
  );
}
