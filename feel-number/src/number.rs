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

//! `FEEL` number type.

use crate::dec::*;
use dmntk_common::Jsonify;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

/// FEEL number.
#[derive(Debug, Copy, Clone)]
pub struct FeelNumber(DecQuad);

impl FeelNumber {
  /// Creates a new [FeelNumber] from number and scale.
  pub fn new(num: i128, scale: u32) -> Self {
    let n = dec_from_string(&format!("{}", num));
    let s = dec_from_string(&format!("{}", -(scale as i64)));
    Self(dec_scale_b(&n, &s))
  }
  /// Creates a new [FeelNumber] from integer value.
  pub fn int(num: i128) -> Self {
    Self(dec_scale_b(&dec_from_string(&format!("{}", num)), &DEC_ZERO))
  }
  ///
  pub fn zero() -> Self {
    Self(*DEC_ZERO)
  }
  ///
  pub fn one() -> Self {
    Self(*DEC_ONE)
  }
  ///
  pub fn two() -> Self {
    Self(*DEC_TWO)
  }
  ///
  pub fn nano() -> Self {
    Self(*DEC_NANO)
  }
  ///
  pub fn abs(&self) -> Self {
    Self(dec_abs(&self.0))
  }
  ///
  pub fn ceiling(&self) -> Self {
    Self(dec_reduce(&dec_ceiling(&self.0)))
  }
  ///
  pub fn even(&self) -> bool {
    if dec_is_integer(&self.0) {
      if dec_is_zero(&dec_remainder(&self.0, &DEC_TWO)) {
        return true;
      }
    }
    false
  }
  ///
  pub fn exp(&self) -> Self {
    Self(dec_exp(&self.0))
  }
  ///
  pub fn floor(&self) -> Self {
    Self(dec_reduce(&dec_floor(&self.0)))
  }
  ///
  pub fn is_integer(&self) -> bool {
    dec_is_integer(&self.0)
  }
  ///
  pub fn is_positive(&self) -> bool {
    dec_is_positive(&self.0)
  }
  ///
  pub fn ln(&self) -> Option<Self> {
    let n = dec_ln(&self.0);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
  ///
  pub fn odd(&self) -> bool {
    if dec_is_integer(&self.0) {
      if !dec_is_zero(&dec_remainder(&self.0, &DEC_TWO)) {
        return true;
      }
    }
    false
  }
  ///
  pub fn pow(&self, rhs: &FeelNumber) -> Option<Self> {
    let n = dec_power(&self.0, &rhs.0);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
  ///
  pub fn round(&self, rhs: FeelNumber) -> Self {
    Self(dec_rescale(&self.0, &dec_minus(&rhs.0)))
  }
  ///
  pub fn sqrt(&self) -> Option<Self> {
    let n = dec_square_root(&self.0);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
  ///
  pub fn square(&self) -> Option<Self> {
    let n = dec_power(&self.0, &DEC_TWO);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
}

impl PartialEq<FeelNumber> for FeelNumber {
  fn eq(&self, rhs: &Self) -> bool {
    dec_is_zero(&dec_compare_total(&self.0, &rhs.0))
  }
}

impl PartialOrd for FeelNumber {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let flag = dec_compare_total(&self.0, &rhs.0);
    if dec_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    return Some(Ordering::Less);
  }
}

impl std::ops::Add<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_add(&self.0, &rhs.0)))
  }
}

impl std::ops::AddAssign<FeelNumber> for FeelNumber {
  ///
  fn add_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_add(&self.0, &rhs.0));
  }
}

impl std::ops::Sub<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_subtract(&self.0, &rhs.0)))
  }
}

impl std::ops::SubAssign<FeelNumber> for FeelNumber {
  ///
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_subtract(&self.0, &rhs.0));
  }
}

impl std::ops::Mul<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_multiply(&self.0, &rhs.0)))
  }
}

impl std::ops::MulAssign<FeelNumber> for FeelNumber {
  ///
  fn mul_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_multiply(&self.0, &rhs.0));
  }
}

impl std::ops::Div<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_divide(&self.0, &rhs.0)))
  }
}

impl std::ops::DivAssign<FeelNumber> for FeelNumber {
  ///
  fn div_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_divide(&self.0, &rhs.0));
  }
}

impl std::ops::Rem<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn rem(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_subtract(
      &self.0,
      &dec_multiply(&rhs.0, &dec_floor(&dec_divide(&self.0, &rhs.0))),
    )))
  }
}

impl std::ops::Neg for FeelNumber {
  type Output = Self;
  ///
  fn neg(self) -> Self::Output {
    Self(dec_minus(&self.0))
  }
}

