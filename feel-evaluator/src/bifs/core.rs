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

//! Core implementation of build-in functions.

use crate::evaluate_equals;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value::YearsAndMonthsDuration;
use dmntk_feel::values::{Value, Values, VALUE_FALSE, VALUE_TRUE};
use dmntk_feel::{value_null, FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelNumber, FeelTime, FeelYearsAndMonthsDuration, Name, Scope, ToFeelString};
use regex::Regex;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::convert::TryFrom;

/// Returns the absolute value of the argument.
pub fn abs(value: &Value) -> Value {
  if let Value::Number(v) = value {
    Value::Number(v.abs())
  } else {
    value_null!("invalid argument type, expected number, actual type is {}", value.type_of())
  }
}

/// Returns `false` if any item is `false`, `true` if empty or all items are true, else `null`.
pub fn all(values: &[Value]) -> Value {
  if values.is_empty() {
    return VALUE_TRUE;
  }
  for value in values {
    if let Value::Boolean(v) = value {
      if !v {
        return VALUE_FALSE;
      }
    } else {
      return value_null!();
    }
  }
  VALUE_TRUE
}

/// ???
pub fn any(values: &[Value]) -> Value {
  if values.is_empty() {
    return VALUE_FALSE;
  }
  let mut has_true = false;
  let mut all_false = true;
  for value in values {
    if let Value::Boolean(v) = value {
      if *v {
        has_true = true;
      }
    } else {
      all_false = false;
    }
  }
  match (has_true, all_false) {
    (false, false) => value_null!(),
    (false, true) => VALUE_FALSE,
    (true, false) => value_null!(),
    (true, true) => VALUE_TRUE,
  }
}

///
pub fn append(list: &Value, values: &[Value]) -> Value {
  if let Value::List(items) = list {
    let mut appended = items.clone();
    for value in values {
      appended.add(value.clone());
    }
    return Value::List(appended);
  }
  value_null!("append")
}

///
pub fn before(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(point1) => match value2 {
      Value::Number(point2) => return Value::Boolean(point1 < point2),
      Value::Range(range_start2, closed_start2, _, _) => {
        if let Value::Number(start2) = range_start2.borrow() {
          return Value::Boolean(point1 < start2 || (point1 == start2 && !*closed_start2));
        }
      }
      _ => {}
    },
    Value::Range(_, _, range_end1, closed_end1) => match range_end1.borrow() {
      Value::Number(end1) => match value2 {
        Value::Number(point2) => return Value::Boolean(end1 < point2 || (end1 == point2 && !*closed_end1)),
        Value::Range(range_start2, closed_start2, _, _) => {
          if let Value::Number(start2) = range_start2.borrow() {
            return Value::Boolean(end1 < start2 || (end1 == start2 && (!*closed_end1 || !*closed_start2)));
          }
        }
        _ => {}
      },
      Value::Date(_end1) => match value2 {
        Value::Date(_point2) => return Value::Boolean(false), //FIXME add operators to dates and then fix this case
        Value::Range(range_start2, _closed_start2, _, _) => {
          if let Value::Date(_start2) = range_start2.borrow() {
            return Value::Boolean(false); //FIXME add operators to dates and then fix this case
          }
        }
        _ => {}
      },
      _ => {}
    },
    _ => {}
  }
  value_null!()
}

/// ???
pub fn ceiling(value: &Value) -> Value {
  if let Value::Number(v) = value {
    Value::Number(v.ceil())
  } else {
    value_null!("ceiling")
  }
}

/// ???
pub fn concatenate(values: &[Value]) -> Value {
  let mut concatenated = vec![];
  for value in values {
    if let Value::List(items) = value {
      for item in items.as_vec() {
        concatenated.push(item.clone());
      }
    } else {
      value_null!("concatenate");
    }
  }
  Value::List(Values::new(concatenated))
}

/// Returns `true` when the input string contains the match.
pub fn contains(input_string_value: &Value, match_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_string_value {
      Value::Boolean(input_string.contains(match_string))
    } else {
      value_null!("contains")
    }
  } else {
    value_null!("contains")
  }
}

/// ???
pub fn count(list: &Value) -> Value {
  if let Value::List(items) = list {
    Value::Number(items.as_vec().len().into())
  } else {
    value_null!("parameter is not a list")
  }
}

/// ???
pub fn date_1(value: &Value) -> Value {
  match value {
    Value::String(text) => {
      if let Ok(date) = FeelDate::try_from(text.as_str()) {
        Value::Date(date)
      } else {
        value_null!("date_1 1")
      }
    }
    Value::Date(date) => Value::Date(date.clone()),
    Value::DateTime(date_time) => Value::Date(date_time.date()),
    _ => value_null!("date_1 2: {}", value),
  }
}

