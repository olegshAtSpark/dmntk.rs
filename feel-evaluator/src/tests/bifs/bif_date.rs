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
  let scope = &scope!();
  te_date(false, scope, r#"date("2012-12-25")"#, 2012, 12, 25);
}

#[test]
fn _0002() {
  let scope = &scope!();
  te_date(false, scope, r#"date("2012-12-25")"#, 2012, 12, 25);
}

#[test]
fn _0003() {
  let scope = &scope!();
  te_date(false, scope, r#"date(2012,12,25)"#, 2012, 12, 25);
}

#[test]
fn _0004() {
  let scope = &scope!();
  te_date(false, scope, r#"date("262143-12-31")"#, 262143, 12, 31);
}

#[test]
fn _0005() {
  let scope = &scope!();
  te_date(false, scope, r#"date("999999999-12-31")"#, 999_999_999, 12, 31);
}

#[test]
fn _0006() {
  let scope = &scope!();
  te_date(false, scope, r#"date(999999999,12,31)"#, 999_999_999, 12, 31);
}

#[test]
fn _0007() {
  let scope = &scope!();
  te_date(false, scope, r#"date("-262144-01-01")"#, -262144, 1, 1);
}

#[test]
fn _0008() {
  let scope = &scope!();
  te_date(false, scope, r#"date("-999999999-01-01")"#, -999_999_999, 1, 1);
}

#[test]
fn _0009() {
  let scope = &scope!();
  te_date(false, scope, r#"date(-999999999,01,01)"#, -999_999_999, 1, 1);
}

#[test]
fn _0010() {
  let scope = &scope!();
  te_date(false, scope, r#"date(date and time("2012-12-25T12:23:18"))"#, 2012, 12, 25);
}

#[test]
fn _0011() {
  let scope = &scope!();
  te_date(false, scope, r#"date(date and time("2012-12-25T12:23:18Z"))"#, 2012, 12, 25);
}

#[test]
fn _0012() {
  let scope = &scope!();
  te_date(false, scope, r#"date(date and time("2012-12-25T12:23:18z"))"#, 2012, 12, 25);
}

#[test]
fn _0013() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2012-12-25") in [date("2012-12-24")..date("2012-12-26")]"#, true);
}

#[test]
fn _0014() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2000-12-25") in [date("2012-12-24")..date("2012-12-26")]"#, false);
}

#[test]
fn _0015() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2020-12-25") in [date("2012-12-24")..date("2012-12-26")]"#, false);
}

#[test]
fn _0016() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2012-12-31") in (date("2012-12-25")..date("2013-02-14"))"#, true);
}

#[test]
fn _0017() {
  let scope = &scope!();
  te_null(false, scope, r#"date("2017-13-10")"#, r#"[core::date] invalid date string '2017-13-10'"#);
}

#[test]
fn _0018() {
  let scope = &scope!();
  te_null(false, scope, r#"date("2017/12/10")"#, r#"[core::date] invalid date string '2017/12/10'"#);
}

#[test]
fn _0019() {
  let scope = &scope!();
  te_null(false, scope, r#"date("2017,12,31")"#, r#"[core::date] invalid date string '2017,12,31'"#);
}

#[test]
fn _0020() {
  let scope = &scope!();
  te_date(false, scope, r#"date("2012-12-25")"#, 2012, 12, 25);
  te_number(false, scope, r#"date("2012-12-25").day"#, 25, 0);
  te_number(false, scope, r#"date("2012-12-25").month"#, 12, 0);
  te_number(false, scope, r#"date("2012-12-25").year"#, 2012, 0);
}

#[test]
fn _0021() {
  let scope = &te_scope(r#"{fromString: "2012-12-25"}"#);
  te_date(false, scope, r#"date(fromString)"#, 2012, 12, 25);
  te_number(false, scope, r#"date(fromString).day"#, 25, 0);
  te_number(false, scope, r#"date(fromString).month"#, 12, 0);
  te_number(false, scope, r#"date(fromString).year"#, 2012, 0);
}

#[test]
fn _0022() {
  te_null(
    false,
    &scope!(),
    r#"date(10)"#,
    r#"[core::date] invalid argument type, expected string or date and time, actual type is number"#,
  );
}

#[test]
fn _0023() {
  te_null(
    false,
    &scope!(),
    r#"date(date("2012-12-25"))"#,
    r#"[core::date] invalid argument type, expected string or date and time, actual type is date"#,
  );
}
