// Original Copyright © 2021 lemonxah
// Modified Copyright © 2022 stringhandler
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::{EngineError, Input, Output};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputValue {
    String(String),
    Bytes(Vec<u8>),
    I64(i64),
    U64(u64),
}

impl Display for OutputValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputValue::String(s) => write!(f, "String: {}", s),
            OutputValue::I64(i) => write!(f, "I64: {}", i),
            OutputValue::U64(u) => write!(f, "U64: {}", u),
            OutputValue::Bytes(b) => write!(f, "Bytes: {:?}", b),
        }
    }
}

impl OutputValue {
    pub fn as_i64(&self) -> Result<i64, EngineError> {
        match self {
            OutputValue::I64(i) => Ok(*i),
            _ => Err(EngineError::InvalidOutputType {
                expected: "i64".to_string(),
                actual: self.to_string(),
            }),
        }
    }
}

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Node input conversion error: {0}")]
    ConversionError(String),
    #[error("No value found for: {0}")]
    NoValueFound(String),
    #[error("Field: {0}, Value: {1}, Deserialization error: {2}")]
    DeserializeError(String, String, serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub data: HashMap<String, Value>,
    pub group: Option<i64>,
    pub position: Option<Vec<f32>>,
    #[serde(default)]
    pub inputs: HashMap<String, Input>,
    #[serde(default)]
    pub outputs: HashMap<String, Output>,
}

impl Node {
    pub fn get_data<TValue: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<TValue>, NodeError> {
        self.data
            .get(key)
            .map(|v| {
                serde_json::from_value(v.clone())
                    .map_err(|e| NodeError::DeserializeError(key.to_string(), v.to_string(), e))
            })
            .transpose()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Number;

    #[test]
    fn test_get_data() {
        let node = Node {
            id: 1,
            name: "test".to_string(),
            data: {
                let mut data = HashMap::new();
                data.insert("test".to_string(), Value::Number(Number::from(1i32)));
                data
            },
            group: None,
            position: None,
            inputs: Default::default(),
            outputs: Default::default(),
        };

        let value: i32 = node.get_data("test").unwrap().unwrap();
        assert_eq!(value, 1);
    }

    // #[test]
    // fn test_get_input() {
    //     let node = Node {
    //         id: 1,
    //         name: "test".to_string(),
    //         data: Default::default(),
    //         group: None,
    //         position: None,
    //         inputs: {
    //             let mut inputs = HashMap::new();
    //             inputs.insert(
    //                 "test".to_string(),
    //                 Input {
    //                     connections: vec![InputConnection {
    //                         node: 1,
    //                         output: "test".to_string(),
    //                         data: Default::default(),
    //                     }],
    //                 },
    //             );
    //             inputs
    //         },
    //         outputs: Default::default(),
    //     };
    //
    //     let value: i32 = node.get_input("test").unwrap().unwrap();
    //     assert_eq!(value, 1);
    // }
}
