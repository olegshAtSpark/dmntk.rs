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
fn test_duration() {
  let scope = &te_scope("{}");
  te_years_and_months_duration(false, scope, r#"duration("P1Y")"#, 1, 0);
  te_years_and_months_duration(false, scope, r#"duration("P4Y")"#, 4, 0);
  te_years_and_months_duration(false, scope, r#"duration("-P999999999Y")"#, -999_999_999, 0);
  te_years_and_months_duration(false, scope, r#"duration("P999999999Y")"#, 999_999_999, 0);
  te_years_and_months_duration(false, scope, r#"duration(from:"P26M")"#, 2, 2);
  te_days_and_time_duration(false, scope, r#"duration("P1D")"#, false, SECONDS_IN_DAY, 0);
  te_days_and_time_duration(false, scope, r#"duration("P4D")"#, false, 4 * SECONDS_IN_DAY, 0);
  te_days_and_time_duration(false, scope, r#"duration("PT2H")"#, false, 2 * SECONDS_IN_HOUR, 0);
  te_days_and_time_duration(false, scope, r#"duration("PT3M")"#, false, 3 * SECONDS_IN_MINUTE, 0);
  te_days_and_time_duration(false, scope, r#"duration("PT4S")"#, false, 4, 0);
  te_days_and_time_duration(false, scope, r#"duration(from:"PT24H")"#, false, SECONDS_IN_DAY, 0);
  te_null(false, scope, "duration(null)", "duration");
  te_null(false, scope, "duration()", "expected 1 parameters, actual number of parameters is 0");
  te_null(false, scope, "duration([])", "duration");
  te_null(false, scope, r#"duration("")"#, "duration");
  te_null(false, scope, "duration(2017)", "duration");
  te_null(false, scope, r#"duration("2012T-12-2511:00:00Z")"#, "duration");
  te_null(false, scope, r#"duration("P")"#, "duration");
  te_null(false, scope, r#"duration("P0")"#, "duration");
  te_null(false, scope, r#"duration("1Y")"#, "duration");
  te_null(false, scope, r#"duration("1D")"#, "duration");
  te_null(false, scope, r#"duration("P1H")"#, "duration");
  te_null(false, scope, r#"duration("P1S")"#, "duration");
}

#[test]
fn test_insert_before() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, r#"insert before([2,3,4,5],1,1)"#, r#"[1,2,3,4,5]"#);
  te_be_value(false, scope, r#"insert before([1,2,3,5],4,4)"#, r#"[1,2,3,4,5]"#);
  te_null(false, scope, r#"insert before([2,3,4,5],0,1)"#, "index is out of range");
  te_null(false, scope, r#"insert before([2,3,4,5],5,1)"#, "index is out of range");
  te_be_value(false, scope, r#"insert before([1,2,3,5],-1,4)"#, r#"[1,2,3,4,5]"#);
  te_be_value(false, scope, r#"insert before([2,3,4,5],-4,1)"#, r#"[1,2,3,4,5]"#);
  te_null(false, scope, r#"insert before([1,2,3,5],0,4)"#, "index is out of range");
  te_null(false, scope, r#"insert before([1,2,3,5],-5,4)"#, "index is out of range");
}

#[test]
fn test_join() {
  let scope = &te_scope(
    r#"{DeptTable:[{manager:"Smith",name:"Sales",number:10},{manager:"Jones",name:"Finance",number:20},{manager:"King",name:"Engineering",number:30}],EmployeeTable:[{deptNum:10,id:"7792",name:"Clark"},{deptNum:10,id:"7934",name:"Miller"},{deptNum:20,id:"7976",name:"Adams"},{deptNum:20,id:"7902",name:"Ford"},{deptNum:30,id:"7900",name:"James"}],LastName:"Clark"}"#,
  );
  te_be_value(false, scope, r#"EmployeeTable[name=LastName]"#, r#"{deptNum:10,id:"7792",name:"Clark"}"#);
  te_value(false, scope, r#"EmployeeTable[name=LastName].deptNum"#, r#"10"#);
  te_value(false, scope, r#"EmployeeTable[name=LastName].deptNum[1]"#, r#"10"#);
  te_be_value(false, scope, r#"DeptTable[number=10]"#, r#"{manager:"Smith",name:"Sales",number:10}"#);
  te_value(false, scope, r#"DeptTable[number=10].manager[1]"#, r#""Smith""#);
  te_value(
    false,
    scope,
    r#"DeptTable[number=EmployeeTable[name=LastName].deptNum[1]].manager[1]"#,
    r#""Smith""#,
  );
}

#[test]
fn test_matches() {
  let scope = &te_scope(r#"{}"#);
  te_bool(false, scope, r#"matches("foobar","^fo*b")"#, true);
  te_bool(false, scope, r#"matches("abracadabra","bra")"#, true);
  te_bool(false, scope, r#"matches("abracadabra","^a.*a$")"#, true);
  te_bool(false, scope, r#"matches("abracadabra","^bra")"#, false);
  let scope = &te_scope(
    r#"{poem:"<poem author=\"Wilhelm Busch\">\nKaum hat dies der Hahn gesehen,\nFängt er auch schon an zu krähen:\nKikeriki! Kikikerikih!!\nTak, tak, tak! - da kommen sie.\n</poem>"}"#,
  );
  te_bool(false, scope, r#"matches(poem, "Kaum.*krähen")"#, false);
  te_bool(false, scope, r#"matches("hello\nworld","hello.*world")"#, false);
  te_bool(false, scope, r#"matches("hello\nworld","hello.*world","s")"#, true);
}

#[test]
fn test_median() {
  let scope = &te_scope("{}");
  te_null(false, scope, "median()", r#"expected 1+ parameters, actual number of parameters is 0"#);
  te_null(false, scope, "median([])", "");
  te_null(false, scope, "median(l:[])", r#"parameter 'list' not found"#);
  te_null(false, scope, "median(l:[1,2,3])", r#"parameter 'list' not found"#);
  te_null(false, scope, "median([true,false])", r#"median"#);
  te_number(false, scope, "median([8, 2, 5, 7])", 6, 0);
  te_number(false, scope, "median(list:[8, 2, 5, 7])", 6, 0);
  te_number(false, scope, "median([8,2,5,3,4])", 4, 0);
  te_number(false, scope, "median(list:[8,2,5,3,4])", 4, 0);
  te_number(false, scope, "median(8,2,5,3,4)", 4, 0);
  te_number(false, scope, "median([8,2,5,3,4.25])", 425, 2);
  te_number(false, scope, "median(list:[8,2,5,3,4.25])", 425, 2);
  te_number(false, scope, "median(8,2,5,3,4.25)", 425, 2);
  te_number(false, scope, "median([6,1,2,3])", 25, 1);
  te_number(false, scope, "median(list:[6,1,2,3])", 25, 1);
  te_number(false, scope, "median(6,1,2,3)", 25, 1);
  te_number(false, scope, "median([2021])", 2021, 0);
  te_number(false, scope, "median(list:[2021])", 2021, 0);
  te_number(false, scope, "median(2021)", 2021, 0);
  te_number(false, scope, "median([1999,2999])", 2499, 0);
  te_number(false, scope, "median(list:[1999,2999])", 2499, 0);
  te_number(false, scope, "median(1999,2999)", 2499, 0);
}

#[test]
fn test_not() {
  let scope = &te_scope("{ On time: true, Too late: false }");
  te_bool(false, scope, "not(true)", false);
  te_bool(false, scope, " not  (  true  ) ", false);
  te_bool(false, scope, "not(false)", true);
  te_bool(false, scope, " not  \n (  \t  false \r  ) \n  ", true);
  te_bool(false, scope, "not(On time)", false);
  te_bool(false, scope, "not(Too late)", true);
}

#[test]
fn test_odd() {
  let scope = &te_scope("{ even number: 20, odd number: 21 }");
  te_bool(false, scope, "odd(2)", false);
  te_bool(false, scope, "odd(-2)", false);
  te_bool(false, scope, "odd(1)", true);
  te_bool(false, scope, "odd(-1)", true);
  te_bool(false, scope, "odd(0)", false);
  te_bool(false, scope, "odd(-0)", false);
  te_null(false, scope, "odd()", r#"expected 1 parameters, actual number of parameters is 0"#);
  te_null(false, scope, "odd(4,4)", r#"expected 1 parameters, actual number of parameters is 2"#);
  te_bool(false, scope, "odd(number:4)", false);
  te_null(false, scope, "odd(n:4)", r#"parameter 'number' not found"#);
  te_bool(false, scope, "odd(even number)", false);
  te_bool(false, scope, "odd(odd number)", true);
}

#[test]
fn test_remove() {
  let scope = &te_scope("{}");
  te_null(false, scope, "remove([1,2,3,4,5],0)", r#"probably index is out of range"#);
  te_null(false, scope, "remove([1,2,3,4,5],6)", r#"probably index is out of range"#);
  te_null(false, scope, "remove([1,2,3,4,5],-6)", r#"probably index is out of range"#);
  te_null(
    false,
    scope,
    "remove([1,2,3,4,5])",
    r#"expected 2 parameters, actual number of parameters is 1"#,
  );
  te_null(false, scope, "remove(6)", r#"expected 2 parameters, actual number of parameters is 1"#);
  te_null(false, scope, "remove([1,2,3,4,5],true)", r#"probably index is out of range"#);
  te_null(false, scope, "remove(l:[1,2,3,4,5],position:1)", r#"parameter 'list' not found"#);
  te_null(false, scope, "remove(list:[1,2,3,4,5],p:1)", r#"parameter 'position' not found"#);
  te_be_value(false, scope, "remove([1,2,3,4,5],1)", "[2,3,4,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],2)", "[1,3,4,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],3)", "[1,2,4,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],4)", "[1,2,3,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],5)", "[1,2,3,4]");
  te_be_value(false, scope, "remove([1,2,3,4,5],-5)", "[2,3,4,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],-4)", "[1,3,4,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],-3)", "[1,2,4,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],-2)", "[1,2,3,5]");
  te_be_value(false, scope, "remove([1,2,3,4,5],-1)", "[1,2,3,4]");
  te_be_value(false, scope, "remove(list:[1,2,3,4,5],position:1)", "[2,3,4,5]");
}

#[test]
fn test_replace() {
  let scope = &te_scope("{}");
  te_string(false, scope, r##"replace("abcd","(ab)|(a)","[1=$1][2=$2]")"##, r##"[1=ab][2=]cd"##);
  te_string(false, scope, r##"replace("a","[b-z]","#")"##, r##"a"##);
  te_string(false, scope, r##"replace("a","[a-z]","#")"##, r##"#"##);
  te_string(false, scope, r##"replace("abc","def","#")"##, r##"abc"##);
  te_string(false, scope, r##"replace("abc","e","#")"##, r##"abc"##);
  te_string(false, scope, r##"replace("foobar","^fo*b*","#")"##, r##"#ar"##);
  te_string(false, scope, r##"replace("abc",".^[d-z]","#")"##, r##"abc"##);
  te_string(false, scope, r##"replace("abracadabra","bra","*")"##, r##"a*cada*"##);
  te_string(false, scope, r##"replace("abracadabra","a.*a","*")"##, r##"*"##);
  te_string(false, scope, r##"replace("abracadabra","a.*?a","*")"##, r##"*c*bra"##);
  te_string(false, scope, r##"replace("abracadabra","a","")"##, r##"brcdbr"##);
  te_string(false, scope, r##"replace("AAAA","A+","b")"##, r##"b"##);
  te_string(false, scope, r##"replace("AAAA","A+?","b")"##, r##"bbbb"##);
  te_string(false, scope, r##"replace("darted","^(.*?)d(.*)$","$1$2")"##, r##"arted"##);
  te_string(false, scope, r##"replace("darted","^(.*?)d(.*)$","$1c$2")"##, r##"carted"##);
  te_string(false, scope, r##"replace("reluctant","r.*?t","X")"##, r##"Xant"##);
  te_string(
    false,
    scope,
    r##"replace("0123456789","(\d{3})(\d{3})(\d{4})","($1) $2-$3")"##,
    r##"(012) 345-6789"##,
  );
  te_string(false, scope, r##"replace("abc","[a-z]","#","")"##, r##"###"##);
  te_string(false, scope, r##"replace("a.b.c.","[a-z]","#","s")"##, r##"#.#.#."##);
  te_string(false, scope, r##"replace("abc","[A-Z]","#","i")"##, r##"###"##);
  te_string(false, scope, r##"replace("abc","[a-z]","#","s")"##, r##"###"##);
  te_string(false, scope, r##"replace("a b c d ","[a-z]","#","x")"##, r##"# # # #"##);
  te_string(false, scope, r##"replace("a b c d ","[a-z]","#")"##, r##"# # # #"##);
  te_string(false, scope, r##"replace("abc",".^[d-z]*","smix")"##, r##"abc"##);
  te_string(false, scope, r##"replace(input:"abc",pattern:"[a-z]",replacement:"#")"##, r##"###"##);
  te_string(false, scope, r##"replace(input:"abc",pattern:"[A-Z]",replacement:"#",flags:"")"##, r##"abc"##);
  te_string(false, scope, r##"replace(input:"abc",pattern:"[A-Z]",replacement:"#",flags:"i")"##, r##"###"##);
  te_string(
    false,
    scope,
    r##"replace(input:"abc",pattern:".^[d-z]*",replacement:"#",flags:"smix")"##,
    r##"abc"##,
  );
  te_string(false, scope, r##"replace("a\b\c","\\","\\\\","q")"##, r##"a\\b\\c"##);
  te_string(false, scope, r##"replace("a/b/c","/","$","q")"##, r##"a$b$c"##);
  te_string(false, scope, r##"replace("abc","[A-Z]","#","all unknown but i")"##, r##"###"##);
  te_string(false, scope, r##"replace(replace("anbncnz","n","\u000A"),"[A-C]\n","u","i")"##, r##"uuuz"##);
  te_string(false, scope, r##"replace("a\u000Ab\u000Ac\u000A","[A-Z]\n","u","i")"##, r##"uuu"##);
}

#[test]
fn test_split() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, r#"split("John Doe","\\s")"#, r#"["John","Doe"]"#);
  te_be_value(false, scope, r#"split("a;b;c;;",";")"#, r#"["a","b","c","",""]"#);
}

#[test]
fn test_sqrt() {
  let scope = &te_scope("{ Area: 144.0 }");
  te_number(false, scope, "sqrt(4)", 2, 0);
  te_null(false, scope, "sqrt(-1)", "?2");
  te_number(false, scope, "sqrt(0)", 0, 0);
  te_number(false, scope, "sqrt(0.0)", 0, 0);
  te_number(false, scope, "sqrt(0.0)", -0, 0);
  te_number(false, scope, "sqrt(-0.0)", 0, 0);
  te_number(false, scope, "sqrt(-0.0)", -0, 0);
  te_null(false, scope, "sqrt()", r#"expected 1 parameters, actual number of parameters is 0"#);
  te_null(false, scope, "sqrt(4,4)", r#"expected 1 parameters, actual number of parameters is 2"#);
  te_null(false, scope, "sqrt(null)", r#"sqrt"#);
  te_null(false, scope, r#"sqrt("4")"#, r#"sqrt"#);
  te_null(false, scope, "sqrt(n:64.0)", r#"parameter 'number' not found"#);
  te_number(false, scope, "sqrt(number:81.0)", 9, 0);
  te_number(false, scope, "sqrt(Area)", 12, 0);
  te_null(false, scope, "sqrt(n  :Area)", r#"parameter 'number' not found"#);
  te_number(false, scope, "sqrt(number : Area)", 12, 0);
}

#[test]
fn test_stddev() {
  let scope = &te_scope("{}");
  te_be_value(false, scope, r#"stddev(2,4,7,5)"#, r#"2.081665999466132735282297706979931"#);
  te_number(false, scope, r#"decimal(stddev(2,4,7,5),13)"#, 20816659994661, 13);
  te_number(false, scope, r#"decimal(stddev(2,4,7,5),9)"#, 2081665999, 9);
  te_be_value(false, scope, r#"stddev([2,4,7,5])"#, r#"2.081665999466132735282297706979931"#);
  te_be_value(false, scope, r#"stddev(list:[2,4,7,5])"#, r#"2.081665999466132735282297706979931"#);
  te_be_value(false, scope, r#"stddev(5,6,8,9)"#, r#"1.825741858350553711523232609336007"#);
}

#[test]
fn test_string_length() {
  let scope = &te_scope("{}");
  te_number(false, scope, r#"string length("engos")"#, 5, 0);
  te_number(false, scope, r#"string length("\u0009")"#, 1, 0);
  te_number(false, scope, r#"string length("\\u0009")"#, 6, 0);
  te_number(false, scope, r#"string length("\U000009")"#, 1, 0);
  te_number(false, scope, r#"string length("\uD83D\uDC0E")"#, 1, 0);
  te_number(false, scope, r#"string length("🐎")"#, 1, 0);
  te_number(false, scope, r#"string length("🐎😀")"#, 2, 0);
}

#[test]
fn test_substring() {
  let scope = &te_scope("{}");
  te_string(false, scope, r#"substring("foobar",3)"#, "obar");
  te_string(false, scope, r#"substring("foobar",3,3)"#, "oba");
  te_string(false, scope, r#"substring("foobar",-2,1)"#, "a");
  te_string(false, scope, r#"substring("foob r",-2,1)"#, " ");
  te_string(false, scope, r#"substring("\U01F40Eab",2)"#, "ab");
}

#[test]
fn test_substring_after() {
  let scope = &te_scope("{}");
  te_string(false, scope, r#"substring after("foobar","ob")"#, "ar");
  te_string(false, scope, r#"substring after("","a")"#, "");
}

#[test]
fn test_substring_before() {
  let scope = &te_scope("{}");
  te_string(false, scope, r#"substring before("foobar","bar")"#, "foo");
  te_string(false, scope, r#"substring before("foobar","xyz")"#, "");
}

#[test]
fn test_years_and_months_duration() {
  let scope = &te_scope("{}");
  te_years_and_months_duration(
    false,
    scope,
    r#"years and months duration(date("2013-08-24"),date and time("2017-12-15T00:59:59"))"#,
    4,
    3,
  );
  te_years_and_months_duration(
    false,
    scope,
    r#"years and months duration(date and time("2017-02-28T23:59:59"),date("2019-07-23"))"#,
    2,
    4,
  );
}
