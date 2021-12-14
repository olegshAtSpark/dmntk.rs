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

use lazy_static::lazy_static;
use libc::c_char;
use std::ffi::{CStr, CString};

const DEC_INIT_DECQUAD: i32 = 128;
const DEC_QUAD_STRING: usize = 43;
const DEC_QUAD_STRING_BUFFER: [c_char; DEC_QUAD_STRING] = [0; DEC_QUAD_STRING];
const DEC_ROUND_CEILING: u32 = 0;
const DEC_ROUND_FLOOR: u32 = 6;

lazy_static! {
  static ref DEFAULT_CONTEXT: DecContext = dec_context_default();
  pub static ref DEC_ZERO: DecQuad = dec_zero();
  pub static ref DEC_ONE: DecQuad = dec_from_string("1");
  pub static ref DEC_TWO: DecQuad = dec_from_string("2");
  pub static ref DEC_NANO: DecQuad = dec_from_string("1000000000");
}

#[repr(C)]
#[derive(Default, Clone)]
struct DecContext {
  digits: i32,
  emax: i32,
  emin: i32,
  round: u32,
  traps: u32,
  status: u32,
  clamp: u8,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct DecNumber {
  digits: i32,
  exponent: i32,
  bits: u8,
  lsu: [u16; 12],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct DecQuad([u8; 16]);

extern "C" {
  fn decContextDefault(arg1: *mut DecContext, arg2: i32) -> *mut DecContext;
  fn decimal128ToNumber(arg1: *const DecQuad, arg2: *mut DecNumber) -> *mut DecNumber;
  fn decimal128FromNumber(arg1: *mut DecQuad, arg2: *const DecNumber, arg3: *mut DecContext) -> *mut DecQuad;
  fn decNumberSquareRoot(arg1: *mut DecNumber, arg2: *const DecNumber, arg3: *mut DecContext) -> *mut DecNumber;
  fn decNumberLn(arg1: *mut DecNumber, arg2: *const DecNumber, arg3: *mut DecContext) -> *mut DecNumber;
  fn decNumberExp(arg1: *mut DecNumber, arg2: *const DecNumber, arg3: *mut DecContext) -> *mut DecNumber;
  fn decNumberPower(arg1: *mut DecNumber, arg2: *const DecNumber, arg3: *const DecNumber, arg4: *mut DecContext) -> *mut DecNumber;
  fn decNumberScaleB(arg1: *mut DecNumber, arg2: *const DecNumber, arg3: *const DecNumber, arg4: *mut DecContext) -> *mut DecNumber;
  fn decNumberReduce(arg1: *mut DecNumber, arg2: *const DecNumber, arg3: *mut DecContext) -> *mut DecNumber;
  fn decQuadFromString(arg1: *mut DecQuad, arg2: *const c_char, arg3: *mut DecContext) -> *mut DecQuad;
  fn decQuadToString(arg1: *const DecQuad, arg2: *mut c_char) -> *mut c_char;
  fn decQuadAbs(arg1: *mut DecQuad, arg2: *const DecQuad, arg3: *mut DecContext) -> *mut DecQuad;
  fn decQuadToIntegralValue(arg1: *mut DecQuad, arg2: *const DecQuad, arg3: *mut DecContext, arg4: u32) -> *mut DecQuad;
  fn decQuadAdd(arg1: *mut DecQuad, arg2: *const DecQuad, arg3: *const DecQuad, arg4: *mut DecContext) -> *mut DecQuad;
  fn decQuadSubtract(arg1: *mut DecQuad, arg2: *const DecQuad, arg3: *const DecQuad, arg4: *mut DecContext) -> *mut DecQuad;
  fn decQuadMultiply(arg1: *mut DecQuad, arg2: *const DecQuad, arg3: *const DecQuad, arg4: *mut DecContext) -> *mut DecQuad;
  fn decQuadDivide(arg1: *mut DecQuad, arg2: *const DecQuad, arg3: *const DecQuad, arg4: *mut DecContext) -> *mut DecQuad;
  fn decQuadMinus(arg1: *mut DecQuad, arg2: *const DecQuad, arg3: *mut DecContext) -> *mut DecQuad;
  fn decQuadZero(arg1: *mut DecQuad) -> *mut DecQuad;
}

/// Returns the default context for decimal arithmetic.
fn dec_context_default() -> DecContext {
  let mut c = DecContext::default();
  unsafe {
    decContextDefault(&mut c, DEC_INIT_DECQUAD);
  }
  c
}

/// Converts a string into decimal.
pub fn dec_from_string(s: &str) -> DecQuad {
  let c_s = CString::new(s).unwrap();
  let mut value = DecQuad::default();
  unsafe {
    decQuadFromString(&mut value, c_s.as_ptr(), &mut DEFAULT_CONTEXT.clone());
  }
  value
}

/// Converts decimal into string.
pub fn dec_to_string(q: &DecQuad) -> Option<String> {
  let c_s = unsafe {
    let mut buf = DEC_QUAD_STRING_BUFFER;
    decQuadToString(q, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
  };
  c_s.to_str().map_or(None, |s| Some(s.to_string()))
}

/// Calculates the square root.
pub fn dec_sqrt(q: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  let mut n = DecNumber::default();
  unsafe {
    decimal128ToNumber(q, &mut n);
    decNumberSquareRoot(&mut n, &n, &mut DEFAULT_CONTEXT.clone());
    decimal128FromNumber(&mut qr, &n, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

/// Calculates the natural logarithm.
pub fn dec_ln(q: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  let mut n = DecNumber::default();
  unsafe {
    decimal128ToNumber(q, &mut n);
    decNumberLn(&mut n, &n, &mut DEFAULT_CONTEXT.clone());
    decimal128FromNumber(&mut qr, &n, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

/// Calculates the exponent.
pub fn dec_exp(q: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  let mut n = DecNumber::default();
  unsafe {
    decimal128ToNumber(q, &mut n);
    decNumberExp(&mut n, &n, &mut DEFAULT_CONTEXT.clone());
    decimal128FromNumber(&mut qr, &n, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_pow(q1: &DecQuad, q2: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  let mut n1 = DecNumber::default();
  let mut n2 = DecNumber::default();
  let mut nr = DecNumber::default();
  unsafe {
    decimal128ToNumber(q1, &mut n1);
    decimal128ToNumber(q2, &mut n2);
    decNumberPower(&mut nr, &n1, &n2, &mut DEFAULT_CONTEXT.clone());
    decimal128FromNumber(&mut qr, &nr, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

/// Returns absolute value of the number.
pub fn dec_abs(q: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  unsafe {
    decQuadAbs(&mut qr, q, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_floor(q: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  unsafe {
    decQuadToIntegralValue(&mut qr, q, &mut DEFAULT_CONTEXT.clone(), DEC_ROUND_FLOOR);
  }
  qr
}

///
pub fn dec_ceiling(q: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  unsafe {
    decQuadToIntegralValue(&mut qr, q, &mut DEFAULT_CONTEXT.clone(), DEC_ROUND_CEILING);
  }
  qr
}

///
pub fn dec_add(q1: &DecQuad, q2: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  unsafe {
    decQuadAdd(&mut qr, q1, q2, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_subtract(q1: &DecQuad, q2: &DecQuad) -> DecQuad {
  let mut qr: DecQuad = Default::default();
  unsafe {
    decQuadSubtract(&mut qr, q1, q2, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_multiply(q1: &DecQuad, q2: &DecQuad) -> DecQuad {
  let mut qr: DecQuad = Default::default();
  unsafe {
    decQuadMultiply(&mut qr, q1, q2, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_divide(q1: &DecQuad, q2: &DecQuad) -> DecQuad {
  let mut qr: DecQuad = Default::default();
  unsafe {
    decQuadDivide(&mut qr, q1, q2, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_neg(q: &DecQuad) -> DecQuad {
  let mut qr: DecQuad = Default::default();
  unsafe {
    decQuadMinus(&mut qr, q, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_scale(q1: &DecQuad, q2: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  let mut n1 = DecNumber::default();
  let mut n2 = DecNumber::default();
  let mut nr = DecNumber::default();
  unsafe {
    decimal128ToNumber(q1, &mut n1);
    decimal128ToNumber(q2, &mut n2);
    decNumberScaleB(&mut nr, &n1, &n2, &mut DEFAULT_CONTEXT.clone());
    decimal128FromNumber(&mut qr, &nr, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_reduce(q: &DecQuad) -> DecQuad {
  let mut qr = DecQuad::default();
  let mut n = DecNumber::default();
  let mut nr = DecNumber::default();
  unsafe {
    decimal128ToNumber(q, &mut n);
    decNumberReduce(&mut nr, &n, &mut DEFAULT_CONTEXT.clone());
    decimal128FromNumber(&mut qr, &nr, &mut DEFAULT_CONTEXT.clone());
  }
  qr
}

///
pub fn dec_zero() -> DecQuad {
  let mut qr = DecQuad::default();
  unsafe {
    decQuadZero(&mut qr);
  }
  qr
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dec_context_default() {
    let ctx = dec_context_default();
    assert_eq!(34, ctx.digits);
    assert_eq!(6144, ctx.emax);
    assert_eq!(-6143, ctx.emin);
    assert_eq!(3, ctx.round);
    assert_eq!(0, ctx.traps);
    assert_eq!(0, ctx.status);
    assert_eq!(1, ctx.clamp);
  }

  #[test]
  fn test_dec_from_string() {
    let value = dec_from_string("0");
    assert_eq!([0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x08, 0x22], value.0);
    assert_eq!("0", dec_to_string(&value).unwrap());
    let value = dec_from_string("1");
    assert_eq!([0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x08, 0x22], value.0);
    assert_eq!("1", dec_to_string(&value).unwrap())
  }

  #[test]
  fn test_dec_to_string() {
    assert_eq!("1", dec_to_string(&dec_from_string("1")).unwrap());
    assert_eq!("0.000123", dec_to_string(&dec_from_string("0.000123")).unwrap());
    assert_eq!("1000000000.01", dec_to_string(&dec_from_string("1000000000.01")).unwrap());
  }

  #[test]
  fn test_dec_square_root() {
    assert_eq!("0", dec_to_string(&dec_sqrt(&dec_from_string("0"))).unwrap());
    assert_eq!("1", dec_to_string(&dec_sqrt(&dec_from_string("1"))).unwrap());
    assert_eq!("1.414213562373095048801688724209698", dec_to_string(&dec_sqrt(&dec_from_string("2"))).unwrap());
    assert_eq!("2", dec_to_string(&dec_sqrt(&dec_from_string("4"))).unwrap());
    assert_eq!("4", dec_to_string(&dec_sqrt(&dec_from_string("16"))).unwrap());
  }

  #[test]
  fn test_dec_ln() {
    assert_eq!("NaN", dec_to_string(&dec_ln(&dec_from_string("-1"))).unwrap());
    assert_eq!("-Infinity", dec_to_string(&dec_ln(&dec_from_string("0"))).unwrap());
    assert_eq!("0", dec_to_string(&dec_ln(&dec_from_string("1"))).unwrap());
    assert_eq!("1.386294361119890618834464242916353", dec_to_string(&dec_ln(&dec_from_string("4"))).unwrap());
    assert_eq!("2.302585092994045684017991454684364", dec_to_string(&dec_ln(&dec_from_string("10"))).unwrap());
  }

  #[test]
  fn test_dec_exp() {
    assert_eq!("1", dec_to_string(&dec_exp(&dec_from_string("0"))).unwrap());
    assert_eq!("2.718281828459045235360287471352662", dec_to_string(&dec_exp(&dec_from_string("1"))).unwrap());
    assert_eq!("148.4131591025766034211155800405523", dec_to_string(&dec_exp(&dec_from_string("5"))).unwrap());
    assert_eq!("162754.7914190039208080052048984868", dec_to_string(&dec_exp(&dec_from_string("12"))).unwrap());
  }

  #[test]
  fn test_dec_pow() {
    assert_eq!("NaN", dec_to_string(&dec_pow(&dec_from_string("0"), &dec_from_string("0"))).unwrap());
    assert_eq!("1", dec_to_string(&dec_pow(&dec_from_string("1"), &dec_from_string("0"))).unwrap());
    assert_eq!("8", dec_to_string(&dec_pow(&dec_from_string("2"), &dec_from_string("3"))).unwrap());
    assert_eq!(
      "41959.85737359436186095331070746801",
      dec_to_string(&dec_pow(&dec_from_string("12.2384283"), &dec_from_string("4.25"))).unwrap()
    );
    assert_eq!("0.001953125", dec_to_string(&dec_pow(&dec_from_string("8"), &dec_from_string("-3"))).unwrap());
  }

  #[test]
  fn test_dec_abs() {
    assert_eq!("0", dec_to_string(&dec_abs(&dec_from_string("0"))).unwrap());
    assert_eq!("0", dec_to_string(&dec_abs(&dec_from_string("-0"))).unwrap());
    assert_eq!("1", dec_to_string(&dec_abs(&dec_from_string("1"))).unwrap());
    assert_eq!("1", dec_to_string(&dec_abs(&dec_from_string("-1"))).unwrap());
    assert_eq!(
      "12.29308753409583475",
      dec_to_string(&dec_abs(&dec_from_string("12.29308753409583475"))).unwrap()
    );
    assert_eq!(
      "12.29308753409583475",
      dec_to_string(&dec_abs(&dec_from_string("-12.29308753409583475"))).unwrap()
    );
  }

  #[test]
  fn test_dec_floor() {
    assert_eq!("1", dec_to_string(&dec_floor(&dec_from_string("1.5"))).unwrap());
    assert_eq!("-2", dec_to_string(&dec_floor(&dec_from_string("-1.5"))).unwrap());
  }

  #[test]
  fn test_dec_ceiling() {
    assert_eq!("2", dec_to_string(&dec_ceiling(&dec_from_string("1.5"))).unwrap());
    assert_eq!("-1", dec_to_string(&dec_ceiling(&dec_from_string("-1.5"))).unwrap());
  }

  #[test]
  fn test_dec_add() {
    assert_eq!("0", dec_to_string(&dec_add(&dec_from_string("0"), &dec_from_string("0"))).unwrap());
    assert_eq!("1", dec_to_string(&dec_add(&dec_from_string("0"), &dec_from_string("1"))).unwrap());
    assert_eq!("1", dec_to_string(&dec_add(&dec_from_string("1"), &dec_from_string("0"))).unwrap());
    assert_eq!("2", dec_to_string(&dec_add(&dec_from_string("1"), &dec_from_string("1"))).unwrap());
    assert_eq!("0.3", dec_to_string(&dec_add(&dec_from_string("0.1"), &dec_from_string("0.2"))).unwrap());
  }

  #[test]
  fn test_dec_subtract() {
    assert_eq!("0", dec_to_string(&dec_subtract(&dec_from_string("0"), &dec_from_string("0"))).unwrap());
    assert_eq!("-1", dec_to_string(&dec_subtract(&dec_from_string("0"), &dec_from_string("1"))).unwrap());
    assert_eq!("1", dec_to_string(&dec_subtract(&dec_from_string("1"), &dec_from_string("0"))).unwrap());
    assert_eq!("0", dec_to_string(&dec_subtract(&dec_from_string("1"), &dec_from_string("1"))).unwrap());
    assert_eq!("-0.1", dec_to_string(&dec_subtract(&dec_from_string("0.1"), &dec_from_string("0.2"))).unwrap());
  }

  #[test]
  fn test_dec_multiply() {
    assert_eq!("0", dec_to_string(&dec_multiply(&dec_from_string("0"), &dec_from_string("0"))).unwrap());
    assert_eq!("0", dec_to_string(&dec_multiply(&dec_from_string("1"), &dec_from_string("0"))).unwrap());
    assert_eq!("2", dec_to_string(&dec_multiply(&dec_from_string("1"), &dec_from_string("2"))).unwrap());
    assert_eq!(
      "3.0135",
      dec_to_string(&dec_multiply(&dec_from_string("1.23"), &dec_from_string("2.45"))).unwrap()
    );
  }

  #[test]
  fn test_dec_divide() {
    assert_eq!("NaN", dec_to_string(&dec_divide(&dec_from_string("0"), &dec_from_string("0"))).unwrap());
    assert_eq!("3", dec_to_string(&dec_divide(&dec_from_string("6"), &dec_from_string("2"))).unwrap());
    assert_eq!(
      "0.3333333333333333333333333333333333",
      dec_to_string(&dec_divide(&dec_from_string("1"), &dec_from_string("3"))).unwrap()
    );
  }

  #[test]
  fn test_dec_scale() {
    assert_eq!("1.23", dec_to_string(&dec_scale(&dec_from_string("123"), &dec_from_string("-2"))).unwrap());
    assert_eq!("1.23E+4", dec_to_string(&dec_scale(&dec_from_string("123"), &dec_from_string("2"))).unwrap());
  }

  #[test]
  fn test_dec_reduce() {
    assert_eq!("1.23", dec_to_string(&dec_reduce(&dec_from_string("1.23000"))).unwrap());
  }

  #[test]
  fn test_dec_neg() {
    assert_eq!("-1.23", dec_to_string(&dec_neg(&dec_from_string("1.23"))).unwrap());
    assert_eq!("1.23", dec_to_string(&dec_neg(&dec_from_string("-1.23"))).unwrap());
  }
}