/// ???
pub fn date_3(year_value: &Value, month_value: &Value, day_value: &Value) -> Value {
  if let Value::Number(year) = year_value {
    if let Value::Number(month) = month_value {
      if let Value::Number(day) = day_value {
        if let Ok(date) = FeelDate::try_from((*year, *month, *day)) {
          Value::Date(date)
        } else {
          value_null!("date_3 1")
        }
      } else {
        value_null!("date_3 2")
      }
    } else {
      value_null!("date_3 3")
    }
  } else {
    value_null!("date_3 4")
  }
}

/// ???
pub fn date_and_time_1(value: &Value) -> Value {
  if let Value::String(text) = value {
    if let Ok(date_time) = FeelDateTime::try_from(text.as_str()) {
      return Value::DateTime(date_time);
    }
    if let Ok(date) = FeelDate::try_from(text.as_str()) {
      return Value::DateTime(FeelDateTime::new(date, FeelTime::local(0, 0, 0, 0)));
    }
  }
  value_null!("date_and_time")
}

/// ???
pub fn date_and_time_2(date_value: &Value, time_value: &Value) -> Value {
  match date_value {
    Value::DateTime(date_time) => {
      if let Value::Time(time) = time_value {
        return Value::DateTime(FeelDateTime::new(date_time.date(), time.clone()));
      }
    }
    Value::Date(date) => {
      if let Value::Time(time) = time_value {
        return Value::DateTime(FeelDateTime::new(date.clone(), time.clone()));
      }
    }
    _ => {}
  }
  value_null!("date_and_time_1")
}

/// Returns `number` rounded to given `scale`.
pub fn decimal(number: &Value, scale: &Value) -> Value {
  if let Value::Number(n) = number {
    if let Value::Number(s) = scale {
      let sc: isize = (*s).into();
      if (-6111..6176).contains(&sc) {
        Value::Number((*n).round(sc.into()))
      } else {
        value_null!("decimal")
      }
    } else {
      value_null!("decimal")
    }
  } else {
    value_null!("decimal")
  }
}

/// ???
pub fn distinct_values(value: &Value) -> Value {
  let mut result = vec![];
  if let Value::List(items) = value {
    for item in items.as_vec() {
      if result.iter().all(|a| !evaluate_equals(a, item)) {
        result.push(item.clone())
      }
    }
  } else {
    return value_null!("distinct_values");
  }
  Value::List(Values::new(result))
}

/// Converts string value to a days and time or years and months duration.
pub fn duration(value: &Value) -> Value {
  if let Value::String(s) = value {
    if let Ok(ym_duration) = FeelYearsAndMonthsDuration::try_from(s.as_str()) {
      Value::YearsAndMonthsDuration(ym_duration)
    } else if let Ok(dt_duration) = FeelDaysAndTimeDuration::try_from(s.as_str()) {
      Value::DaysAndTimeDuration(dt_duration)
    } else {
      value_null!("duration")
    }
  } else {
    value_null!("duration")
  }
}

/// Returns **true** when the input string ends with specified match string.
pub fn ends_with(input_string_value: &Value, match_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_string_value {
      Value::Boolean(input_string.ends_with(match_string))
    } else {
      value_null!("ends_with")
    }
  } else {
    value_null!("ends_with")
  }
}

/// Returns true if number is even, false if it is odd.
pub fn even(value: &Value) -> Value {
  if let Value::Number(v) = value {
    Value::Boolean(v.even())
  } else {
    value_null!("even")
  }
}

/// Returns the Euler’s number e raised to the power of **value** given as a parameter.
pub fn exp(value: &Value) -> Value {
  if let Value::Number(num) = value {
    if let Some(num_exp) = num.exp() {
      return Value::Number(num_exp);
    }
  }
  value_null!()
}

/// ???
pub fn flatten(value: &Value) -> Value {
  if let Value::List(_) = value {
    let mut flattened = vec![];
    flatten_value(value, &mut flattened);
    return Value::List(Values::new(flattened));
  }
  value_null!("flatten")
}

/// ???
fn flatten_value(value: &Value, flattened: &mut Vec<Value>) {
  if let Value::List(items) = value {
    for item in items.as_vec() {
      if let Value::List(_) = item {
        flatten_value(item, flattened);
      } else {
        flattened.push(item.clone())
      }
    }
  }
}

/// Returns greatest **integer** <= **value** specified as a parameter.
pub fn floor(value: &Value) -> Value {
  if let Value::Number(v) = value {
    Value::Number(v.floor())
  } else {
    value_null!("floor")
  }
}

