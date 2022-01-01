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

//! Simple examples of decision tables.

pub const EXAMPLE_0001_DTB: &str = r#"
  ┌───┬──────────┬───────╥──────┐
  │ U │ Customer │ Order ║      │
  ╞═══╪══════════╪═══════╬══════╡
  │ 1 │ Business │  <10  ║ 0.10 │
  ├───┼──────────┼───────╫──────┤
  │ 2 │ Business │ >=10  ║ 0.15 │
  ├───┼──────────┼───────╫──────┤
  │ 3 │ Private  │   -   ║ 0.05 │
  └───┴──────────┴───────╨──────┘
"#;

pub const EXAMPLE_0001_CTX: &str = r#"
{
  Customer: "Business",
     Order:  10.00
}
"#;

pub const EXAMPLE_0002_DTB: &str = r#"
  ┌───────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║ good │  bad   │     -    │  good  │ bad  │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║ Low  │ Medium │  Medium  │ Medium │ High │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      ║  10  │    7   │     6    │    4   │  0   │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 7}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 4}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

pub const EXAMPLE_0002_CTX: &str = r#"
{
    Applicant age: 24,
  Medical history: "good"
}
"#;
