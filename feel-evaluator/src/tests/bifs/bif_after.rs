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
  te_bool(false, &scope!(), r#"after(10,1)"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"after(date("2022-01-01"),date("2021-12-31"))"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"after(1,10)"#, false);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"after(date("2021-12-31"),date("2022-01-01"))"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"after(point1:10,point2:1)"#, true);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"after(point1:date("2022-01-01"),point2:date("2021-12-31"))"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"after(point1:1,point2:10)"#, false);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"after(point1:date("2021-12-31"),point2:date("2022-01-01"))"#, false);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"after(point2:1,point1:10)"#, true);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"after(point2:date("2021-12-31"),point1:date("2022-01-01"))"#, true);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#"after(point2: 10, point1: 1)"#, false);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"after(point2:date("2022-01-01"),point1:date("2021-12-31"))"#, false);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#"after(11,[1..10])"#, true);
}

#[test]
fn _0014() {
  te_bool(false, &scope!(), r#"after(date("2021-06-01"),[date("2021-01-01")..date("2021-05-30")])"#, true);
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#"after([1..10],11)"#, false);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#"after([date("2021-01-01")..date("2021-05-30")],date("2021-06-01"))"#, false);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#"after(point:11,range:[1..10])"#, true);
}

#[test]
fn _0018() {
  te_bool(
    false,
    &scope!(),
    r#"after(point:date("2021-06-01"),range:[date("2021-01-01")..date("2021-05-30")])"#,
    true,
  );
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#"after(range:[1..10],point:11)"#, false);
}

#[test]
fn _0020() {
  te_bool(
    false,
    &scope!(),
    r#"after(range:[date("2021-01-01")..date("2021-05-30")],point:date("2021-06-01"))"#,
    false,
  );
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#"after(range1:[11..20],range2:[1..10])"#, true);
}

#[test]
fn _0022() {
  te_bool(
    false,
    &scope!(),
    r#"after(range1:[date("2021-01-06")..date("2021-01-23")],range2:[date("2020-11-18")..date("2021-01-05")])"#,
    true,
  );
}

#[test]
fn _0023() {
  te_bool(false, &scope!(), r#"after(range1:[1..10],range2:[11..20])"#, false);
}

#[test]
fn _0024() {
  te_bool(
    false,
    &scope!(),
    r#"after(range1:[date("2020-11-18")..date("2021-01-05")],range2:[date("2021-01-06")..date("2021-01-23")])"#,
    false,
  );
}

#[test]
fn _0025() {
  te_bool(false, &scope!(), r#"after(range2:[1..10],range1:[11..20])"#, true);
}

#[test]
fn _0026() {
  te_bool(
    false,
    &scope!(),
    r#"after(range2:[date("2020-11-18")..date("2021-01-05")],range1:[date("2021-01-06")..date("2021-01-23")])"#,
    true,
  );
}

#[test]
fn _0027() {
  te_bool(false, &scope!(), r#"after(range2:[11..20],range1:[1..10])"#, false);
}

#[test]
fn _0028() {
  te_bool(
    false,
    &scope!(),
    r#"after(range2:[date("2021-01-06")..date("2021-01-23")],range1:[date("2020-11-18")..date("2021-01-05")])"#,
    false,
  );
}

#[test]
fn _0029() {
  te_null(false, &scope!(), r#"after(p1:10,point2:1)"#, r#"[named::after] invalid named parameters"#);
}

#[test]
fn _0030() {
  te_null(false, &scope!(), r#"after()"#, r#"expected 2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0031() {
  te_null(
    false,
    &scope!(),
    r#"after(1,2,3)"#,
    r#"expected 2 parameters, actual number of parameters is 3"#,
  );
}

#[test]
fn _0032() {
  te_null(
    false,
    &scope!(),
    r#"after(null,null)"#,
    r#"[core::before] invalid argument type, expected scalar or range of scalars, actual type is Null"#,
  );
}

#[test]
#[ignore]
fn _0033() {
  te_null(
    false,
    &scope!(),
    r#"after(1,null)"#,
    r#"[core::before] invalid argument type, expected scalar or range of scalars, actual type is Null"#,
  );
}

#[test]
fn _0034() {
  te_null(
    false,
    &scope!(),
    r#"after(null,1)"#,
    r#"[core::before] invalid argument type, expected scalar or range of scalars, actual type is Null"#,
  );
}