impl std::ops::RemAssign<FeelNumber> for FeelNumber {
  ///
  fn rem_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_subtract(&self.0, &dec_multiply(&rhs.0, &dec_floor(&dec_divide(&self.0, &rhs.0)))));
  }
}

impl std::fmt::Display for FeelNumber {
  /// Converts [FeelNumber] to its textual representation.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(s) = dec_to_string(&self.0) {
      write!(f, "{}", scientific_to_plain(s))
    } else {
      write!(f, "invalid number")
    }
  }
}

impl Jsonify for FeelNumber {
  /// Converts [FeelNumber] to its `JSON` representation.
  fn jsonify(&self) -> String {
    if let Some(s) = dec_to_string(&self.0) {
      scientific_to_plain(s)
    } else {
      "Null".to_string()
    }
  }
}

/// Converts a string in scientific notation into digits without exponent.
fn scientific_to_plain(s: String) -> String {
  if s.contains("E+") {
    let mut split1 = s.split("E+");
    let before_exponent = split1.next().unwrap();
    let after_exponent = split1.next().unwrap();
    let exponent_digits = usize::from_str(after_exponent).unwrap();
    if before_exponent.contains('.') {
      let mut split2 = before_exponent.split('.');
      let before_decimal = split2.next().unwrap();
      let after_decimal = split2.next().unwrap();
      let zeroes = (0..(exponent_digits - after_decimal.len())).map(|_| "0").collect::<String>();
      format!("{}{}{}", before_decimal, after_decimal, zeroes)
    } else {
      let zeroes = (0..exponent_digits).map(|_| "0").collect::<String>();
      format!("{}{}", before_exponent, zeroes)
    }
  } else if s.contains("E-") {
    let mut split1 = s.split("E-");
    let before_exponent = split1.next().unwrap();
    let after_exponent = split1.next().unwrap();
    let exponent_digits = usize::from_str(after_exponent).unwrap();
    if before_exponent.contains('.') {
      let mut split2 = before_exponent.split('.');
      let before_decimal = split2.next().unwrap();
      let after_decimal = split2.next().unwrap();
      let zeroes = (1..exponent_digits).map(|_| "0").collect::<String>();
      format!("0.{}{}{}", zeroes, before_decimal, after_decimal)
    } else {
      let zeroes = (1..exponent_digits).map(|_| "0").collect::<String>();
      format!("0.{}{}", zeroes, before_exponent)
    }
  } else {
    s
  }
}

