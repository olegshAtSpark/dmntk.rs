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

//! `FEEL` numbers.

use crate::numbers::errors::err_invalid_number_literal;
use dmntk_common::{DmntkError, Jsonify};
use rust_decimal::prelude::*;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
use std::str::FromStr;

/// FEEL number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FeelNumber(Decimal);

impl FeelNumber {
  ///
  pub fn new(num: i128, scale: u32) -> Self {
    Self(Decimal::from_i128_with_scale(num, scale))
  }
  ///
  pub fn new_int(num: i64) -> Self {
    Self(Decimal::new(num, 0))
  }
  ///
  #[inline]
  pub fn zero() -> Self {
    Self(Decimal::zero())
  }
  ///
  #[inline]
  pub fn one() -> Self {
    Self(Decimal::one())
  }
  ///
  #[inline]
  pub fn two() -> Self {
    Self(dec!(2.0))
  }
  ///
  #[inline]
  pub fn nano() -> Self {
    Self(dec!(1_000_000_000))
  }
  ///
  pub fn abs(&self) -> Self {
    Self(self.0.abs())
  }
  ///
  pub fn ceil(&self) -> Self {
    Self(self.0.ceil().normalize())
  }
  ///
  pub fn floor(&self) -> Self {
    Self(self.0.floor().normalize())
  }
  ///
  pub fn pow(&self, rhs: &FeelNumber) -> Option<Self> {
    rhs.0.to_i64().and_then(|scale| self.0.checked_powi(scale).map(Self))
  }
  ///
  pub fn pow2(&self) -> Self {
    Self(self.0.powu(2))
  }
  ///
  pub fn exp(&self) -> Option<Self> {
    self.0.to_f64().and_then(|n| Decimal::from_f64_retain(n.exp()).map(|d| Self(d.normalize())))
  }
  ///
  pub fn sqrt(&self) -> Option<Self> {
    if self.0.is_zero() {
      Some(Self::zero())
    } else {
      self.0.sqrt().map(Self)
    }
  }
  ///
  pub fn ln(&self) -> Option<Self> {
    self.0.to_f64().and_then(|n| Decimal::from_f64_retain(n.ln()).map(|d| Self(d.normalize())))
  }
  ///
  pub fn round(&self, rhs: FeelNumber) -> Self {
    Self(
      self
        .0
        .round_dp_with_strategy(rhs.0.to_u32().unwrap_or(0), RoundingStrategy::MidpointNearestEven),
    )
  }
  ///
  pub fn trunc(&self) -> Self {
    Self(self.0.trunc())
  }
  ///
  pub fn fract(&self) -> Self {
    Self(self.0.fract())
  }
  ///
  pub fn even(&self) -> bool {
    self.0.fract() == Decimal::zero() && self.0 % dec!(2) == Decimal::zero()
  }
  ///
  pub fn odd(&self) -> bool {
    self.0.fract() == Decimal::zero() && self.0 % dec!(2) != Decimal::zero()
  }
  ///
  pub fn is_integer(&self) -> bool {
    self.0.fract() == Decimal::zero()
  }
  ///
  pub fn is_positive(&self) -> bool {
    self.0 >= Decimal::zero()
  }
  ///
  pub fn is_one(&self) -> bool {
    self.0 == Decimal::one()
  }
}

impl FromStr for FeelNumber {
  type Err = DmntkError;
  ///
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(s.parse::<Decimal>().map_err(|e| err_invalid_number_literal(&e.to_string()))?))
  }
}

impl From<Decimal> for FeelNumber {
  ///
  fn from(value: Decimal) -> Self {
    Self(value)
  }
}

impl From<usize> for FeelNumber {
  ///
  fn from(value: usize) -> Self {
    Self(value.into())
  }
}

impl From<isize> for FeelNumber {
  ///
  fn from(value: isize) -> Self {
    Self(value.into())
  }
}

impl From<u8> for FeelNumber {
  ///
  fn from(value: u8) -> Self {
    Self(value.into())
  }
}

impl From<i32> for FeelNumber {
  ///
  fn from(value: i32) -> Self {
    Self(value.into())
  }
}

impl From<u32> for FeelNumber {
  ///
  fn from(value: u32) -> Self {
    Self(value.into())
  }
}

impl From<i64> for FeelNumber {
  ///
  fn from(value: i64) -> Self {
    Self(value.into())
  }
}

