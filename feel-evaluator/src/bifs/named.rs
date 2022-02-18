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

use crate::bifs::core;
use dmntk_feel::bif::Bif;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, Name};

lazy_static! {
  static ref NAME_DATE: Name = Name::from("date");
  static ref NAME_DECIMAL_SEPARATOR: Name = Name::new(&["decimal", "separator"]);
  static ref NAME_DELIMITER: Name = Name::from("delimiter");
  static ref NAME_GROUPING_SEPARATOR: Name = Name::new(&["grouping", "separator"]);
  static ref NAME_DAY: Name = Name::from("day");
  static ref NAME_DIVIDEND: Name = Name::from("dividend");
  static ref NAME_DIVISOR: Name = Name::from("divisor");
  static ref NAME_FLAGS: Name = Name::from("flags");
  static ref NAME_FROM: Name = Name::from("from");
  static ref NAME_HOUR: Name = Name::from("hour");
  static ref NAME_INPUT: Name = Name::from("input");
  static ref NAME_KEY: Name = Name::from("key");
  static ref NAME_LENGTH: Name = Name::from("length");
  static ref NAME_LIST: Name = Name::from("list");
  static ref NAME_MATCH: Name = Name::from("match");
  static ref NAME_MONTH: Name = Name::from("month");
  static ref NAME_MINUTE: Name = Name::from("minute");
  static ref NAME_N: Name = Name::from("n");
  static ref NAME_M: Name = Name::from("m");
  static ref NAME_NEGAND: Name = Name::from("negand");
  static ref NAME_NEW_ITEM: Name = Name::from("newItem");
  static ref NAME_NUMBER: Name = Name::from("number");
  static ref NAME_OFFSET: Name = Name::from("offset");
  static ref NAME_PATTERN: Name = Name::from("pattern");
  static ref NAME_POINT: Name = Name::from("point");
  static ref NAME_POINT_1: Name = Name::from("point1");
  static ref NAME_POINT_2: Name = Name::from("point2");
  static ref NAME_POSITION: Name = Name::from("position");
  static ref NAME_PRECEDES: Name = Name::from("precedes");
  static ref NAME_RANGE: Name = Name::from("range");
  static ref NAME_RANGE_1: Name = Name::from("range1");
  static ref NAME_RANGE_2: Name = Name::from("range2");
  static ref NAME_REPLACEMENT: Name = Name::from("replacement");
  static ref NAME_SCALE: Name = Name::from("scale");
  static ref NAME_SECOND: Name = Name::from("second");
  static ref NAME_START_POSITION: Name = Name::new(&["start", "position"]);
  static ref NAME_STRING: Name = Name::from("string");
  static ref NAME_TIME: Name = Name::from("time");
  static ref NAME_TO: Name = Name::from("to");
  static ref NAME_VALUE1: Name = Name::from("value1");
  static ref NAME_VALUE2: Name = Name::from("value2");
  static ref NAME_YEAR: Name = Name::from("year");
}

type NamedParameters = Value;

macro_rules! parameter_not_found {
  ($l:expr) => {{
    use dmntk_feel::value_null;
    value_null!(r"parameter '{}' not found", $l.to_string())
  }};
}