// mod errors {
//   use dmntk_common::DmntkError;
//
//   /// `FEEL` number errors.
//   #[derive(Error, Debug)]
//   enum FeelNumberError {
//     #[error("invalid number")]
//     InvalidNumber,
//   }
//
//   impl From<FeelNumberError> for DmntkError {
//     fn from(e: FeelNumberError) -> Self {
//       DmntkError::new("FeelNumberError", &e.to_string())
//     }
//   }
//
//   pub fn err_invalid_number() -> DmntkError {
//     FeelNumberError::InvalidNumber.into()
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stringify() {
    assert_eq!("49", FeelNumber::new(49, 0).to_string());
    assert_eq!("49", FeelNumber::int(49).to_string());
    assert_eq!("49.0", FeelNumber::new(490, 1).to_string());
    assert_eq!("4900", FeelNumber::new(4900, 0).to_string());
    assert_eq!("50", FeelNumber::new(50, 0).to_string());
    assert_eq!("50", FeelNumber::int(50).to_string());
    assert_eq!("50.5", FeelNumber::new(505, 1).to_string());
    assert_eq!("50.50", FeelNumber::new(5050, 2).to_string());
  }

  #[test]
  fn test_abs() {
    assert_eq!("0", FeelNumber::new(0, 0).abs().to_string());
    assert_eq!("0", FeelNumber::new(-0, 0).abs().to_string());
    assert_eq!("1", FeelNumber::new(1, 0).abs().to_string());
    assert_eq!("1", FeelNumber::new(-1, 0).abs().to_string());
    assert_eq!("0.123456", FeelNumber::new(123456, 6).abs().to_string());
    assert_eq!("0.123456", FeelNumber::new(-123456, 6).abs().to_string());
  }

  #[test]
  fn test_add() {
    assert_eq!("2", (FeelNumber::new(123, 2) + FeelNumber::new(77, 2)).to_string());
  }

  #[test]
  fn test_add_assign() {
    let mut x = FeelNumber::new(123, 2);
    x += FeelNumber::new(77, 2);
    assert_eq!("2", x.to_string());
  }

  #[test]
  fn test_ceiling() {
    assert_eq!("2", FeelNumber::new(15, 1).ceiling().to_string());
    assert_eq!("-1", FeelNumber::new(-15, 1).ceiling().to_string());
    assert_eq!("1", FeelNumber::new(3333, 4).ceiling().to_string());
    assert_eq!("-0", FeelNumber::new(-3333, 4).ceiling().to_string());
  }

  #[test]
  fn test_comparison() {
    assert!(!(FeelNumber::int(0) > FeelNumber::int(0)));
    assert!((FeelNumber::int(0) >= FeelNumber::int(0)));
    assert!((FeelNumber::new(123456, 2) > FeelNumber::new(123456, 3)));
    assert!((FeelNumber::new(123456, 3) < FeelNumber::new(123456, 2)));
    assert!((FeelNumber::new(123456, 2) <= FeelNumber::new(123456, 2)));
    assert!((FeelNumber::new(123456, 2) >= FeelNumber::new(123456, 2)));
  }

  #[test]
  fn test_constants() {
    assert_eq!("0", FeelNumber::zero().to_string());
    assert_eq!("1", FeelNumber::one().to_string());
    assert_eq!("2", FeelNumber::two().to_string());
    assert_eq!("1000000000", FeelNumber::nano().to_string());
  }

  #[test]
  fn test_div() {
    assert_eq!("2.5", (FeelNumber::new(20, 0) / FeelNumber::new(8, 0)).to_string());
  }

  #[test]
  fn test_div_assign() {
    let mut x = FeelNumber::new(20, 0);
    x /= FeelNumber::new(8, 0);
    assert_eq!("2.5", x.to_string());
  }

  #[test]
  fn test_equal() {
    assert!((FeelNumber::int(0) == FeelNumber::int(0)));
    assert!(!(FeelNumber::int(0) == FeelNumber::int(1)));
    assert!(!(FeelNumber::int(1) == FeelNumber::int(0)));
    assert!((FeelNumber::new(123456, 2) == FeelNumber::new(123456, 2)));
    assert!(!(FeelNumber::new(123456, 2) == FeelNumber::new(-123456, 2)));
  }

  #[test]
  fn test_even() {
    assert!(FeelNumber::int(-4).even());
    assert!(!FeelNumber::int(-3).even());
    assert!(FeelNumber::int(-2).even());
    assert!(!FeelNumber::int(-1).even());
    assert!(FeelNumber::int(-0).even());
    assert!(FeelNumber::int(0).even());
    assert!(!FeelNumber::int(1).even());
    assert!(FeelNumber::int(2).even());
    assert!(!FeelNumber::int(3).even());
    assert!(FeelNumber::int(4).even());
    assert!(!FeelNumber::new(41, 1).even());
  }

  #[test]
  fn test_exp() {
    assert_eq!("2.718281828459045235360287471352662", FeelNumber::int(1).exp().to_string());
    assert_eq!("54.59815003314423907811026120286088", FeelNumber::int(4).exp().to_string());
  }

  #[test]
  fn test_floor() {
    assert_eq!("1", FeelNumber::new(15, 1).floor().to_string());
    assert_eq!("-2", FeelNumber::new(-15, 1).floor().to_string());
    assert_eq!("0", FeelNumber::new(3333, 4).floor().to_string());
    assert_eq!("-1", FeelNumber::new(-3333, 4).floor().to_string());
  }

  #[test]
  fn test_is_integer() {
    assert!(FeelNumber::new(41, 0).is_integer());
    assert!(!FeelNumber::new(41, 1).is_integer());
  }

  #[test]
  fn test_is_positive() {
    assert!(!FeelNumber::new(-123, 2).is_positive());
    assert!(!FeelNumber::new(-1, 0).is_positive());
    assert!(!FeelNumber::new(-0, 0).is_positive());
    assert!(!FeelNumber::new(0, 0).is_positive());
    assert!(FeelNumber::new(1, 0).is_positive());
    assert!(FeelNumber::new(123, 2).is_positive());
  }

  #[test]
  fn test_ln() {
    assert!(FeelNumber::int(-1).ln().is_none());
    assert!(FeelNumber::int(0).ln().is_none());
    assert_eq!("0", FeelNumber::int(1).ln().unwrap().to_string());
    assert_eq!("1.386294361119890618834464242916353", FeelNumber::int(4).ln().unwrap().to_string());
    assert_eq!("2.302585092994045684017991454684364", FeelNumber::int(10).ln().unwrap().to_string());
  }

  #[test]
  fn test_minus_zero() {
    assert_eq!("0", FeelNumber::new(-0, 0).to_string());
  }

  #[test]
  fn test_mul() {
    assert_eq!("12", (FeelNumber::new(12, 1) * FeelNumber::new(10, 0)).to_string());
  }

  #[test]
  fn test_mul_assign() {
    let mut x = FeelNumber::new(12, 1);
    x *= FeelNumber::new(10, 0);
    assert_eq!("12", x.to_string());
  }

  #[test]
  fn test_neg() {
    assert_eq!("-1.23", (-FeelNumber::new(123, 2)).to_string());
    assert_eq!("1.23", (-FeelNumber::new(-123, 2)).to_string());
  }

  #[test]
  fn test_odd() {
    assert!(!FeelNumber::int(-4).odd());
    assert!(FeelNumber::int(-3).odd());
    assert!(!FeelNumber::int(-2).odd());
    assert!(FeelNumber::int(-1).odd());
    assert!(!FeelNumber::int(-0).odd());
    assert!(!FeelNumber::int(0).odd());
    assert!(FeelNumber::int(1).odd());
    assert!(!FeelNumber::int(2).odd());
    assert!(FeelNumber::int(3).odd());
    assert!(!FeelNumber::int(4).odd());
    assert!(!FeelNumber::new(31, 1).odd());
  }

  #[test]
  fn test_pow() {
    assert!(FeelNumber::int(0).pow(&FeelNumber::int(0)).is_none());
    assert_eq!(
      "41959.85737359436186095331070746801",
      FeelNumber::new(122384283, 7).pow(&FeelNumber::new(425, 2)).unwrap().to_string()
    );
  }

  #[test]
  fn test_rem() {
    assert_eq!("2", (FeelNumber::new(12, 0) % FeelNumber::new(5, 0)).to_string());
    assert_eq!("3", (FeelNumber::new(-12, 0) % FeelNumber::new(5, 0)).to_string());
    assert_eq!("-3", (FeelNumber::new(12, 0) % FeelNumber::new(-5, 0)).to_string());
    assert_eq!("-2", (FeelNumber::new(-12, 0) % FeelNumber::new(-5, 0)).to_string());
    assert_eq!("1.1", (FeelNumber::new(101, 1) % FeelNumber::new(45, 1)).to_string());
    assert_eq!("3.4", (FeelNumber::new(-101, 1) % FeelNumber::new(45, 1)).to_string());
    assert_eq!("-3.4", (FeelNumber::new(101, 1) % FeelNumber::new(-45, 1)).to_string());
    assert_eq!("-1.1", (FeelNumber::new(-101, 1) % FeelNumber::new(-45, 1)).to_string());
  }

  #[test]
  fn test_rem_assign() {
    let mut x = FeelNumber::new(101, 1);
    x %= FeelNumber::new(-45, 1);
    assert_eq!("-3.4", x.to_string());
  }

  #[test]
  fn test_round() {
    assert_eq!("123.46", FeelNumber::new(1234567, 4).round(FeelNumber::int(2)).to_string());
    assert_eq!("123.45", FeelNumber::new(1234547, 4).round(FeelNumber::int(2)).to_string());
    assert_eq!("100", FeelNumber::new(1234567, 4).round(FeelNumber::int(-2)).to_string());
    assert_eq!("200", FeelNumber::new(1634567, 4).round(FeelNumber::int(-2)).to_string());
  }

  #[test]
  fn test_sqrt() {
    assert!(FeelNumber::int(-1).sqrt().is_none());
    assert_eq!("0", FeelNumber::int(0).sqrt().unwrap().to_string());
    assert_eq!("1", FeelNumber::int(1).sqrt().unwrap().to_string());
    assert_eq!("1.414213562373095048801688724209698", FeelNumber::int(2).sqrt().unwrap().to_string());
  }

  #[test]
  fn test_square() {
    assert_eq!("4", FeelNumber::int(2).square().unwrap().to_string());
    assert_eq!("25", FeelNumber::int(5).square().unwrap().to_string());
  }

  #[test]
  fn test_sub() {
    assert_eq!("1", (FeelNumber::new(123, 2) - FeelNumber::new(23, 2)).to_string());
  }

  #[test]
  fn test_sub_assign() {
    let mut x = FeelNumber::new(123, 2);
    x -= FeelNumber::new(23, 2);
    assert_eq!("1", x.to_string());
  }

  #[test]
  fn test_scientific_to_plain() {
    assert_eq!("12300", scientific_to_plain("1.23E+4".to_string()));
    assert_eq!("100", scientific_to_plain("1E+2".to_string()));
    assert_eq!("0.00000000000000000000001", scientific_to_plain("1E-23".to_string()));
    assert_eq!("0.00000000000000001234567", scientific_to_plain("1.234567E-17".to_string()));
  }
}
