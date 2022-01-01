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
  te_time(false, scope, r#"time(11,59,45,null)"#, FeelTime::local(11, 59, 45, 0));
}

#[test]
fn _0002() {
  let scope = &te_scope(r#"{Hours:12,Minutes:59,Seconds:1.3,Timezone:@"-PT1H"}"#);
  te_time(
    false,
    scope,
    r#"time(Hours,Minutes,Seconds,Timezone)"#,
    FeelTime::new_hmso_opt(12, 59, 1, 300_000_000, -3600).unwrap(),
  );
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time("23:59:00")"#, FeelTime::local(23, 59, 0, 0));
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"  time  (  "23:59:00"         )   "#, FeelTime::local(23, 59, 0, 0));
}

#[test]
fn _0005() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time("23:59:00Z")"#, FeelTime::utc(23, 59, 0, 0));
}

#[test]
fn _0006() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time("23:59:00z")"#, FeelTime::utc(23, 59, 0, 0));
}

#[test]
fn _0007() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time("11:22:33-00:00")"#, FeelTime::utc(11, 22, 33, 0));
}

#[test]
fn _0008() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time("11:22:33+00:00")"#, FeelTime::utc(11, 22, 33, 0));
}

#[test]
fn _0009() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time(time("11:00:00"))"#, FeelTime::local(11, 0, 0, 0));
}

#[test]
fn _0010() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time(date and time("2019-12-06T18:34:12"))"#, FeelTime::local(18, 34, 12, 0));
}

#[test]
fn _0011() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time(date and time("2019-12-06T11:00:00Z"))"#, FeelTime::utc(11, 0, 0, 0));
}

#[test]
fn _0012() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time(date and time("2019-12-06T11:00:00z"))"#, FeelTime::utc(11, 0, 0, 0));
}

#[test]
fn _0013() {
  let scope = &te_scope("{}");
  te_time(false, scope, r#"time(date("2019-12-06"))"#, FeelTime::utc(0, 0, 0, 0));
}

#[test]
fn _0014() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"time("12:21:12") in [time("12:21:12")..time("12:21:12")]"#, true);
}

#[test]
fn _0015() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"time("12:21:11") in [time("12:21:12")..time("12:21:12")]"#, false);
}

#[test]
fn _0016() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"time("12:21:13") in [time("12:21:12")..time("12:21:12")]"#, false);
}

#[test]
fn _0017() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"time("12:21:12") in (time("12:21:11")..time("12:21:13"))"#, true);
}

#[test]
fn _0018() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time("22:63:12")"#, "time_1");
}

#[test]
fn _0019() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time("22:10:12+15:00")"#, "time_1");
}

#[test]
fn _0020() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time("22:10:12-15:00")"#, "time_1");
}

#[test]
fn _0021() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(24,59,45,null)"#, "time_4");
}

#[test]
fn _0022() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(23,60,45,null)"#, "time_4");
}

#[test]
fn _0023() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(24,59,45,null)"#, "time_4");
}

#[test]
fn _0024() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(23,60,45,null)"#, "time_4");
}

#[test]
fn _0025() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(23,59,60,null)"#, "time_4");
}

#[test]
fn _0026() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(-12,12,12,null)"#, "time_4");
}

#[test]
fn _0027() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(12,-12,12,null)"#, "time_4");
}

#[test]
fn _0028() {
  let scope = &te_scope("{}");
  te_null(false, scope, r#"time(12,12,-12,null)"#, "time_4");
}