///
pub fn get_entries(context: &Value) -> Value {
  if let Value::Context(ctx) = context {
    let name_key: Name = "key".into();
    let name_value: Name = "value".into();
    let mut entries = vec![];
    ctx.get_entries().iter().for_each(|(name, value)| {
      let mut key_value_pair = FeelContext::default();
      key_value_pair.set_entry(&name_key, Value::String(name.to_string()));
      key_value_pair.set_entry(&name_value, (**value).clone());
      entries.push(Value::Context(key_value_pair));
    });
    Value::List(Values::new(entries))
  } else {
    value_null!()
  }
}

///
pub fn get_value(context: &Value, key: &Value) -> Value {
  if let Value::Context(ctx) = context {
    if let Value::String(entry_key) = key {
      let name = Name::from(entry_key.to_owned());
      if let Some(entry_value) = ctx.get_entry(&name) {
        return entry_value.clone();
      }
    }
  }
  value_null!()
}

/// ???
pub fn index_of(list: &Value, element: &Value) -> Value {
  if let Value::List(items) = list {
    let mut indexes = vec![];
    for (i, item) in items.as_vec().iter().enumerate() {
      if evaluate_equals(item, element) {
        indexes.push(Value::Number(i.into()));
      }
    }
    return Value::List(Values::new(indexes));
  }
  value_null!("index_of")
}

/// ???
pub fn insert_before(list: &Value, position: &Value, new_item: &Value) -> Value {
  if let Value::List(mut items) = list.clone() {
    if let Value::Number(pos) = position {
      let index: i64 = (*pos).into();
      if index > 0 {
        let i = (index as usize) - 1;
        if i < items.len() {
          items.insert(i, new_item.clone());
          return Value::List(items);
        }
      }
      if index < 0 {
        let i = index.abs() as usize;
        if i <= items.as_vec().len() {
          items.insert(items.len() - i, new_item.clone());
          return Value::List(items);
        }
      }
    }
  }
  value_null!("probably index is out of range")
}

/// ???
pub fn list_contains(list: &Value, element: &Value) -> Value {
  if let Value::List(items) = list {
    for item in items.as_vec() {
      if evaluate_equals(item, element) {
        return VALUE_TRUE;
      }
    }
  }
  VALUE_FALSE
}

/// Returns the natural logarithm (base **e**) of the number parameter.
pub fn log(number: &Value) -> Value {
  if let Value::Number(num) = number {
    if *num > FeelNumber::zero() {
      if let Some(num_log) = num.ln() {
        return Value::Number(num_log);
      }
    }
  }
  value_null!()
}

/// Returns lower-cased string.
pub fn lower_case(input_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    Value::String(input_string.to_lowercase().trim().to_string())
  } else {
    value_null!("lower_case")
  }
}

/// Returns `true` when the input matches the regexp pattern.
pub fn matches(input_string_value: &Value, pattern_string_value: &Value, flags_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(pattern_string) = pattern_string_value {
      if let Value::String(flags_string) = flags_string_value {
        if let Ok(re) = Regex::new(format!("(?{}){}", flags_string, pattern_string).as_str()) {
          return Value::Boolean(re.is_match(input_string));
        }
      } else if let Ok(re) = Regex::new(pattern_string) {
        return Value::Boolean(re.is_match(input_string));
      }
    }
  }
  value_null!("matches")
}

/// Returns the maximum value in the collection of comparable values.
pub fn max(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  return match &values[0] {
    Value::Number(n) => {
      let mut max = *n;
      for value in values.iter().skip(1) {
        match value {
          Value::Number(v) => {
            if *v > max {
              max = *v;
            }
          }
          Value::Null(_) => {}
          other => {
            return value_null!("max: expected value of type number or Null, but encountered: {:?}", other.type_of());
          }
        }
      }
      Value::Number(max)
    }
    Value::String(s) => {
      let mut max = s.clone();
      for value in values.iter().skip(1) {
        match value {
          Value::String(v) => {
            if *v > max {
              max = v.clone();
            }
          }
          Value::Null(_) => {}
          other => {
            return value_null!("max: expected value of type string or Null, but encountered: {:?}", other.type_of());
          }
        }
      }
      Value::String(max)
    }
    other => {
      value_null!("max: unhandled value type: {:?}", other.type_of())
    }
  };
}

/// Returns the mean of numbers.
pub fn mean(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  let mut sum = FeelNumber::zero();
  for value in values {
    if let Value::Number(n) = value {
      sum += *n;
    } else {
      return value_null!("not a number");
    }
  }
  Value::Number(sum / values.len().into())
}

/// Returns the median of numbers.
pub fn median(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  let mut list = vec![];
  for value in values {
    if let Value::Number(n) = value {
      list.push(*n);
    } else {
      return value_null!("median");
    }
  }
  list.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
  let index = values.len() / 2;
  if list.len() % 2 == 0 {
    Value::Number((list[index - 1] + list[index]) / FeelNumber::two())
  } else {
    Value::Number(list[index])
  }
}