///
pub fn evaluate_bif(bif: Bif, parameters: &NamedParameters) -> Value {
  match bif {
    Bif::Abs => bif_abs(parameters),
    Bif::After => bif_after(parameters),
    Bif::All => bif_all(parameters),
    Bif::Any => bif_any(parameters),
    Bif::Append => bif_append(parameters),
    Bif::Before => bif_before(parameters),
    Bif::Ceiling => bif_ceiling(parameters),
    Bif::Coincides => bif_coincides(parameters),
    Bif::Concatenate => bif_concatenate(parameters),
    Bif::Contains => bif_contains(parameters),
    Bif::Count => bif_count(parameters),
    Bif::Date => bif_date(parameters),
    Bif::DateAndTime => bif_date_and_time(parameters),
    Bif::DayOfWeek => bif_day_of_week(parameters),
    Bif::DayOfYear => bif_day_of_year(parameters),
    Bif::Decimal => bif_decimal(parameters),
    Bif::DistinctValues => bif_distinct_values(parameters),
    Bif::Duration => bif_duration(parameters),
    Bif::During => bif_during(parameters),
    Bif::EndsWith => bif_ends_with(parameters),
    Bif::Even => bif_even(parameters),
    Bif::Exp => bif_exp(parameters),
    Bif::FinishedBy => bif_finished_by(parameters),
    Bif::Finishes => bif_finishes(parameters),
    Bif::Flatten => bif_flatten(parameters),
    Bif::Floor => bif_floor(parameters),
    Bif::GetEntries => bif_get_entries(parameters),
    Bif::GetValue => bif_get_value(parameters),
    Bif::Includes => bif_includes(parameters),
    Bif::IndexOf => bif_index_of(parameters),
    Bif::InsertBefore => bif_insert_before(parameters),
    Bif::Is => bif_is(parameters),
    Bif::ListContains => bif_list_contains(parameters),
    Bif::Log => bif_log(parameters),
    Bif::LoweCase => bif_lower_case(parameters),
    Bif::Matches => bif_matches(parameters),
    Bif::Max => bif_max(parameters),
    Bif::Mean => bif_mean(parameters),
    Bif::Meets => bif_meets(parameters),
    Bif::Median => bif_median(parameters),
    Bif::MetBy => bif_met_by(parameters),
    Bif::Min => bif_min(parameters),
    Bif::Mode => bif_mode(parameters),
    Bif::Modulo => bif_modulo(parameters),
    Bif::MonthOfYear => bif_month_of_year(parameters),
    Bif::Not => bif_not(parameters),
    Bif::Number => bif_number(parameters),
    Bif::Odd => bif_odd(parameters),
    Bif::Overlaps => bif_overlaps(parameters),
    Bif::OverlapsAfter => bif_overlaps_after(parameters),
    Bif::OverlapsBefore => bif_overlaps_before(parameters),
    Bif::Product => bif_product(parameters),
    Bif::Remove => bif_remove(parameters),
    Bif::Replace => bif_replace(parameters),
    Bif::Reverse => bif_reverse(parameters),
    Bif::Sort => bif_sort(parameters),
    Bif::Split => bif_split(parameters),
    Bif::Sqrt => bif_sqrt(parameters),
    Bif::StartedBy => bif_started_by(parameters),
    Bif::Starts => bif_starts(parameters),
    Bif::StartsWith => bif_starts_with(parameters),
    Bif::Stddev => bif_stddev(parameters),
    Bif::String => bif_string(parameters),
    Bif::StringLength => bif_string_length(parameters),
    Bif::Sublist => bif_sublist(parameters),
    Bif::Substring => bif_substring(parameters),
    Bif::SubstringAfter => bif_substring_after(parameters),
    Bif::SubstringBefore => bif_substring_before(parameters),
    Bif::Sum => bif_sum(parameters),
    Bif::Time => bif_time(parameters),
    Bif::Union => bif_union(parameters),
    Bif::UpperCase => bif_upper_case(parameters),
    Bif::WeekOfYear => bif_week_of_year(parameters),
    Bif::YearsAndMonthsDuration => bif_years_and_months_duration(parameters),
  }
}

fn bif_abs(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_N) {
    core::abs(value)
  } else {
    parameter_not_found!(NAME_N)
  }
}

fn bif_after(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT_1).zip(get_param(parameters, &NAME_POINT_2)) {
    core::after(value1, value2)
  } else if let Some(((value1, pos1), (value2, pos2))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    if pos1 > pos2 {
      core::after(value2, value1)
    } else {
      core::after(value1, value2)
    }
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::after(value1, value2)
  } else {
    value_null!("[named::after] invalid named parameters")
  }
}

