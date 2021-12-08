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

use self::errors::*;
use dmntk_common::DmntkError;
use dmntk_feel::context::FeelContext;
use dmntk_feel::value_null;
use dmntk_feel::values::{Value, Values};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

pub struct WrappedValue(pub Value);

#[derive(Debug, Deserialize)]
pub struct InputNodeDto {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "value")]
  pub value: Option<ValueDto>,
}

#[derive(Debug, Serialize)]
pub struct OutputNodeDto {
  #[serde(rename = "value")]
  pub value: Option<ValueDto>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ValueDto {
  #[serde(rename = "simple")]
  pub simple: Option<SimpleDto>,
  #[serde(rename = "components")]
  pub components: Option<Vec<ComponentDto>>,
  #[serde(rename = "list")]
  pub list: Option<ListDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleDto {
  #[serde(rename = "type")]
  pub typ: Option<String>,
  #[serde(rename = "text")]
  pub text: Option<String>,
  #[serde(rename = "isNil")]
  pub nil: bool,
}

impl SimpleDto {
  /// Creates [SimpleDto] with some initial value.
  pub fn some(typ: &str, text: &str) -> Option<Self> {
    Some(Self {
      typ: Some(typ.to_string()),
      text: Some(text.to_string()),
      nil: false,
    })
  }
  /// Creates [SimpleDto] with `nil` value.
  pub fn nil() -> Option<Self> {
    Some(Self {
      typ: None,
      text: None,
      nil: true,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentDto {
  #[serde(rename = "name")]
  pub name: Option<String>,
  #[serde(rename = "value")]
  pub value: Option<ValueDto>,
  #[serde(rename = "isNil")]
  pub nil: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDto {
  #[serde(rename = "items")]
  pub items: Vec<ValueDto>,
  #[serde(rename = "isNil")]
  pub nil: bool,
}

impl ListDto {
  /// Creates [ListDto] with initial items.
  pub fn items(items: Vec<ValueDto>) -> Option<Self> {
    Some(Self { items, nil: false })
  }
}

impl TryFrom<Value> for OutputNodeDto {
  type Error = DmntkError;
  /// Tries to convert [Value] to [OutputNodeDto].
  fn try_from(value: Value) -> Result<Self, Self::Error> {
    match value {
      Value::String(inner) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:string", &inner),
          ..Default::default()
        }),
      }),
      v @ Value::Number(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:decimal", &v.to_string()),
          ..Default::default()
        }),
      }),
      v @ Value::Boolean(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:boolean", &v.to_string()),
          ..Default::default()
        }),
      }),
      v @ Value::Date(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:date", &v.to_string()),
          ..Default::default()
        }),
      }),
      v @ Value::DateTime(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:dateTime", &v.to_string()),
          ..Default::default()
        }),
      }),
      v @ Value::Time(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:time", &v.to_string()),
          ..Default::default()
        }),
      }),
      v @ Value::YearsAndMonthsDuration(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:duration", &v.to_string()),
          ..Default::default()
        }),
      }),
      v @ Value::DaysAndTimeDuration(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::some("xsd:duration", &v.to_string()),
          ..Default::default()
        }),
      }),
      Value::Null(_) => Ok(OutputNodeDto {
        value: Some(ValueDto {
          simple: SimpleDto::nil(),
          ..Default::default()
        }),
      }),
      Value::Context(ctx) => {
        let mut components = vec![];
        for (name, value) in ctx.iter() {
          components.push(ComponentDto {
            name: Some(name.to_string()),
            value: Some(value.try_into()?),
            nil: false,
          });
        }
        Ok(OutputNodeDto {
          value: Some(ValueDto {
            components: Some(components),
            ..Default::default()
          }),
        })
      }
      Value::List(list) => {
        let mut items = vec![];
        for value in list.as_vec() {
          items.push(value.try_into()?);
        }
        Ok(OutputNodeDto {
          value: Some(ValueDto {
            list: ListDto::items(items),
            ..Default::default()
          }),
        })
      }
      _ => Ok(OutputNodeDto { value: None }),
    }
  }
}

impl TryFrom<&Value> for ValueDto {
  type Error = DmntkError;
  /// Tries to convert [Value] to [ValueDto].
  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    match value {
      Value::String(inner) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:string", inner),
        ..Default::default()
      }),
      v @ Value::Number(_) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:decimal", &v.to_string()),
        ..Default::default()
      }),
      v @ Value::Boolean(_) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:boolean", &v.to_string()),
        ..Default::default()
      }),
      v @ Value::Date(_) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:date", &v.to_string()),
        ..Default::default()
      }),
      v @ Value::DateTime(_) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:dateTime", &v.to_string()),
        ..Default::default()
      }),
      v @ Value::Time(_) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:time", &v.to_string()),
        ..Default::default()
      }),
      v @ Value::YearsAndMonthsDuration(_) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:duration", &v.to_string()),
        ..Default::default()
      }),
      v @ Value::DaysAndTimeDuration(_) => Ok(ValueDto {
        simple: SimpleDto::some("xsd:duration", &v.to_string()),
        ..Default::default()
      }),
      Value::Null(_) => Ok(ValueDto {
        simple: SimpleDto::nil(),
        ..Default::default()
      }),
      Value::Context(ctx) => {
        let mut components = vec![];
        for (name, value) in ctx.iter() {
          components.push(ComponentDto {
            name: Some(name.to_string()),
            value: Some(value.try_into()?),
            nil: false,
          });
        }
        Ok(ValueDto {
          components: Some(components),
          ..Default::default()
        })
      }
      Value::List(list) => {
        let mut items = vec![];
        for value in list.as_vec() {
          items.push(value.try_into()?);
        }
        Ok(ValueDto {
          list: ListDto::items(items),
          ..Default::default()
        })
      }
      _ => Ok(Default::default()),
    }
  }
}

