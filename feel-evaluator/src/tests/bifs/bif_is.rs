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
  te_bool(false, &scope!(), r#"is(date(2019,7,12),date("2019-07-12"))"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"is(value1:date(2019,7,12),value2:date("2019-07-12"))"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"is(date(2019,7,11),date("2019-07-12"))"#, false);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"is(value1:date(2019,7,11),value2:date("2019-07-12"))"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"is(time(12,13,14),time("12:13:14"))"#, true);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"is(value1:time(12,13,14),value2:time("12:13:14"))"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"is(time(12,13,15),time("12:13:14"))"#, false);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"is(value1:time(12,13,15),value2:time("12:13:14"))"#, false);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"is(time("23:00:50z"),time("23:00:50"))"#, false);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"is(time("23:00:50z"),time("23:00:50Z"))"#, true);
}

#[test]
#[ignore]
fn _0011() {
  //TODO It is not precisely defined if `z` offset, +00:00 offset and Etc/UTC should be treated equally. Time is the same, but specification says that these values should be treated not equal. Check it.
  te_bool(false, &scope!(), r#"is(time("23:00:50z"),time("23:00:50@Etc/UTC"))"#, true);
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#"is(v1:time(12,13,15),value2:time("12:13:14"))"#,
    "parameter 'value1' not found",
  );
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"is(value1:time(12,13,15),v2:time("12:13:14"))"#,
    "parameter 'value2' not found",
  );
}

#[test]
fn _0014() {
  te_null(false, &scope!(), r#"is()"#, "expected 2 parameters, actual number of parameters is 0");
}

#[test]
fn _0015() {
  te_null(
    false,
    &scope!(),
    r#"is(time(12,13,15))"#,
    "expected 2 parameters, actual number of parameters is 1",
  );
}

#[test]
fn _0016() {
  te_null(
    false,
    &scope!(),
    r#"is(time(12,13,15),time(12,13,15),time(12,13,15))"#,
    "expected 2 parameters, actual number of parameters is 3",
  );
}

#[test]
fn _0017() {
  te_null(
    false,
    &scope!(),
    r#"is(10,time(12,13,15))"#,
    "[core::is] invalid argument type, expected date or time, actual type is number",
  );
}

#[test]
fn _0018() {
  te_null(
    false,
    &scope!(),
    r#"is(time(12,13,15), 10)"#,
    "[core::is] invalid argument type, expected time, actual type is number",
  );
}

#[test]
fn _0019() {
  te_null(
    false,
    &scope!(),
    r#"is(date(2012,11,15), 10)"#,
    "[core::is] invalid argument type, expected date, actual type is number",
  );
}
