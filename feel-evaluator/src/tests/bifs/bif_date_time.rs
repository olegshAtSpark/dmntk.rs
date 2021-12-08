/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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
  te_date_time_local(
    false,
    scope,
    r#"date and time(date:date("2017-01-01"),time:time("23:59:01"))"#,
    (2017, 1, 1),
    (23, 59, 1, 0),
  );
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_date_time_local(false, scope, r#"date and time("2012-12-24")"#, (2012, 12, 24), (0, 0, 0, 0));
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_date_time_local(false, scope, r#"date and time("2012-12-24T23:59:00")"#, (2012, 12, 24), (23, 59, 0, 0));
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_date_time_local(
    false,
    scope,
    "date    and  \n \t  time  ( \"2012-12-24T23:59:00\"  )   ",
    (2012, 12, 24),
    (23, 59, 0, 0),
  );
}

#[test]
fn _0005() {
  let scope = &te_scope("{}");
  te_date_time_local(false, scope, r#"date and time("-2017-02-28T02:02:02")"#, (-2017, 2, 28), (2, 2, 2, 0));
}

#[test]
fn _0006() {
  let scope = &te_scope("{}");
  te_date_time_local(false, scope, r#"date and time("-2016-01-30T09:05:00")"#, (-2016, 1, 30), (9, 5, 0, 0));
}

#[test]
fn _0007() {
  let scope = &te_scope("{}");
  te_date_time_local(
    false,
    scope,
    r#"date and time("2015-12-31T23:59:59.9999999")"#,
    (2015, 12, 31),
    (23, 59, 59, 999_999_900),
  );
}

#[test]
fn _0008() {
  let scope = &te_scope("{}");
  te_date_time_local(
    false,
    scope,
    r#"date and time("2018-10-01T12:32:59.111111")"#,
    (2018, 10, 1),
    (12, 32, 59, 111_111_000),
  );
}

#[test]
fn _0009() {
  let scope = &te_scope("{}");
  te_date_time_local(
    false,
    scope,
    r#"date and time("2018-10-01T12:32:59.123123123123")"#,
    (2018, 10, 1),
    (12, 32, 59, 123_123_123),
  );
}

#[test]
fn _0010() {
  let scope = &te_scope("{}");
  te_date_time_utc(false, scope, r#"date and time("2012-12-24T23:59:00Z")"#, (2012, 12, 24), (23, 59, 0, 0));
}

#[test]
fn _0011() {
  let scope = &te_scope("{}");
  te_date_time_utc(false, scope, r#"date and time("2012-12-24T23:59:00z")"#, (2012, 12, 24), (23, 59, 0, 0));
}

#[test]
fn _0012() {
  let scope = &te_scope("{}");
  te_date_time_utc(false, scope, r#"date and time("2016-12-24T23:59:00-08:00")"#, (2016, 12, 25), (7, 59, 0, 0));
}

#[test]
fn _0013() {
  let scope = &te_scope("{}");
  te_bool(
    false,
    scope,
    r#"date and time("2012-12-24T23:59:00") in [date and time("2012-12-24T23:59:00")..date and time("2012-12-24T23:59:00")]"#,
    true,
  );
}

#[test]
fn _0014() {
  let scope = &te_scope("{}");
  te_string(false, scope, r#"string(date and time("2016-12-24T23:59:00"))"#, "2016-12-24T23:59:00");
}

#[test]
fn _0015() {
  let scope = &te_scope("{}");
  te_string(false, scope, r#"string(date and time("2016-12-24T23:59:00Z"))"#, "2016-12-24T23:59:00Z");
}

#[test]
fn _0016() {
  let scope = &te_scope("{}");
  te_string(false, scope, r#"string(date and time("2016-12-24T23:59:00z"))"#, "2016-12-24T23:59:00Z");
}

#[test]
fn _0017() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("2016-12-24T23:59:00-08:00"))"#,
    "2016-12-24T23:59:00-08:00",
  );
}

#[test]
fn _0018() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("2016-12-24T23:59:00+02:12"))"#,
    "2016-12-24T23:59:00+02:12",
  );
}

#[test]
fn _0019() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("2016-12-24T23:59:00+14:59"))"#,
    "2016-12-24T23:59:00+14:59",
  );
}

#[test]
fn _0020() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("2016-12-24T23:59:00-14:59"))"#,
    "2016-12-24T23:59:00-14:59",
  );
}

#[test]
fn _0021() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("2016-12-24T23:59:00@Etc/UTC"))"#,
    "2016-12-24T23:59:00@Etc/UTC",
  );
}

#[test]
fn _0022() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("2016-12-24T23:59:00@Europe/Warsaw"))"#,
    "2016-12-24T23:59:00@Europe/Warsaw",
  );
}

#[test]
fn _0023() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("999999999-12-31T23:59:59.999999999@Europe/Paris"))"#,
    "999999999-12-31T23:59:59.999999999@Europe/Paris",
  );
}

#[test]
fn _0024() {
  let scope = &te_scope("{}");
  te_string(
    false,
    scope,
    r#"string(date and time("999999999-12-31T23:59:59.999999999@Europe/Paris"))"#,
    "999999999-12-31T23:59:59.999999999@Europe/Paris",
  );
}

#[test]
fn _0025() {
  let scope = &te_scope(r#"{dateTimeString:"2016-12-24T23:59:00-08:00"}"#);
  te_string(false, scope, r#"string(date and time(dateTimeString))"#, "2016-12-24T23:59:00-08:00");
}

#[test]
fn _0026() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"date and time(null)"#, "date_and_time");
}

#[test]
fn _0027() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"date and time("2012-13-24T23:59:00")"#, "date_and_time");
}

#[test]
fn _0028() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"date and time("2012-12-24T13:65:00")"#, "date_and_time");
}

#[test]
fn _0029() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"date and time("2016-12-24T23:59:00+25:00")"#, "date_and_time");
}

#[test]
fn _0030() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"date and time("2016-12-24T23:59:00-27:30")"#, "date_and_time");
}

#[test]
fn _0031() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"date and time("2017-12-31T13:20:00@xyz/abc")"#, "date_and_time");
}

#[test]
fn _0032() {
  let scope = &te_scope(r#"{}"#);
  te_date_time_offset(
    false,
    scope,
    r#"date and time("2018-12-10T10:30:00.0001+05:00:01")"#,
    (2018, 12, 10),
    (10, 30, 0, 100_000),
    18001,
  )
}

#[test]
fn _0033() {
  let scope = &te_scope(r#"{}"#);
  te_date_time_offset(
    false,
    scope,
    r#"date and time("2018-12-10T10:30:40.001+05:00")"#,
    (2018, 12, 10),
    (10, 30, 40, 1_000_000),
    18000,
  )
}