impl From<u64> for FeelNumber {
  ///
  fn from(value: u64) -> Self {
    Self(value.into())
  }
}

impl From<FeelNumber> for usize {
  ///
  fn from(value: FeelNumber) -> usize {
    value.0.to_usize().unwrap_or(0)
  }
}

impl From<FeelNumber> for isize {
  ///
  fn from(value: FeelNumber) -> isize {
    value.0.to_isize().unwrap_or(0)
  }
}

impl From<FeelNumber> for u8 {
  ///
  fn from(value: FeelNumber) -> u8 {
    value.0.to_u8().unwrap_or(0)
  }
}

impl From<FeelNumber> for i32 {
  ///
  fn from(value: FeelNumber) -> i32 {
    value.0.to_i32().unwrap_or(0)
  }
}

impl From<FeelNumber> for i64 {
  ///
  fn from(value: FeelNumber) -> i64 {
    value.0.to_i64().unwrap_or(0)
  }
}

impl From<FeelNumber> for u64 {
  ///
  fn from(value: FeelNumber) -> u64 {
    value.0.to_u64().unwrap_or(0)
  }
}

impl std::ops::Add<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl std::ops::AddAssign<FeelNumber> for FeelNumber {
  ///
  fn add_assign(&mut self, rhs: FeelNumber) {
    self.0 = self.0 + rhs.0
  }
}

impl std::ops::Sub<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}

impl std::ops::Mul<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    Self((self.0 * rhs.0).normalize())
  }
}

impl std::ops::Div<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    Self((self.0 / rhs.0).normalize())
  }
}

impl std::ops::Rem<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn rem(self, rhs: Self) -> Self::Output {
    Self((self.0 % rhs.0).normalize())
  }
}

impl std::ops::Neg for FeelNumber {
  type Output = Self;
  ///
  fn neg(self) -> Self::Output {
    Self(-self.0)
  }
}

impl std::fmt::Display for FeelNumber {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Jsonify for FeelNumber {
  /// Converts a [FeelNumber] to its `JSON` representation.
  fn jsonify(&self) -> String {
    format!("{}", self.0)
  }
}

mod errors {
  use dmntk_common::DmntkError;

  /// `FEEL` number errors.
  #[derive(Error, Debug)]
  enum FeelNumberError {
    #[error("invalid number literal `{0}`")]
    InvalidNumberLiteral(String),
  }

  impl From<FeelNumberError> for DmntkError {
    fn from(e: FeelNumberError) -> Self {
      DmntkError::new("FeelNumberError", &e.to_string())
    }
  }

  pub fn err_invalid_number_literal(s: &str) -> DmntkError {
    FeelNumberError::InvalidNumberLiteral(s.to_string()).into()
  }
}

#[cfg(test)]
mod tests {
  use crate::FeelNumber;

  #[test]
  fn test_stringify() {
    assert_eq!("49", FeelNumber::new(49, 0).to_string());
    assert_eq!("49.0", FeelNumber::new(490, 1).to_string());
    assert_eq!("50", FeelNumber::new(50, 0).to_string());
    assert_eq!("50.5", FeelNumber::new(505, 1).to_string());
    assert_eq!("50.50", FeelNumber::new(5050, 2).to_string());
  }

  #[test]
  fn test_division() {
    assert_eq!("2.5", (FeelNumber::new(20, 0) / FeelNumber::new(8, 0)).to_string());
  }

  #[test]
  fn test_multiplication() {
    assert_eq!(
      "1200",
      (FeelNumber::new(12, 1) * FeelNumber::new(10, 0).pow(&FeelNumber::new(3, 0)).unwrap()).to_string()
    );
  }

  #[test]
  fn test_minus_zero() {
    assert_eq!("0", FeelNumber::new(-0, 0).to_string());
  }

  #[test]
  fn test_ceiling() {
    assert_eq!("0", FeelNumber::new(-3333, 4).ceil().to_string());
  }

  #[test]
  fn test_exp() {
    assert_eq!("54.5981500331442362039524596", FeelNumber::new(4, 0).exp().unwrap().to_string());
  }

  #[test]
  fn test_log() {
    assert_eq!("1.3862943611198905724535279656", FeelNumber::new(4, 0).ln().unwrap().to_string());
  }
}