/// Returns the minimum value in the collection of comparable values.
pub fn min(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  match &values[0] {
    Value::Number(n) => {
      let mut min = *n;
      for value in values.iter().skip(1) {
        if let Value::Number(v) = value {
          if *v < min {
            min = *v;
          }
        } else {
          return value_null!("min");
        }
      }
      return Value::Number(min);
    }
    Value::String(s) => {
      let mut min = s.clone();
      for value in values.iter().skip(1) {
        if let Value::String(v) = value {
          if *v < min {
            min = v.clone();
          }
        } else {
          return value_null!("min");
        }
      }
      return Value::String(min);
    }
    _ => {}
  }
  value_null!("min")
}

/// Returns the mode of numbers.
pub fn mode(values: &[Value]) -> Value {
  if values.is_empty() {
    return Value::List(Values::default());
  }
  // make sure all values are numbers and prepare the list of numbers
  let mut list = vec![];
  for value in values {
    if let Value::Number(n) = value {
      list.push(*n);
    } else {
      return value_null!("mode");
    }
  }
  // sort values in ascending order
  list.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
  // calculate the frequencies of the numbers
  let mut mode: Vec<(usize, FeelNumber)> = vec![];
  for x in list {
    if let Some((count, value)) = mode.pop() {
      if x == value {
        mode.push((count + 1, value));
      } else {
        mode.push((count, value));
        mode.push((1_usize, x));
      }
    } else {
      mode.push((1_usize, x));
    }
  }
  // sort frequencies in descending order, and when equal then by number in ascending order
  mode.sort_by(|x, y| match x.0.cmp(&y.0).reverse() {
    std::cmp::Ordering::Equal => x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal),
    other => other,
  });
  // there must be minimum one element in the list but to be sure check it
  if let Some((max, _)) = mode.get(0) {
    // return items with maximum frequency
    return Value::List(Values::new(
      mode
        .iter()
        .filter_map(|(c, v)| if *c == *max { Some(Value::Number(*v)) } else { None })
        .collect(),
    ));
  }
  value_null!("mode")
}

/// Returns the remainder of the division of dividend by divisor.
pub fn modulo(dividend_value: &Value, divisor_value: &Value) -> Value {
  if let Value::Number(dividend) = *dividend_value {
    if let Value::Number(divisor) = *divisor_value {
      if divisor.abs() == FeelNumber::zero() {
        value_null!("division by zero")
      } else {
        Value::Number(dividend - divisor * (dividend / divisor).floor())
      }
    } else {
      value_null!("modulo")
    }
  } else {
    value_null!("modulo")
  }
}

/// Logical negation.
pub fn not(negand: &Value) -> Value {
  if let Value::Boolean(v) = negand {
    Value::Boolean(!(*v))
  } else {
    value_null!("not")
  }
}

/// Converts string to a number.
/// Grouping...
pub fn number(from: &Value, grouping_separator: &Value, decimal_separator: &Value) -> Value {
  // function for converting string to Value::Number
  let convert = |value: String| match value.parse::<FeelNumber>() {
    Ok(number) => Value::Number(number),
    Err(reason) => value_null!("number: {}", reason),
  };
  match from {
    Value::String(value) => {
      // prepare grouping separator from Value::String ot VALUE_NULL
      let grouping_sep = match grouping_separator {
        Value::String(s) => match s.as_str() {
          " " | "." | "," => Some((*s).clone()),
          _ => {
            return value_null!();
          }
        },
        Value::Null(_) => None,
        _ => {
          return value_null!("number");
        }
      };
      // prepare decimal separator from Value::String ot VALUE_NULL
      let decimal_sep = match decimal_separator {
        Value::String(s) => match s.as_str() {
          "." | "," => Some((*s).clone()),
          _ => {
            return value_null!();
          }
        },
        Value::Null(_) => None,
        _ => {
          return value_null!("number");
        }
      };
      // replace both separators and try to convert
      if let Some(grp_sep) = &grouping_sep {
        if let Some(dec_sep) = &decimal_sep {
          return if *grp_sep != *dec_sep {
            convert(value.replace(grp_sep, "").replace(dec_sep, "."))
          } else {
            value_null!()
          };
        }
      }
      // replace grouping separator and try to convert
      if decimal_sep.is_none() {
        if let Some(sep) = grouping_sep {
          return convert(value.replace(&sep, ""));
        }
      }
      // replace decimal separator and try to convert
      if grouping_sep.is_none() {
        if let Some(sep) = decimal_sep {
          return convert(value.replace(&sep, "."));
        }
      }
      // try to convert an input parameter without replacing
      convert(value.clone())
    }
    _ => {
      value_null!("number")
    }
  }
}

/// Returns **true** if number is odd, **false** if it is even.
pub fn odd(value: &Value) -> Value {
  if let Value::Number(v) = value {
    Value::Boolean(v.odd())
  } else {
    value_null!("odd")
  }
}

