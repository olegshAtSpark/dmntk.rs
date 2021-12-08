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
fn _0001() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of number", true);
}

#[test]
fn _0002() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of string", false);
}

#[test]
fn _0003() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of boolean", false);
}

#[test]
fn _0004() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of date", false);
}

#[test]
fn _0005() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of time", false);
}

#[test]
fn _0006() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of date and time", false);
}

#[test]
fn _0007() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of days and time duration", false);
}

#[test]
fn _0008() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of years and months duration", false);
}

#[test]
fn _0009() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of Any", true);
}

#[test]
fn _0010() {
  let scope = &te_scope(r#"{Order:4.0}"#);
  te_bool(false, scope, "Order instance of Null", false);
}

#[test]
fn _0011() {
  let scope = &te_scope(r#"{Order:null}"#);
  te_bool(false, scope, "Order instance of Null", true);
}

#[test]
fn _0012() {
  let scope = &te_scope(r#"{Customer:"Business"}"#);
  te_bool(false, scope, "Customer instance of string", true);
}

#[test]
fn _0013() {
  let scope = &te_scope(r#"{Customer:"Business"}"#);
  te_bool(false, scope, "Customer instance of number", false);
}

#[test]
fn _0014() {
  let scope = &te_scope(r#"{Customer:"Business"}"#);
  te_bool(false, scope, "Customer instance of boolean", false);
}

#[test]
fn _0015() {
  let scope = &te_scope(r#"{Delivery status:true}"#);
  te_bool(false, scope, "Delivery status instance of boolean", true);
}

#[test]
fn _0016() {
  let scope = &te_scope(r#"{Delivery status:true}"#);
  te_bool(false, scope, "Delivery status instance of number", false);
}

#[test]
fn _0017() {
  let scope = &te_scope(r#"{Delivery status:true}"#);
  te_bool(false, scope, "Delivery status instance of string", false);
}

#[test]
fn _0018() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<number>", true);
}

#[test]
fn _0019() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<string>", false);
}

#[test]
fn _0020() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<boolean>", false);
}

#[test]
fn _0021() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<date>", false);
}

#[test]
fn _0022() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<time>", false);
}

#[test]
fn _0023() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<date and time>", false);
}

#[test]
fn _0024() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<years and months duration>", false);
}

#[test]
fn _0025() {
  let scope = &te_scope(r#"{Orders:[1,2,3,4]}"#);
  te_bool(false, scope, "Orders instance of list<days and time duration>", false);
}

#[test]
fn _0026() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<number>", true);
}

#[test]
fn _0027() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<string>", false);
}

#[test]
fn _0028() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<boolean>", false);
}

#[test]
fn _0029() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<date>", false);
}

#[test]
fn _0030() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<time>", false);
}

#[test]
fn _0031() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<date and time>", false);
}

#[test]
fn _0032() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<years and months duration>", false);
}

#[test]
fn _0033() {
  let scope = &te_scope(r#"{Items:[1..10]}"#);
  te_bool(false, scope, "Items instance of range<days and time duration>", false);
}

#[test]
fn _0034() {
  let scope = &te_scope(r#"{Person:{name:"John",age:49}}"#);
  te_bool(false, scope, "Person instance of context<name:string>", false);
}

#[test]
fn _0035() {
  let scope = &te_scope(r#"{Person:{name:"John",age:49}}"#);
  te_bool(false, scope, "Person instance of context<name:string,age:number,car:string>", false);
}

#[test]
fn _0036() {
  let scope = &te_scope(r#"{Person:{name:"John",age:49}}"#);
  te_bool(false, scope, "Person instance of context<name:string,a:number>", false);
}

#[test]
fn _0037() {
  let scope = &te_scope(r#"{Person:{name:"John",age:49}}"#);
  te_bool(false, scope, "Person instance of context<n:string,age:number>", false);
}

#[test]
fn _0038() {
  let scope = &te_scope(r#"{Person:{name:"John",age:49}}"#);
  te_bool(false, scope, "Person instance of context<name:string,age:number>", true);
}

#[test]
fn _0039() {
  let scope = &te_scope(r#"{Person:{name:"John",age:49}}"#);
  te_bool(false, scope, "Person instance of function<string>->string", false);
}

#[test]
fn _0040() {
  let scope = &te_scope(r#"{Power: 25.5,engine:{power:280.5}}"#);
  te_bool(false, scope, "Power instance of engine.power", true);
}

//TODO Add more tests with qualified name to different types
