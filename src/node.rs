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
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputValue {
    String(String),
    I64(i64),
    U64(u64),
}

impl Display for OutputValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputValue::String(s) => write!(f, "String: {}", s),
            OutputValue::I64(i) => write!(f, "I64: {}", i),
            OutputValue::U64(u) => write!(f, "U64: {}", u),
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

#[derive(Debug)]
pub struct IOData {
    pub data: Box<dyn Any>,
}

#[allow(dead_code)]
impl IOData {
    pub fn is<B: Any>(&self) -> bool {
        TypeId::of::<B>() == (*self.data).type_id()
    }
    pub fn get<A>(&self) -> Option<&A>
    where
        A: 'static,
    {
        self.data.downcast_ref::<A>()
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

#[derive(Debug)]
pub struct NodeResult(pub IOData);

impl Deref for NodeResult {
    type Target = IOData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct OutputData(pub Rc<HashMap<String, NodeResult>>);

impl From<Rc<HashMap<String, NodeResult>>> for OutputData {
    fn from(inner: Rc<HashMap<String, NodeResult>>) -> Self {
        OutputData(inner)
    }
}

#[derive(Default)]
pub struct OutputDataBuilder {
    data: Vec<(String, Box<dyn Any>)>,
}

impl OutputDataBuilder {
    pub fn add_data<S: ToString>(&mut self, key: S, data: Box<dyn Any>) -> &mut Self {
        self.data.push((key.to_string(), data));
        self
    }

    pub fn data<S: ToString>(mut self, key: S, data: Box<dyn Any>) -> Self {
        self.data.push((key.to_string(), data));
        self
    }

    pub fn build(self) -> OutputData {
        OutputData(Rc::new(
            self.data
                .into_iter()
                .map(|(key, data)| (key, NodeResult(IOData { data })))
                .collect::<HashMap<_, _>>(),
        ))
    }
}

impl Deref for OutputData {
    type Target = Rc<HashMap<String, NodeResult>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct InputData(pub HashMap<String, OutputData>);

#[derive(Default)]
pub struct InputDataBuilder {
    data: Vec<(String, OutputData)>,
}

impl InputDataBuilder {
    pub fn add_data(mut self, key: String, data: OutputData) -> InputDataBuilder {
        self.data.push((key, data));
        self
    }

    pub fn build(self) -> InputData {
        InputData(
            self.data
                .into_iter()
                .map(|(key, data)| (key, data))
                .collect::<HashMap<_, _>>(),
        )
    }
}

impl From<HashMap<String, OutputData>> for InputData {
    fn from(inner: HashMap<String, OutputData>) -> Self {
        InputData(inner)
    }
}

impl Deref for InputData {
    type Target = HashMap<String, OutputData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
    use crate::InputConnection;

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