impl TryFrom<&Vec<InputNodeDto>> for WrappedValue {
  type Error = DmntkError;
  fn try_from(items: &Vec<InputNodeDto>) -> Result<Self, Self::Error> {
    let mut ctx: FeelContext = Default::default();
    for item in items {
      let name = dmntk_feel_parser::parse_longest_name(&item.name)?;
      ctx.set_entry(&name, WrappedValue::try_from(item)?.0);
    }
    Ok(WrappedValue(ctx.into()))
  }
}

impl TryFrom<&InputNodeDto> for WrappedValue {
  type Error = DmntkError;
  fn try_from(input_node_dto: &InputNodeDto) -> Result<Self, Self::Error> {
    if let Some(value_dto) = &input_node_dto.value {
      WrappedValue::try_from(value_dto)
    } else {
      Err(missing_parameter("InputNodeDto.value"))
    }
  }
}

impl TryFrom<&Vec<ValueDto>> for WrappedValue {
  type Error = DmntkError;
  fn try_from(items: &Vec<ValueDto>) -> Result<Self, Self::Error> {
    let mut values = vec![];
    for item in items {
      values.push(WrappedValue::try_from(item)?.0);
    }
    Ok(WrappedValue(Value::List(Values::new(values))))
  }
}

impl TryFrom<&ValueDto> for WrappedValue {
  type Error = DmntkError;
  fn try_from(value: &ValueDto) -> Result<Self, Self::Error> {
    if let Some(value_dto) = &value.simple {
      return WrappedValue::try_from(value_dto);
    }
    if let Some(components) = &value.components {
      return WrappedValue::try_from(components);
    }
    if let Some(list) = &value.list {
      return WrappedValue::try_from(list);
    }
    Err(missing_parameter("no `simple`, `components` or `list` attribute in ValueTypeDto"))
  }
}

impl TryFrom<&SimpleDto> for WrappedValue {
  type Error = DmntkError;
  fn try_from(value: &SimpleDto) -> Result<Self, Self::Error> {
    if value.nil {
      return Ok(WrappedValue(Value::Null(None)));
    }
    if let Some(typ) = &value.typ {
      if let Some(text) = &value.text {
        return match typ.as_str() {
          "xsd:string" => Ok(WrappedValue(Value::String(text.clone()))),
          "xsd:integer" => Ok(WrappedValue(Value::try_from_xsd_integer(text)?)),
          "xsd:decimal" => Ok(WrappedValue(Value::try_from_xsd_decimal(text)?)),
          "xsd:double" => Ok(WrappedValue(Value::try_from_xsd_double(text)?)),
          "xsd:boolean" => Ok(WrappedValue(Value::try_from_xsd_boolean(text)?)),
          "xsd:date" => Ok(WrappedValue(Value::try_from_xsd_date(text)?)),
          "xsd:time" => Ok(WrappedValue(Value::try_from_xsd_time(text)?)),
          "xsd:dateTime" => Ok(WrappedValue(Value::try_from_xsd_date_time(text)?)),
          "xsd:duration" => Ok(WrappedValue(Value::try_from_xsd_duration(text)?)),
          _ => Err(invalid_parameter(&format!("unrecognized type: `{}` in value", typ))),
        };
      }
    }
    Err(invalid_parameter("ValueDto"))
  }
}

impl TryFrom<&Vec<ComponentDto>> for WrappedValue {
  type Error = DmntkError;
  fn try_from(items: &Vec<ComponentDto>) -> Result<Self, Self::Error> {
    let mut ctx: FeelContext = Default::default();
    for item in items {
      let item_name = item.name.as_ref().ok_or_else(|| invalid_parameter("component should have a name"))?;
      let value = WrappedValue::try_from(item)?;
      let key = dmntk_feel_parser::parse_longest_name(item_name)?;
      ctx.set_entry(&key, value.0);
    }
    Ok(WrappedValue(ctx.into()))
  }
}

impl TryFrom<&ComponentDto> for WrappedValue {
  type Error = DmntkError;
  fn try_from(value: &ComponentDto) -> Result<Self, Self::Error> {
    if value.nil {
      return Ok(WrappedValue(value_null!()));
    }
    if let Some(v) = &value.value {
      WrappedValue::try_from(v)
    } else {
      Err(invalid_parameter("component should have a value"))
    }
  }
}

impl TryFrom<&ListDto> for WrappedValue {
  type Error = DmntkError;
  fn try_from(value: &ListDto) -> Result<Self, Self::Error> {
    if value.nil {
      return Ok(WrappedValue(value_null!()));
    }
    WrappedValue::try_from(&value.items)
  }
}

/// Definitions of errors reported by DTO conversions.
mod errors {
  use dmntk_common::DmntkError;

  /// DTO conversion errors.
  #[derive(Debug, PartialEq)]
  enum DtoError {
    MissingParameter(String),
    InvalidParameter(String),
  }

  impl From<DtoError> for DmntkError {
    fn from(e: DtoError) -> Self {
      DmntkError::new("DTOError", &format!("{}", e))
    }
  }

  impl std::fmt::Display for DtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        DtoError::MissingParameter(name) => {
          write!(f, "missing parameter: {}", name)
        }
        DtoError::InvalidParameter(name) => {
          write!(f, "invalid parameter: {}", name)
        }
      }
    }
  }

  pub fn missing_parameter(name: &str) -> DmntkError {
    DtoError::MissingParameter(name.to_string()).into()
  }

  pub fn invalid_parameter(description: &str) -> DmntkError {
    DtoError::InvalidParameter(description.to_string()).into()
  }
}