/// ???
pub fn remove(list: &Value, position: &Value) -> Value {
  if let Value::List(mut items) = list.clone() {
    if let Value::Number(pos) = position {
      let index: i64 = (*pos).into();
      if index > 0 {
        let i = (index as usize) - 1;
        if i < items.as_vec().len() {
          items.remove(i);
          return Value::List(items);
        }
      }
      if index < 0 {
        let i = index.abs() as usize;
        if i <= items.len() {
          items.remove(items.len() - i);
          return Value::List(items);
        }
      }
    }
  }
  value_null!("probably index is out of range")
}

/// ???
pub fn replace(input_string_value: &Value, pattern_string_value: &Value, replacement_string_value: &Value, flags_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(pattern_string) = pattern_string_value {
      if let Value::String(replacement_string) = replacement_string_value {
        // Rust implementation is eager when parsing matching groups, so place numbers in square brackets
        let repl = if let Ok(rg) = Regex::new("\\$([1-9][0-9]*)") {
          rg.replace_all(replacement_string.as_str(), "$${${1}}").to_string()
        } else {
          replacement_string.clone()
        };
        // check and use flags
        if let Value::String(flags_string) = flags_string_value {
          let mut flags = "".to_string();
          let mut flag_q = false;
          let mut clear_flag_q = false;
          for ch in flags_string.chars() {
            if ch == 'q' {
              flag_q = true;
            }
            if matches!(ch, 's' | 'm' | 'i' | 'x') {
              flags.push(ch);
              if ch != 'i' {
                clear_flag_q = true;
              }
            }
          }
          if clear_flag_q {
            flag_q = false;
          }
          let mut patt = "".to_string();
          for ch in pattern_string.chars() {
            if flag_q {
              patt.push('\\');
            }
            patt.push(ch);
          }
          if flags.is_empty() {
            if let Ok(re) = Regex::new(&patt) {
              let result = re.replace_all(input_string.as_str(), repl.as_str()).trim().to_string();
              return Value::String(result);
            }
          } else if let Ok(re) = Regex::new(format!("(?{}){}", flags, patt).as_str()) {
            let result = re.replace_all(input_string.as_str(), repl.as_str()).trim().to_string();
            return Value::String(result);
          }
        }
        // replace without any flags
        if let Ok(re) = Regex::new(pattern_string) {
          let result = re.replace_all(input_string.as_str(), repl.as_str()).trim().to_string();
          return Value::String(result);
        }
      }
    }
  }
  value_null!("replace")
}

///
pub fn reverse(list: &Value) -> Value {
  if let Value::List(mut items) = list.clone() {
    items.reverse();
    return Value::List(items);
  }
  value_null!("reverse")
}

///
pub fn sort(list: &Value, ordering_function: &Value) -> Value {
  if let Value::List(items) = list.clone() {
    if let Value::FunctionDefinition(parameters, body, _) = ordering_function {
      if parameters.len() == 2 {
        let mut elements = items.as_vec().clone();
        elements.sort_by(|x, y| {
          let mut ctx = FeelContext::default();
          ctx.set_entry(&parameters[0].0, x.clone());
          ctx.set_entry(&parameters[1].0, y.clone());
          let scope: Scope = ctx.into();
          if let Value::Boolean(result) = body.evaluate(&scope) {
            if result {
              Ordering::Less
            } else {
              Ordering::Equal
            }
          } else {
            Ordering::Equal
          }
        });
        Value::List(Values::new(elements))
      } else {
        value_null!("sort: ordering function should take exactly two arguments")
      }
    } else {
      value_null!("sort: expected ordering function definition as a second argument")
    }
  } else {
    value_null!("sort: expected a list of values as a first argument")
  }
}

///
pub fn split(input_string_value: &Value, delimiter_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(delimiter_string) = delimiter_string_value {
      if let Ok(re) = Regex::new(delimiter_string) {
        return Value::List(Values::new(re.split(input_string).map(|s| Value::String(s.to_string())).collect()));
      }
    }
  }
  value_null!("split")
}

/// Returns the square root of the given **value** specified as a parameter.
/// If the given number is negative it returns [VALUE_NULL].
pub fn sqrt(value: &Value) -> Value {
  if let Value::Number(v) = value {
    if *v >= FeelNumber::zero() {
      if let Some(result) = v.sqrt() {
        Value::Number(result)
      } else {
        value_null!("?1")
      }
    } else {
      value_null!("?2")
    }
  } else {
    value_null!("sqrt")
  }
}

/// Returns **true** when the input string starts with specified match string.
pub fn starts_with(input_string_value: &Value, match_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_string_value {
      Value::Boolean(input_string.starts_with(match_string))
    } else {
      value_null!("starts_with")
    }
  } else {
    value_null!("starts_with")
  }
}

