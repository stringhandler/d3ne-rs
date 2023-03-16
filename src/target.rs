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
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, ops::Deref};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputConnection {
    pub node: i64,
    pub output: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub connections: Vec<InputConnection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutputConnection {
    pub node: i64,
    pub input: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Output {
    pub connections: Vec<OutputConnection>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Outputs(HashMap<String, Output>);

impl Outputs {
    pub fn inner(&self) -> &HashMap<String, Output> {
        &self.0
    }
}

impl Deref for Outputs {
    type Target = HashMap<String, Output>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
