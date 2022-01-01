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

use crate::point::Point;
use crate::rect::Rect;
use dmntk_common::DmntkError;

#[derive(Debug, PartialEq)]
pub enum RecognizerError {
  CanvasExpectedCharactersNotFound(Vec<char>),
  CanvasCharacterIsNotAllowed(char, Vec<char>),
  CanvasRectangleNotClosed(Point, Point),
  CanvasRegionNotFound(Rect),
  PlaneIsEmpty,
  PlaneRowIsOutOfRange,
  PlaneColumnIsOutOfRange,
  PlaneNoMainDoubleCrossing,
  PlaneInvalidOutputClause,
  PlaneInvalidRuleNumber(usize),
  PlaneCellIsNotRegion(String),
  InvalidInputExpressions,
  TooManyRowsInInputClause,
  NoOutputClause,
  ExpectedLeftBelowRuleNumbersPlacement,
  ExpectedRightAfterRuleNumbersPlacement,
  ExpectedTopLeftHitPolicyPlacement,
  ExpectedBottomLeftHitPolicyPlacement,
  ExpectedNoRuleNumbersPresent,
  RecognizingCrossTabNotSupportedYet,
  InvalidSize(String),
}

impl From<RecognizerError> for DmntkError {
  fn from(e: RecognizerError) -> Self {
    DmntkError::new("RecognizerError", &format!("{}", e))
  }
}

impl std::fmt::Display for RecognizerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RecognizerError::CanvasExpectedCharactersNotFound(chars) => {
        write!(f, "expected characters not found: {:?}", chars)
      }
      RecognizerError::CanvasCharacterIsNotAllowed(ch, allowed) => {
        write!(f, "character '{}' is not allowed in {:?}", ch, allowed)
      }
      RecognizerError::CanvasRectangleNotClosed(p1, p2) => {
        write!(f, "rectangle is not closed, start point: {}, end point: {}", p1, p2)
      }
      RecognizerError::CanvasRegionNotFound(r) => {
        write!(f, "region not found, rect: {}", r)
      }
      RecognizerError::PlaneIsEmpty => {
        write!(f, "plane is empty")
      }
      RecognizerError::PlaneRowIsOutOfRange => {
        write!(f, "plane row is out of range")
      }
      RecognizerError::PlaneColumnIsOutOfRange => {
        write!(f, "plane column is out of range")
      }
      RecognizerError::PlaneNoMainDoubleCrossing => {
        write!(f, "plane no main double crossing")
      }
      RecognizerError::PlaneInvalidOutputClause => {
        write!(f, "plane invalid output clause")
      }
      RecognizerError::PlaneInvalidRuleNumber(num) => {
        write!(f, "plane invalid rule number: {}", num)
      }
      RecognizerError::PlaneCellIsNotRegion(details) => {
        write!(f, "not a region cell in plane: {}", details)
      }
      RecognizerError::InvalidInputExpressions => {
        write!(f, "invalid input expressions")
      }
      RecognizerError::TooManyRowsInInputClause => {
        write!(f, "too many rows in output clause")
      }
      RecognizerError::NoOutputClause => {
        write!(f, "no output clause")
      }
      RecognizerError::ExpectedLeftBelowRuleNumbersPlacement => {
        write!(f, "expected left-below rule numbers placement")
      }
      RecognizerError::ExpectedRightAfterRuleNumbersPlacement => {
        write!(f, "expected right-after rule numbers placement")
      }
      RecognizerError::ExpectedTopLeftHitPolicyPlacement => {
        write!(f, "expected top-left hit policy placement")
      }
      RecognizerError::ExpectedBottomLeftHitPolicyPlacement => {
        write!(f, "expected bottom-left hit policy placement")
      }
      RecognizerError::ExpectedNoRuleNumbersPresent => {
        write!(f, "expected no rule numbers present")
      }
      RecognizerError::RecognizingCrossTabNotSupportedYet => {
        write!(f, "recognizing cross-tab decision tables is not yet implemented")
      }
      RecognizerError::InvalidSize(details) => {
        write!(f, "invalid size: {}", details)
      }
    }
  }
}

pub fn canvas_expected_characters_not_found(chars: Vec<char>) -> DmntkError {
  RecognizerError::CanvasExpectedCharactersNotFound(chars).into()
}

pub fn canvas_character_is_not_allowed(ch: char, allowed: Vec<char>) -> DmntkError {
  RecognizerError::CanvasCharacterIsNotAllowed(ch, allowed).into()
}

pub fn canvas_rectangle_not_closed(p1: Point, p2: Point) -> DmntkError {
  RecognizerError::CanvasRectangleNotClosed(p1, p2).into()
}

pub fn canvas_region_not_found(r: Rect) -> DmntkError {
  RecognizerError::CanvasRegionNotFound(r).into()
}

pub fn plane_is_empty() -> DmntkError {
  RecognizerError::PlaneIsEmpty.into()
}

pub fn plane_cell_is_not_region(details: &str) -> DmntkError {
  RecognizerError::PlaneCellIsNotRegion(details.to_string()).into()
}

pub fn plane_row_is_out_of_range() -> DmntkError {
  RecognizerError::PlaneRowIsOutOfRange.into()
}

pub fn plane_no_main_double_crossing() -> DmntkError {
  RecognizerError::PlaneNoMainDoubleCrossing.into()
}

pub fn plane_column_is_out_of_range() -> DmntkError {
  RecognizerError::PlaneColumnIsOutOfRange.into()
}

pub fn plane_invalid_output_clause() -> DmntkError {
  RecognizerError::PlaneInvalidOutputClause.into()
}

pub fn plane_invalid_rule_number(num: usize) -> DmntkError {
  RecognizerError::PlaneInvalidRuleNumber(num).into()
}

pub fn expected_no_rule_numbers_present() -> DmntkError {
  RecognizerError::ExpectedNoRuleNumbersPresent.into()
}

pub fn invalid_input_expressions() -> DmntkError {
  RecognizerError::InvalidInputExpressions.into()
}

pub fn no_output_clause() -> DmntkError {
  RecognizerError::NoOutputClause.into()
}

pub fn expected_right_after_rule_numbers_placement() -> DmntkError {
  RecognizerError::ExpectedRightAfterRuleNumbersPlacement.into()
}

pub fn expected_left_below_rule_numbers_placement() -> DmntkError {
  RecognizerError::ExpectedLeftBelowRuleNumbersPlacement.into()
}

pub fn expected_bottom_left_hit_policy_placement() -> DmntkError {
  RecognizerError::ExpectedBottomLeftHitPolicyPlacement.into()
}

pub fn expected_top_left_hit_policy_placement() -> DmntkError {
  RecognizerError::ExpectedTopLeftHitPolicyPlacement.into()
}

pub fn recognizing_cross_tab_not_supported_yet() -> DmntkError {
  RecognizerError::RecognizingCrossTabNotSupportedYet.into()
}

pub fn too_many_rows_in_input_clause() -> DmntkError {
  RecognizerError::TooManyRowsInInputClause.into()
}

pub fn invalid_size(details: &str) -> DmntkError {
  RecognizerError::InvalidSize(details.to_string()).into()
}