///
pub fn stddev(values: &[Value]) -> Value {
  if values.len() < 2 {
    return value_null!();
  }
  let mut sum = FeelNumber::zero();
  let mut numbers = vec![];
  for value in values {
    if let Value::Number(x) = *value {
      sum += x;
      numbers.push(x);
    } else {
      return value_null!("stddev");
    }
  }
  let n: FeelNumber = numbers.len().into();
  let avg = sum / n;
  let mut sum2 = FeelNumber::zero();
  for number in numbers {
    sum2 += (number - avg).pow2();
  }
  if let Some(stddev) = (sum2 / (n - FeelNumber::one())).sqrt() {
    Value::Number(stddev)
  } else {
    value_null!("stddev")
  }
}

/// Converts specified value to [Value::String].
pub fn string(value: &Value) -> Value {
  match value {
    Value::Null(_) => value_null!(),
    Value::String(s) => Value::String(s.clone()),
    other => Value::String(other.to_feel_string()),
  }
}

/// Returns the number of characters in string.
pub fn string_length(input_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    Value::Number(input_string.chars().count().into())
  } else {
    value_null!("string_length")
  }
}

/// Returns the sum of values in the collection of numbers.
pub fn sum(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  if let Value::Number(n) = values[0] {
    let mut sum = n;
    for value in values.iter().skip(1) {
      if let Value::Number(v) = *value {
        sum += v;
      } else {
        return value_null!("sum");
      }
    }
    return Value::Number(sum);
  }
  value_null!("sum")
}

/// ???
pub fn sublist2(list: &Value, position: &Value) -> Value {
  if let Value::List(items) = list {
    if let Value::Number(position) = position {
      let position: i64 = (*position).into();
      if position > 0 {
        let index = (position as usize) - 1;
        if index < items.len() {
          return Value::List(Values::new(items.as_vec()[index..].to_vec()));
        }
      }
      if position < 0 {
        let index = position.abs() as usize;
        if index <= items.len() {
          return Value::List(Values::new(items.as_vec()[items.len() - index..].to_vec()));
        }
      }
    }
  }
  value_null!("probably index is out of range")
}

/// ???
pub fn sublist3(list: &Value, position: &Value, length: &Value) -> Value {
  if let Value::List(items) = list {
    if let Value::Number(length) = length {
      let length: i64 = (*length).into();
      if length > 0 {
        if let Value::Number(position) = position {
          let position: i64 = (*position).into();
          if position > 0 {
            let first = (position as usize) - 1;
            let last = first + (length as usize);
            if first < items.len() && last <= items.len() {
              return Value::List(Values::new(items.as_vec()[first..last].to_vec()));
            }
          }
          if position < 0 {
            let first = items.len() - (position.abs() as usize);
            let last = first + (length as usize);
            if first < items.len() && last <= items.len() {
              return Value::List(Values::new(items.as_vec()[first..last].to_vec()));
            }
          }
        }
      }
    }
  }
  value_null!("probably index is out of range")
}

/// Returns `length` (or all) characters from string, starting at
/// `start_position`. First position is 1, last position is -1.
pub fn substring(input_string_value: &Value, start_position_value: &Value, length_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::Number(start_position) = start_position_value {
      let start: isize = (*start_position).into();
      let input_string_len = input_string.chars().count();
      match length_value {
        Value::Number(length) => {
          if *length < FeelNumber::one() {
            return value_null!();
          }
          let count: usize = (*length).into();
          if start > 0 {
            let index = (start - 1) as usize;
            if index < input_string_len && index + count <= input_string_len {
              return Value::String(input_string.chars().skip(index).take(count).collect());
            }
          }
          if start < 0 {
            let index = (input_string_len as isize) + start;
            if index >= 0 && index as usize + count <= input_string_len {
              return Value::String(input_string.chars().skip(index as usize).take(count).collect());
            }
          }
          value_null!()
        }
        Value::Null(_) => {
          if start > 0 {
            let index = (start - 1) as usize;
            if index < input_string_len {
              return Value::String(input_string.chars().skip(index).collect());
            }
          }
          if start < 0 {
            let index = (input_string_len as isize) + start;
            if index >= 0 {
              return Value::String(input_string.chars().skip(index as usize).collect());
            }
          }
          value_null!()
        }
        _ => {
          value_null!("substring")
        }
      }
    } else {
      value_null!("substring")
    }
  } else {
    value_null!("substring")
  }
}

/// Returns substring of `input_string_value`  after the `match_input_string` in string.
pub fn substring_after(input_string_value: &Value, match_input_string: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_input_string {
      return if let Some(index) = input_string.find(match_string) {
        Value::String(input_string[match_string.len() + index..].to_string())
      } else {
        Value::String("".to_string())
      };
    }
  }
  value_null!("substring_after")
}

