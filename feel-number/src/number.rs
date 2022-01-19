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

//! `FEEL` number type.

use crate::dec::*;
use crate::number::errors::*;
use dmntk_common::{DmntkError, Jsonify};
use std::cmp::Ordering;
use std::str::FromStr;

macro_rules! from_feel_number_into {
  ($l:tt) => {
    impl TryFrom<FeelNumber> for $l {
      type Error = DmntkError;
      fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
        $l::try_from(&value)
      }
    }

    impl TryFrom<&FeelNumber> for $l {
      type Error = DmntkError;
      fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
        return value.to_string().parse::<$l>().map_err(|_| err_number_conversion_failed());
      }
    }
  };
}

/// FEEL number.
#[derive(Copy, Clone)]
#[must_use]
pub struct FeelNumber(DecQuad);

impl FeelNumber {
  /// Creates a new [FeelNumber] from integer value and scale.
  pub fn new(n: i128, s: i32) -> Self {
    Self(dec_scale_b(&dec_from_string(&format!("{}", n)), &dec_from_string(&format!("{}", -s))))
  }
  /// Creates a new [FeelNumber] from [isize].
  pub fn from_isize(n: isize) -> Self {
    Self(dec_from_string(&format!("{}", n)))
  }
  /// Creates a new [FeelNumber] from [usize].
  pub fn from_usize(n: usize) -> Self {
    Self(dec_from_string(&format!("{}", n)))
  }
  /// Creates a new [FeelNumber] from [i128].
  pub fn from_i128(n: i128) -> Self {
    Self(dec_from_string(&format!("{}", n)))
  }
  /// Creates a new [FeelNumber] from [String].
  pub fn from_string(s: &str) -> Self {
    Self(dec_from_string(s))
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
    dec_is_zero(&dec_remainder(&self.0, &DEC_TWO))
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
  pub fn fract(&self) -> Self {
    Self(dec_fract(&self.0))
  }
  ///
  pub fn is_integer(&self) -> bool {
    dec_is_integer(&self.0)
  }
  ///
  pub fn is_one(&self) -> bool {
    dec_is_zero(&dec_compare(&self.0, &DEC_ONE))
  }
  ///
  pub fn is_negative(&self) -> bool {
    dec_is_negative(&self.0)
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
    dec_is_integer(&self.0) && !dec_is_zero(&dec_remainder(&self.0, &DEC_TWO))
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
  pub fn round(&self, rhs: &FeelNumber) -> Self {
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
  ///
  pub fn trunc(&self) -> Self {
    Self(dec_trunc(&self.0))
  }
  ///
  pub fn to_u8(&self) -> Option<u8> {
    u8::try_from(self).ok()
  }
  ///
  pub fn to_u64(&self) -> Option<u64> {
    u64::try_from(self).ok()
  }
  ///
  pub fn to_isize(&self) -> Option<isize> {
    isize::try_from(self).ok()
  }
  ///
  pub fn to_usize(&self) -> Option<usize> {
    usize::try_from(self).ok()
  }
}

impl PartialEq<FeelNumber> for FeelNumber {
  fn eq(&self, rhs: &Self) -> bool {
    dec_is_zero(&dec_compare(&self.0, &rhs.0))
  }
}

impl PartialEq<FeelNumber> for isize {
  fn eq(&self, rhs: &FeelNumber) -> bool {
    dec_is_zero(&dec_compare(&FeelNumber::from_isize(*self).0, &rhs.0))
  }
}

impl PartialEq<isize> for FeelNumber {
  fn eq(&self, rhs: &isize) -> bool {
    dec_is_zero(&dec_compare(&self.0, &FeelNumber::from_isize(*rhs).0))
  }
}

impl PartialOrd<FeelNumber> for FeelNumber {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let flag = dec_compare(&self.0, &rhs.0);
    if dec_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<FeelNumber> for isize {
  fn partial_cmp(&self, rhs: &FeelNumber) -> Option<Ordering> {
    let flag = dec_compare(&FeelNumber::from_isize(*self).0, &rhs.0);
    if dec_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<isize> for FeelNumber {
  fn partial_cmp(&self, rhs: &isize) -> Option<Ordering> {
    let flag = dec_compare(&self.0, &FeelNumber::from_isize(*rhs).0);
    if dec_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
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

impl std::fmt::Debug for FeelNumber {
  ///
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", dec_to_string(&dec_reduce(&self.0)))
  }
}

impl std::fmt::Display for FeelNumber {
  /// Converts [FeelNumber] to its textual representation.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", scientific_to_plain(dec_to_string(&self.0)))
  }
}

impl Jsonify for FeelNumber {
  /// Converts [FeelNumber] to its `JSON` representation.
  fn jsonify(&self) -> String {
    scientific_to_plain(dec_to_string(&self.0))
  }
}

impl FromStr for FeelNumber {
  type Err = DmntkError;
  ///
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let n = dec_from_string(s);
    if dec_is_finite(&n) {
      Ok(Self(n))
    } else {
      Err(err_invalid_number_literal(s))
    }
  }
}

impl From<u8> for FeelNumber {
  ///
  fn from(value: u8) -> Self {
    Self(dec_from_u32(value as u32))
  }
}

impl From<FeelNumber> for u8 {
  ///
  fn from(value: FeelNumber) -> Self {
    dec_to_u32(&value.0) as u8
  }
}

impl From<&FeelNumber> for u8 {
  ///
  fn from(value: &FeelNumber) -> Self {
    dec_to_u32(&value.0) as u8
  }
}

impl From<i32> for FeelNumber {
  ///
  fn from(value: i32) -> Self {
    Self(dec_from_i32(value))
  }
}

impl From<FeelNumber> for i32 {
  ///
  fn from(value: FeelNumber) -> i32 {
    dec_to_i32(&value.0)
  }
}

impl From<u32> for FeelNumber {
  ///
  fn from(value: u32) -> Self {
    Self(dec_from_u32(value))
  }
}

impl From<i64> for FeelNumber {
  ///
  fn from(value: i64) -> Self {
    Self::from_i128(value as i128)
  }
}

impl From<isize> for FeelNumber {
  ///
  fn from(value: isize) -> Self {
    Self::from_i128(value as i128)
  }
}

impl From<usize> for FeelNumber {
  ///
  fn from(value: usize) -> Self {
    Self::from_i128(value as i128)
  }
}

from_feel_number_into!(isize);
from_feel_number_into!(usize);
from_feel_number_into!(u64);

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

mod errors {
  use dmntk_common::DmntkError;

  /// `FEEL` number errors.
  struct FeelNumberError(String);

  impl From<FeelNumberError> for DmntkError {
    /// Converts into [DmntkError].
    fn from(e: FeelNumberError) -> Self {
      DmntkError::new("FeelNumberError", &e.0)
    }
  }

  /// Creates invalid number literal error.
  pub fn err_invalid_number_literal(s: &str) -> DmntkError {
    FeelNumberError(format!("invalid number literal '{}'", s)).into()
  }

  /// Creates number conversion error.
  pub fn err_number_conversion_failed() -> DmntkError {
    FeelNumberError("number conversion failed".to_string()).into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stringify() {
    assert_eq!("49", FeelNumber::new(49, 0).to_string());
    assert_eq!("49", FeelNumber::from_i128(49).to_string());
    assert_eq!("49.0", FeelNumber::new(490, 1).to_string());
    assert_eq!("4900", FeelNumber::new(4900, 0).to_string());
    assert_eq!("50", FeelNumber::new(50, 0).to_string());
    assert_eq!("50", FeelNumber::from_i128(50).to_string());
    assert_eq!("50.5", FeelNumber::new(505, 1).to_string());
    assert_eq!("50.50", FeelNumber::new(5050, 2).to_string());
  }

  #[test]
  fn test_debug() {
    assert_eq!("49", format!("{:?}", FeelNumber::new(49, 0)));
    assert_eq!("1.23456789", format!("{:?}", FeelNumber::new(123456789, 8)));
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
    assert_eq!("0", FeelNumber::new(-3333, 4).ceiling().to_string());
  }

  #[test]
  fn test_comparison() {
    assert_eq!(FeelNumber::new(120000, 0), FeelNumber::from_i128(120000));
    assert!(!(FeelNumber::from_i128(0) > FeelNumber::from_i128(0)));
    assert!((FeelNumber::from_i128(0) >= FeelNumber::from_i128(0)));
    assert!((FeelNumber::new(123456, 2) > FeelNumber::new(123456, 3)));
    assert!((FeelNumber::new(123456, 3) < FeelNumber::new(123456, 2)));
    assert!((FeelNumber::new(123456, 2) <= FeelNumber::new(123456, 2)));
    assert!((FeelNumber::new(123456, 2) >= FeelNumber::new(123456, 2)));
    assert!((-6111..6176).contains(&FeelNumber::from_i128(0)));
    assert!((0..6176).contains(&FeelNumber::from_i128(6175)));
    assert!((-3..2).contains(&FeelNumber::from_i128(-3)));
    assert!(!(-3..2).contains(&FeelNumber::from_i128(-4)));
    assert!((0..60).contains(&FeelNumber::new(0, 0)));
    assert!((0..60).contains(&FeelNumber::new(59_999_999_999, 9)));
    assert!(!(0..60).contains(&FeelNumber::from_i128(60)));
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
    assert!((FeelNumber::from_i128(0) == FeelNumber::from_i128(0)));
    assert!(!(FeelNumber::from_i128(0) == FeelNumber::from_i128(1)));
    assert!(!(FeelNumber::from_i128(1) == FeelNumber::from_i128(0)));
    assert!((FeelNumber::new(123456, 2) == FeelNumber::new(123456, 2)));
    assert!(!(FeelNumber::new(123456, 2) == FeelNumber::new(-123456, 2)));
    assert!((FeelNumber::from_i128(0) == 0_isize));
    assert!((FeelNumber::from_i128(1) == 1_isize));
    assert!((FeelNumber::from_i128(-1) == -1_isize));
    assert!((0_isize == FeelNumber::from_i128(0)));
    assert!((1_isize == FeelNumber::from_i128(1)));
    assert!((-1_isize == FeelNumber::from_i128(-1)));
  }

  #[test]
  fn test_even() {
    assert!(FeelNumber::from_i128(-4).even());
    assert!(!FeelNumber::from_i128(-3).even());
    assert!(FeelNumber::from_i128(-2).even());
    assert!(!FeelNumber::from_i128(-1).even());
    assert!(FeelNumber::from_i128(-0).even());
    assert!(FeelNumber::from_i128(0).even());
    assert!(!FeelNumber::from_i128(1).even());
    assert!(FeelNumber::from_i128(2).even());
    assert!(!FeelNumber::from_i128(3).even());
    assert!(FeelNumber::from_i128(4).even());
    assert!(!FeelNumber::new(41, 1).even());
  }

  #[test]
  fn test_exp() {
    assert_eq!("2.718281828459045235360287471352662", FeelNumber::from_i128(1).exp().to_string());
    assert_eq!("54.59815003314423907811026120286088", FeelNumber::from_i128(4).exp().to_string());
  }

  #[test]
  fn test_from_string() {
    assert_eq!("0", FeelNumber::from_string("0").to_string());
    assert_eq!("-0", FeelNumber::from_string("-0").to_string());
    assert_eq!("1", FeelNumber::from_string("1").to_string());
    assert_eq!("-1", FeelNumber::from_string("-1").to_string());
    assert_eq!("1.23456789", FeelNumber::from_string("1.23456789").to_string());
    assert_eq!("-1.23456789", FeelNumber::from_string("-1.23456789").to_string());
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
  fn test_is_negative() {
    assert!(FeelNumber::new(-123, 2).is_negative());
    assert!(FeelNumber::new(-1, 0).is_negative());
    assert!(!FeelNumber::new(-0, 0).is_negative());
    assert!(!FeelNumber::new(0, 0).is_negative());
    assert!(!FeelNumber::new(1, 0).is_negative());
    assert!(!FeelNumber::new(123, 2).is_negative());
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
    assert!(FeelNumber::from_i128(-1).ln().is_none());
    assert!(FeelNumber::from_i128(0).ln().is_none());
    assert_eq!("0", FeelNumber::from_i128(1).ln().unwrap().to_string());
    assert_eq!("1.386294361119890618834464242916353", FeelNumber::from_i128(4).ln().unwrap().to_string());
    assert_eq!("2.302585092994045684017991454684364", FeelNumber::from_i128(10).ln().unwrap().to_string());
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
    assert!(!FeelNumber::from_i128(-4).odd());
    assert!(FeelNumber::from_i128(-3).odd());
    assert!(!FeelNumber::from_i128(-2).odd());
    assert!(FeelNumber::from_i128(-1).odd());
    assert!(!FeelNumber::from_i128(-0).odd());
    assert!(!FeelNumber::from_i128(0).odd());
    assert!(FeelNumber::from_i128(1).odd());
    assert!(!FeelNumber::from_i128(2).odd());
    assert!(FeelNumber::from_i128(3).odd());
    assert!(!FeelNumber::from_i128(4).odd());
    assert!(!FeelNumber::new(31, 1).odd());
  }

  #[test]
  fn test_pow() {
    assert!(FeelNumber::from_i128(0).pow(&FeelNumber::from_i128(0)).is_none());
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
    assert_eq!("123.46", FeelNumber::new(1234567, 4).round(&FeelNumber::from_i128(2)).to_string());
    assert_eq!("123.45", FeelNumber::new(1234547, 4).round(&FeelNumber::from_i128(2)).to_string());
    assert_eq!("100", FeelNumber::new(1234567, 4).round(&FeelNumber::from_i128(-2)).to_string());
    assert_eq!("200", FeelNumber::new(1634567, 4).round(&FeelNumber::from_i128(-2)).to_string());
  }

  #[test]
  fn test_sqrt() {
    assert!(FeelNumber::from_i128(-1).sqrt().is_none());
    assert_eq!("0", FeelNumber::from_i128(0).sqrt().unwrap().to_string());
    assert_eq!("1", FeelNumber::from_i128(1).sqrt().unwrap().to_string());
    assert_eq!("1.414213562373095048801688724209698", FeelNumber::from_i128(2).sqrt().unwrap().to_string());
  }

  #[test]
  fn test_square() {
    assert_eq!("4", FeelNumber::from_i128(2).square().unwrap().to_string());
    assert_eq!("25", FeelNumber::from_i128(5).square().unwrap().to_string());
    assert_eq!(None, FeelNumber::from_string("NaN").square());
    assert_eq!(None, FeelNumber::from_string("Inf").square());
    assert_eq!(None, FeelNumber::from_string("-Inf").square());
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

  #[test]
  fn test_try_from_number_to_isize() {
    assert!(isize::try_from(FeelNumber::from_i128(2)).is_ok());
    assert!(isize::try_from(FeelNumber::from_i128(isize::MAX as i128)).is_ok());
    assert!(isize::try_from(FeelNumber::from_i128(isize::MIN as i128)).is_ok());
    assert!(isize::try_from(FeelNumber::from_i128(i128::MAX)).is_err());
    assert!(isize::try_from(FeelNumber::from_i128(i128::MIN)).is_err());
  }

  #[test]
  fn test_try_from_number_to_usize() {
    assert!(usize::try_from(FeelNumber::from_i128(0)).is_ok());
    assert!(usize::try_from(FeelNumber::from_i128(2)).is_ok());
    assert!(usize::try_from(FeelNumber::from_usize(usize::MAX)).is_ok());
    assert!(usize::try_from(FeelNumber::from_usize(usize::MIN)).is_ok());
    assert!(usize::try_from(FeelNumber::from_isize(isize::MAX)).is_ok());
    assert!(usize::try_from(FeelNumber::from_isize(isize::MIN)).is_err());
    assert!(usize::try_from(FeelNumber::from_i128(i128::MAX)).is_err());
    assert!(usize::try_from(FeelNumber::from_i128(i128::MIN)).is_err());
    assert!(usize::try_from(FeelNumber::from_i128(-1)).is_err());
    assert_eq!(Some(0), FeelNumber::from_i128(0).to_usize());
    assert_eq!(Some(2), FeelNumber::from_i128(2).to_usize());
    assert_eq!(Some(usize::MAX), FeelNumber::from_usize(usize::MAX).to_usize());
    assert_eq!(Some(usize::MIN), FeelNumber::from_usize(usize::MIN).to_usize());
    assert_eq!(Some(usize::MAX / 2), FeelNumber::from_isize(isize::MAX).to_usize());
    assert_eq!(None, FeelNumber::from_isize(isize::MIN).to_usize());
    assert_eq!(None, FeelNumber::from_i128(i128::MAX).to_usize());
    assert_eq!(None, FeelNumber::from_i128(i128::MIN).to_usize());
    assert_eq!(None, FeelNumber::from_i128(-1).to_usize());
  }

  #[test]
  fn test_try_from_number_to_u64() {
    assert!(u64::try_from(FeelNumber::from_i128(0)).is_ok());
    assert!(u64::try_from(FeelNumber::from_i128(2)).is_ok());
    assert!(u64::try_from(FeelNumber::from_usize(usize::MAX)).is_ok());
    assert!(u64::try_from(FeelNumber::from_usize(usize::MIN)).is_ok());
    assert!(u64::try_from(FeelNumber::from_isize(isize::MAX)).is_ok());
    assert!(u64::try_from(FeelNumber::from_isize(isize::MIN)).is_err());
    assert!(u64::try_from(FeelNumber::from_i128(i128::MAX)).is_err());
    assert!(u64::try_from(FeelNumber::from_i128(i128::MIN)).is_err());
    assert!(u64::try_from(FeelNumber::from_i128(-1)).is_err());
    assert_eq!(Some(300_000_000), FeelNumber::from_i128(300_000_000).to_u64());
  }
}
