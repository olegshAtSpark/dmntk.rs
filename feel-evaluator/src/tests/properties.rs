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

use super::*;

#[test]
fn test_properties() {
  let scope = &te_scope("{}");
  te_value(false, scope, r#"@"P1Y0M""#, r#"duration("P1Y")"#);
  te_number(false, scope, r#"date("2021-02-10").year"#, 2021, 0);
  te_number(false, scope, r#"date("2021-02-10").month"#, 2, 0);
  te_number(false, scope, r#"date("2021-02-10").day"#, 10, 0);
  te_number(false, scope, r#"date("2021-02-10").weekday"#, 3, 0);
  te_number(false, scope, r#"date and time("2018-12-10T10:30:01").year"#, 2018, 0);
  te_number(false, scope, r#"date and time("2018-12-10T10:30:01").month"#, 12, 0);
  te_number(false, scope, r#"date and time("2018-12-10T10:30:01").day"#, 10, 0);
  te_number(false, scope, r#"date and time("2018-12-10T11:30:01").hour"#, 11, 0);
  te_number(false, scope, r#"date and time("2018-12-10T11:30:01").hour"#, 11, 0);
  te_number(false, scope, r#"date and time("2018-12-10T11:30:01").minute"#, 30, 0);
  te_number(false, scope, r#"date and time("2018-12-10T11:30:01").second"#, 1, 0);
  te_number(false, scope, r#"date and time("2021-02-10").hour"#, 0, 0);
  te_number(false, scope, r#"date and time("2021-02-10").minute"#, 0, 0);
  te_number(false, scope, r#"date and time("2021-02-10").second"#, 0, 0);
  te_value(false, scope, r#"date and time("2018-12-10T10:30:00+05:00").time offset"#, r#"@"PT5H""#);
  te_null(false, scope, r#"date and time("2018-12-10T10:30:00").time offset"#, "aaaa");
  te_string(false, scope, r#"date and time("2018-12-10T10:30:00@Etc/UTC").timezone"#, r#"Etc/UTC"#);
  te_null(false, scope, r#"date and time("2018-12-10T10:30:00").timezone"#, "bbb");
  te_number(false, scope, r#"time("08:45:27").hour"#, 8, 0);
  te_number(false, scope, r#"time("08:45:27").minute"#, 45, 0);
  te_number(false, scope, r#"time("08:45:27").second"#, 27, 0);
  te_value(false, scope, r#"time("08:45:27-05:00").time offset"#, r#"@"-PT5H""#);
  te_null(false, scope, r#"time("08:45:27").time offset"#, "ccc");
  te_string(false, scope, r#"time("08:45:27@Etc/UTC").timezone"#, r#"Etc/UTC"#);
  te_null(false, scope, r#"time("08:45:27").timezone"#, "ddd");
  te_number(false, scope, r#"duration("P1Y2M").years"#, 1, 0);
  te_number(false, scope, r#"duration("P2M").years"#, 0, 0);
  te_number(false, scope, r#"duration("P2M").months"#, 2, 0);
  te_number(false, scope, r#"duration("P1Y").months"#, 0, 0);
  te_null(false, scope, r#"duration("P1Y").days"#, "no such property in years and months duration");
  te_null(false, scope, r#"duration("P1Y").hours"#, "no such property in years and months duration");
  te_null(false, scope, r#"duration("P1Y").minutes"#, "no such property in years and months duration");
  te_null(false, scope, r#"duration("P1Y").seconds"#, "no such property in years and months duration");
  te_null(false, scope, r#"duration("P1D").years"#, "no such property in days and time duration");
  te_null(false, scope, r#"duration("P1D").months"#, "no such property in days and time duration");
  te_number(false, scope, r#"duration("P1D").days"#, 1, 0);
  te_number(false, scope, r#"duration("PT2H").days"#, 0, 0);
  te_number(false, scope, r#"duration("PT2H").hours"#, 2, 0);
  te_number(false, scope, r#"duration("P1D").hours"#, 0, 0);
  te_number(false, scope, r#"duration("P1DT3H").hours"#, 3, 0);
  te_number(false, scope, r#"duration("PT2M").minutes"#, 2, 0);
  te_number(false, scope, r#"duration("P1D").minutes"#, 0, 0);
  te_number(false, scope, r#"duration("PT5S").seconds"#, 5, 0);
  te_number(false, scope, r#"duration("P1D").seconds"#, 0, 0);
  te_number(false, scope, r#"duration("P3DT15H47M13S").days"#, 3, 0);
  te_number(false, scope, r#"duration("P3DT15H47M13S").hours"#, 15, 0);
  te_number(false, scope, r#"duration("P3DT15H47M13S").minutes"#, 47, 0);
  te_number(false, scope, r#"duration("P3DT15H47M13S").seconds"#, 13, 0);
}