/// Returns substring of `input_string_value`  before the `match_input_string` in string.
pub fn substring_before(input_string_value: &Value, match_input_string: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_input_string {
      return if let Some(index) = input_string.find(match_string) {
        Value::String(input_string[..index].to_string())
      } else {
        Value::String("".to_string())
      };
    }
  }
  value_null!("substring_before")
}

///
pub fn time_1(value: &Value) -> Value {
  match value {
    Value::String(text) => {
      if let Ok(time) = text.parse::<FeelTime>() {
        return Value::Time(time);
      }
    }
    Value::Date(_) => return Value::Time(FeelTime::utc(0, 0, 0, 0)),
    Value::DateTime(date_time) => return Value::Time(date_time.time()),
    Value::Time(time) => return Value::Time(time.clone()),
    _ => {}
  }
  value_null!("time_1")
}

///
pub fn time_3(hour_value: &Value, minute_value: &Value, second_value: &Value) -> Value {
  if let Value::Number(hour) = hour_value {
    if let Value::Number(minute) = minute_value {
      if let Value::Number(second) = second_value {
        if hour.is_positive() && minute.is_positive() && second.is_positive() {
          let seconds = second.trunc();
          let nanoseconds = second.fract() * FeelNumber::nano();
          if let Some(feel_time) = FeelTime::new_hms_opt((*hour).into(), (*minute).into(), seconds.into(), nanoseconds.into()) {
            return Value::Time(feel_time);
          }
        }
      }
    }
  }
  value_null!("time_3")
}

///
pub fn time_4(hour_value: &Value, minute_value: &Value, second_value: &Value, duration_value: &Value) -> Value {
  if let Value::Number(hour) = hour_value {
    if let Value::Number(minute) = minute_value {
      if let Value::Number(second) = second_value {
        if hour.is_positive() && minute.is_positive() && second.is_positive() {
          let seconds = second.trunc();
          let nanoseconds = second.fract() * FeelNumber::nano();
          match duration_value {
            Value::DaysAndTimeDuration(duration) => {
              if let Some(feel_time) = FeelTime::new_hmso_opt(
                (*hour).into(),
                (*minute).into(),
                seconds.into(),
                nanoseconds.into(),
                duration.as_seconds() as i32,
              ) {
                return Value::Time(feel_time);
              }
            }
            Value::Null(_) => {
              if let Some(feel_time) = FeelTime::new_hms_opt((*hour).into(), (*minute).into(), seconds.into(), nanoseconds.into()) {
                return Value::Time(feel_time);
              }
            }
            _ => {}
          }
        }
      }
    }
  }
  value_null!("time_4")
}

/// ???
pub fn union(lists: &[Value]) -> Value {
  let mut result = vec![];
  for list in lists {
    if let Value::List(items) = list {
      for item in items.as_vec() {
        if result.iter().all(|a| !evaluate_equals(a, item)) {
          result.push(item.clone())
        }
      }
    } else {
      return value_null!("union");
    }
  }
  Value::List(Values::new(result))
}

/// Returns upper-cased string.
pub fn upper_case(input_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    Value::String(input_string.to_uppercase().trim().to_string())
  } else {
    value_null!("upper_case")
  }
}

/// ???
pub fn years_and_months_duration(from_value: &Value, to_value: &Value) -> Value {
  if let Value::Date(from) = from_value {
    if let Value::DateTime(to) = to_value {
      return YearsAndMonthsDuration(to.ym_duration_1(from));
    }
    if let Value::Date(to) = to_value {
      return YearsAndMonthsDuration(to.ym_duration(from));
    }
  }
  if let Value::DateTime(from) = from_value {
    if let Value::DateTime(to) = to_value {
      return YearsAndMonthsDuration(to.ym_duration(from));
    }
    if let Value::Date(to) = to_value {
      return YearsAndMonthsDuration(to.ym_duration(&from.date()));
    }
  }
  value_null!("years_and_months_duration")
}

#[cfg(test)]
mod tests {
  use crate::bifs::core;
  use crate::bifs::core::substring;
  use dmntk_feel::values::{Value, VALUE_FALSE, VALUE_TRUE};
  use dmntk_feel::{value_null, value_number, FeelDaysAndTimeDuration, FeelNumber, FeelYearsAndMonthsDuration};