fn bif_all(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::all(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_any(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::any(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

/// `append` built-in function, currently no version with named parameters, always returns `null`.
fn bif_append(_parameters: &NamedParameters) -> Value {
  value_null!("[named::append] this function has no version with named parameters")
}

fn bif_before(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT_1).zip(get_param(parameters, &NAME_POINT_2)) {
    core::before(value1, value2)
  } else if let Some(((value1, pos1), (value2, pos2))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    if pos1 > pos2 {
      core::before(value2, value1)
    } else {
      core::before(value1, value2)
    }
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::before(value1, value2)
  } else {
    value_null!("[named::before] invalid named parameters")
  }
}

fn bif_ceiling(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_N) {
    core::ceiling(value)
  } else {
    parameter_not_found!(NAME_N)
  }
}

/// `coincides` built-in function, named parameters.
fn bif_coincides(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT_1).zip(get_param(parameters, &NAME_POINT_2)) {
    core::coincides(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::coincides(value1, value2)
  } else {
    value_null!("[named::coincides] invalid named parameters")
  }
}

/// `concatenate` built-in function, currently no version with named parameters, always returns `null`.
fn bif_concatenate(_parameters: &NamedParameters) -> Value {
  value_null!("[named::concatenate] this function has no version with named parameters")
}

fn bif_contains(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((match_string_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::contains(input_string_value, match_string_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_count(parameters: &NamedParameters) -> Value {
  if let Some((list, _)) = get_param(parameters, &NAME_LIST) {
    core::count(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_date(parameters: &NamedParameters) -> Value {
  if let Some((from, _)) = get_param(parameters, &NAME_FROM) {
    return core::date_1(from);
  }
  if let Some((year, _)) = get_param(parameters, &NAME_YEAR) {
    if let Some((month, _)) = get_param(parameters, &NAME_MONTH) {
      if let Some((day, _)) = get_param(parameters, &NAME_DAY) {
        return core::date_3(year, month, day);
      }
    }
  }
  value_null!("invalid parameters in named::bif_date")
}

fn bif_date_and_time(parameters: &NamedParameters) -> Value {
  if let Some((from_value, _)) = get_param(parameters, &NAME_FROM) {
    return core::date_and_time_1(from_value);
  }
  if let Some((date_value, _)) = get_param(parameters, &NAME_DATE) {
    if let Some((time_value, _)) = get_param(parameters, &NAME_TIME) {
      return core::date_and_time_2(date_value, time_value);
    }
  }
  value_null!("invalid parameters in named::bif_date_and_time")
}

fn bif_day_of_week(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::day_of_week(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_day_of_year(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::day_of_year(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_decimal(parameters: &NamedParameters) -> Value {
  if let Some((number, _)) = get_param(parameters, &NAME_N) {
    if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
      core::decimal(number, scale)
    } else {
      parameter_not_found!(NAME_SCALE)
    }
  } else {
    parameter_not_found!(NAME_N)
  }
}

fn bif_distinct_values(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_LIST) {
    core::distinct_values(value)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_duration(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_FROM) {
    core::duration(value)
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

fn bif_during(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::during(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::during(value1, value2)
  } else {
    value_null!("[named::during] invalid named parameters")
  }
}

fn bif_ends_with(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((match_string_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::ends_with(input_string_value, match_string_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_even(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::even(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_exp(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::exp(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_finished_by(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::finished_by(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::finished_by(value1, value2)
  } else {
    value_null!("[named::finished by] invalid named parameters")
  }
}

fn bif_finishes(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::finishes(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::finishes(value1, value2)
  } else {
    value_null!("[named::finishes] invalid named parameters")
  }
}

fn bif_flatten(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_LIST) {
    core::flatten(value)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_floor(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_N) {
    core::floor(value)
  } else {
    parameter_not_found!(NAME_N)
  }
}

fn bif_get_entries(parameters: &NamedParameters) -> Value {
  if let Some((context, _)) = get_param(parameters, &NAME_M) {
    core::get_entries(context)
  } else {
    parameter_not_found!(NAME_M)
  }
}

fn bif_get_value(parameters: &NamedParameters) -> Value {
  if let Some((context, _)) = get_param(parameters, &NAME_M) {
    if let Some((key, _)) = get_param(parameters, &NAME_KEY) {
      core::get_value(context, key)
    } else {
      parameter_not_found!(NAME_KEY)
    }
  } else {
    parameter_not_found!(NAME_M)
  }
}

fn bif_includes(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::includes(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::includes(value1, value2)
  } else {
    value_null!("[named::includes] invalid named parameters")
  }
}

fn bif_index_of(parameters: &NamedParameters) -> Value {
  if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((match_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::index_of(list_value, match_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_insert_before(parameters: &NamedParameters) -> Value {
  if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((position_value, _)) = get_param(parameters, &NAME_POSITION) {
      if let Some((new_item_value, _)) = get_param(parameters, &NAME_NEW_ITEM) {
        core::insert_before(list_value, position_value, new_item_value)
      } else {
        parameter_not_found!(NAME_NEW_ITEM)
      }
    } else {
      parameter_not_found!(NAME_POSITION)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_is(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_VALUE1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_VALUE2) {
      core::is(value1, value2)
    } else {
      parameter_not_found!(NAME_VALUE2)
    }
  } else {
    parameter_not_found!(NAME_VALUE1)
  }
}

fn bif_list_contains(parameters: &NamedParameters) -> Value {
  if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((match_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::list_contains(list_value, match_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_log(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::log(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_lower_case(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    core::lower_case(input_string_value)
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_matches(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_INPUT) {
    if let Some((pattern_string_value, _)) = get_param(parameters, &NAME_PATTERN) {
      if let Some((flags_string_value, _)) = get_param(parameters, &NAME_FLAGS) {
        core::matches(input_string_value, pattern_string_value, flags_string_value)
      } else {
        core::matches(input_string_value, pattern_string_value, &value_null!())
      }
    } else {
      parameter_not_found!(NAME_PATTERN)
    }
  } else {
    parameter_not_found!(NAME_INPUT)
  }
}

fn bif_max(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::max(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_mean(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::mean(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_meets(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::meets(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_median(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::median(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_met_by(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::met_by(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_min(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::min(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_modulo(parameters: &NamedParameters) -> Value {
  if let Some((dividend, _)) = get_param(parameters, &NAME_DIVIDEND) {
    if let Some((divisor, _)) = get_param(parameters, &NAME_DIVISOR) {
      core::modulo(dividend, divisor)
    } else {
      parameter_not_found!(NAME_DIVISOR)
    }
  } else {
    parameter_not_found!(NAME_DIVIDEND)
  }
}

fn bif_month_of_year(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::month_of_year(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_mode(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::mode(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_not(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NEGAND) {
    core::not(value)
  } else {
    parameter_not_found!(NAME_NEGAND)
  }
}

fn bif_number(parameters: &NamedParameters) -> Value {
  if let Some((from, _)) = get_param(parameters, &NAME_FROM) {
    if let Some((grouping_separator, _)) = get_param(parameters, &NAME_GROUPING_SEPARATOR) {
      if let Some((decimal_separator, _)) = get_param(parameters, &NAME_DECIMAL_SEPARATOR) {
        core::number(from, grouping_separator, decimal_separator)
      } else {
        parameter_not_found!(NAME_DECIMAL_SEPARATOR)
      }
    } else {
      parameter_not_found!(NAME_GROUPING_SEPARATOR)
    }
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

fn bif_odd(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::odd(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_overlaps(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::overlaps(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_overlaps_after(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::overlaps_after(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_overlaps_before(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::overlaps_after(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_product(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::product(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_remove(parameters: &NamedParameters) -> Value {
  if let Some((list, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((position, _)) = get_param(parameters, &NAME_POSITION) {
      core::remove(list, position)
    } else {
      parameter_not_found!(NAME_POSITION)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_replace(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_INPUT) {
    if let Some((pattern_string_value, _)) = get_param(parameters, &NAME_PATTERN) {
      if let Some((replacement_string_value, _)) = get_param(parameters, &NAME_REPLACEMENT) {
        if let Some((flags_string_value, _)) = get_param(parameters, &NAME_FLAGS) {
          core::replace(input_string_value, pattern_string_value, replacement_string_value, flags_string_value)
        } else {
          core::replace(input_string_value, pattern_string_value, replacement_string_value, &value_null!())
        }
      } else {
        parameter_not_found!(NAME_REPLACEMENT)
      }
    } else {
      parameter_not_found!(NAME_PATTERN)
    }
  } else {
    parameter_not_found!(NAME_INPUT)
  }
}

///
fn bif_reverse(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_LIST) {
    core::reverse(value)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

///
fn bif_sort(parameters: &NamedParameters) -> Value {
  if let Some((list, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((ordering_function, _)) = get_param(parameters, &NAME_PRECEDES) {
      core::sort(list, ordering_function)
    } else {
      parameter_not_found!(NAME_PRECEDES)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

///
fn bif_split(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((delimiter_string_value, _)) = get_param(parameters, &NAME_DELIMITER) {
      core::split(input_string_value, delimiter_string_value)
    } else {
      parameter_not_found!(NAME_DELIMITER)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_sqrt(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::sqrt(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_started_by(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::started_by(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::started_by(value1, value2)
  } else {
    value_null!("[named::started by] invalid named parameters")
  }
}

fn bif_starts(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::starts(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::starts(value1, value2)
  } else {
    value_null!("[named::starts] invalid named parameters")
  }
}

fn bif_starts_with(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((match_string_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::starts_with(input_string_value, match_string_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_stddev(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::stddev(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_string(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_FROM) {
    core::string(value)
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

fn bif_string_length(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    core::string_length(input_string_value)
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_sublist(parameters: &NamedParameters) -> Value {
  if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((start_position_value, _)) = get_param(parameters, &NAME_START_POSITION) {
      if let Some((length_value, _)) = get_param(parameters, &NAME_LENGTH) {
        core::sublist3(list_value, start_position_value, length_value)
      } else {
        core::sublist2(list_value, start_position_value)
      }
    } else {
      parameter_not_found!(NAME_START_POSITION)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_substring(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((start_position_value, _)) = get_param(parameters, &NAME_START_POSITION) {
      if let Some((length_value, _)) = get_param(parameters, &NAME_LENGTH) {
        core::substring(input_string_value, start_position_value, length_value)
      } else {
        core::substring(input_string_value, start_position_value, &value_null!())
      }
    } else {
      parameter_not_found!(NAME_START_POSITION)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_substring_after(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((input_string_match, _)) = get_param(parameters, &NAME_MATCH) {
      core::substring_after(input_string_value, input_string_match)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_substring_before(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((input_string_match, _)) = get_param(parameters, &NAME_MATCH) {
      core::substring_before(input_string_value, input_string_match)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_sum(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::sum(list.as_vec())
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_time(parameters: &NamedParameters) -> Value {
  if let Some((from_value, _)) = get_param(parameters, &NAME_FROM) {
    return core::time_1(from_value);
  }
  if let Some((hour_value, _)) = get_param(parameters, &NAME_HOUR) {
    if let Some((minute_value, _)) = get_param(parameters, &NAME_MINUTE) {
      if let Some((second_value, _)) = get_param(parameters, &NAME_SECOND) {
        return if let Some((offset_value, _)) = get_param(parameters, &NAME_OFFSET) {
          core::time_4(hour_value, minute_value, second_value, offset_value)
        } else {
          core::time_3(hour_value, minute_value, second_value)
        };
      }
    }
  }
  value_null!("invalid parameters in bif time")
}

fn bif_union(_parameters: &NamedParameters) -> Value {
  value_null!("[named::union] this function has no version with named parameters")
}

fn bif_upper_case(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    core::upper_case(input_string_value)
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_week_of_year(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::week_of_year(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_years_and_months_duration(parameters: &NamedParameters) -> Value {
  if let Some((from_value, _)) = get_param(parameters, &NAME_FROM) {
    if let Some((to_value, _)) = get_param(parameters, &NAME_TO) {
      core::years_and_months_duration(from_value, to_value)
    } else {
      parameter_not_found!(NAME_TO)
    }
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

/// Returns reference to the value of the named parameter with specified name.
/// Checks all names given in parameter `names`. The value of the first name found is returned.
fn get_param<'a>(parameters: &'a NamedParameters, name: &'a Name) -> Option<(&'a Value, &'a usize)> {
  if let Value::NamedParameters(map) = parameters {
    if let Some((value, position)) = map.get(name) {
      return Some((value, position));
    }
  }
  None
}
