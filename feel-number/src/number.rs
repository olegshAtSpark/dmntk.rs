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
use std::fmt::Debug;
use std::str::FromStr;

/// FEEL number.
#[derive(Debug, Copy, Clone)]
pub struct FeelNumber(DecQuad);

impl FeelNumber {
  /// Creates a new [FeelNumber] from number and scale.
  pub fn new(number: i128, scale: i32) -> Self {
    let n = dec_from_string(&format!("{}", number));
    let s = dec_from_string(&format!("{}", -scale));
    Self(dec_scale(&n, &s))
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
    Self(dec_neg(&self.0))
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
    let mut split2 = before_exponent.split('.');
    let before_decimal = split2.next().unwrap();
    let after_decimal = split2.next().unwrap();
    let zeroes = (1..(exponent_digits - after_decimal.len() + 1)).map(|_| "0").collect::<String>();
    format!("{}{}{}", before_decimal, after_decimal, zeroes)
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
    assert_eq!("49.0", FeelNumber::new(490, 1).to_string());
    assert_eq!("4900", FeelNumber::new(490, -1).to_string());
    assert_eq!("50", FeelNumber::new(50, 0).to_string());
    assert_eq!("50.5", FeelNumber::new(505, 1).to_string());
    assert_eq!("50.50", FeelNumber::new(5050, 2).to_string());
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
  fn test_minus_zero() {
    assert_eq!("0", FeelNumber::new(-0, 0).to_string());
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
  fn test_neg() {
    assert_eq!("-1.23", (-FeelNumber::new(123, 2)).to_string());
    assert_eq!("1.23", (-FeelNumber::new(-123, 2)).to_string());
  }

  #[test]
  fn test_constants() {
    assert_eq!("0", FeelNumber::zero().to_string());
    assert_eq!("1", FeelNumber::one().to_string());
    assert_eq!("2", FeelNumber::two().to_string());
    assert_eq!("1000000000", FeelNumber::nano().to_string());
  }

  //
  // #[test]
  // fn test_ceiling() {
  //   assert_eq!("0", FeelNumber::new(-3333, 4).ceil().to_string());
  // }
  //
  // #[test]
  // fn test_exp() {
  //   assert_eq!("54.5981500331442362039524596", FeelNumber::new(4, 0).exp().unwrap().to_string());
  // }
  //
  // #[test]
  // fn test_log() {
  //   assert_eq!("1.3862943611198905724535279656", FeelNumber::new(4, 0).ln().unwrap().to_string());
  // }
}