  #[test]
  fn bif_abs() {
    let expected = value_number!(1201, 2);
    assert_eq!(expected, core::abs(&value_number!(1201, 2)));
    assert_eq!(expected, core::abs(&value_number!(-1201, 2)));
    assert_eq!(
      Value::Null(Some(
        "invalid argument type, expected number, actual type is days and time duration".to_string()
      )),
      core::abs(&Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::default().second(2).nano(5).build()))
    );
    assert_eq!(
      Value::Null(Some(
        "invalid argument type, expected number, actual type is years and months duration".to_string()
      )),
      core::abs(&Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::new_m(-18)))
    );
    assert_eq!(
      Value::Null(Some("invalid argument type, expected number, actual type is Null".to_string())),
      core::abs(&Value::Null(None))
    );
    assert_eq!(
      Value::Null(Some("invalid argument type, expected number, actual type is boolean".to_string())),
      core::abs(&Value::Boolean(true))
    );
    assert_eq!(
      Value::Null(Some("invalid argument type, expected number, actual type is string".to_string())),
      core::abs(&Value::String("text".to_string()))
    );
  }

  #[test]
  fn bif_all() {
    assert_eq!(VALUE_TRUE, core::all(&[]));
    assert_eq!(VALUE_TRUE, core::all(&[Value::Boolean(true)]));
    assert_eq!(VALUE_TRUE, core::all(&[Value::Boolean(true), Value::Boolean(true)]));
    assert_eq!(VALUE_TRUE, core::all(&[Value::Boolean(true), Value::Boolean(true), Value::Boolean(true)]));
    assert_eq!(VALUE_FALSE, core::all(&[Value::Boolean(false)]));
    assert_eq!(VALUE_FALSE, core::all(&[Value::Boolean(true), Value::Boolean(false)]));
    assert_eq!(VALUE_FALSE, core::all(&[Value::Boolean(false), Value::Boolean(true), Value::Boolean(true)]));
    assert_eq!(value_null!(), core::all(&[value_number!(1)]));
    assert_eq!(value_null!(), core::all(&[Value::Boolean(true), value_number!(1)]));
    assert_eq!(value_null!(), core::all(&[value_number!(1), Value::Boolean(true), Value::Boolean(true)]));
  }

  #[test]
  fn bif_substring() {
    // *** utility functions ***

    ///
    fn eq_substring(expected: &str, input_string: &str, start_position: i64) {
      assert_eq!(
        Value::String(expected.to_string()),
        substring(&Value::String(input_string.to_string()), &value_number!(start_position), &value_null!())
      );
    }
    ///
    fn eq_substring_null(input_string: &str, start_position: i64) {
      assert_eq!(
        value_null!(),
        substring(&Value::String(input_string.to_string()), &value_number!(start_position), &value_null!())
      );
    }
    ///
    fn eq_substring_len(expected: &str, input_string: &str, start_position: i64, length: i64) {
      assert_eq!(
        Value::String(expected.to_string()),
        substring(&Value::String(input_string.to_string()), &value_number!(start_position), &value_number!(length))
      );
    }
    ///
    fn eq_substring_len_null(input_string: &str, start_position: i64, length: i64) {
      assert_eq!(
        value_null!(),
        substring(&Value::String(input_string.to_string()), &value_number!(start_position), &value_number!(length))
      );
    }

    // *** tests ***

    // starting position may be not zero
    eq_substring_null("homeless", 0);
    // positive starting position
    eq_substring("homeless", "homeless", 1);
    eq_substring("less", "homeless", 5);
    eq_substring("ss", "homeless", 7);
    eq_substring("s", "homeless", 8);
    eq_substring("😀", "foo\u{1F40E}bar\u{1F600}", 8);
    eq_substring_null("homeless", 9);
    // negative starting position
    eq_substring("s", "homeless", -1);
    eq_substring("less", "homeless", -4);
    eq_substring("homeless", "homeless", -8);
    eq_substring_null("homeless", -9);
    // positive starting position with length
    eq_substring_len("homeless", "homeless", 1, 8);
    eq_substring_len("home", "homeless", 1, 4);
    eq_substring_len("less", "homeless", 5, 4);
    eq_substring_len("el", "homeless", 4, 2);
    eq_substring_len("ss", "homeless", 7, 2);
    eq_substring_len("s", "homeless", 7, 1);
    eq_substring_len("s", "homeless", 8, 1);
    eq_substring_len_null("homeless", 0, 4);
    eq_substring_len_null("homeless", 1, 0);
    eq_substring_len_null("homeless", 1, 9);
    // negative starting position with length
    eq_substring_len("homeless", "homeless", -8, 8);
    eq_substring_len("home", "homeless", -8, 4);
    eq_substring_len("less", "homeless", -4, 4);
    eq_substring_len("el", "homeless", -5, 2);
    eq_substring_len("ss", "homeless", -2, 2);
    eq_substring_len("s", "homeless", -2, 1);
    eq_substring_len("s", "homeless", -1, 1);
    eq_substring_len("😀", "foo\u{1F40E}bar\u{1F600}", -1, 1);
    eq_substring_len_null("homeless", -1, 0);
    eq_substring_len_null("homeless", -3, 4);
    eq_substring_len_null("homeless", -9, 2);
  }
}
