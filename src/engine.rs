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
use crate::workers::Workers;
use crate::{node::*, WorkerError};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Version mismatch: Engine({0}), Nodes({1})")]
    VersionMismatch(String, String),
    #[error(transparent)]
    WorkerError(WorkerError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct Engine<TContext> {
    id: String,
    workers: Workers<TContext>,
}

#[allow(dead_code)]
impl<TContext> Engine<TContext> {
    pub fn new(id: String, workers: Workers<TContext>) -> Self {
        Self { id, workers }
    }

    pub fn parse_json(&self, json: &str) -> Result<HashMap<i64, Node>> {
        let value: Value = serde_json::from_str(json)?;
        self.parse_value(value)
    }

    pub fn parse_value(&self, value: Value) -> Result<HashMap<i64, Node>> {
        let version = value["id"]
            .as_str()
            .ok_or(anyhow!("Engine has no version"))?
            .to_string();
        if self.id != version {
            bail!(EngineError::VersionMismatch(self.id.to_string(), version));
        }
        let nodess: HashMap<String, Node> = serde_json::from_value(value["nodes"].clone())?;
        nodess
            .into_iter()
            .map(|(k, v)| Ok((k.parse::<i64>()?, v)))
            .collect::<Result<HashMap<_, _>>>()
    }

    /// Consumes engine
    pub fn process(
        self,
        context: &TContext,
        nodes: &HashMap<i64, Node>,
        start_node_id: i64,
    ) -> Result<OutputData> {
        let mut cache: HashMap<i64, OutputData> = HashMap::new();
        let mut closed_nodes: Vec<i64> = Vec::new();
        let end_id = self.process_nodes(
            context,
            &nodes[&start_node_id],
            nodes,
            &mut cache,
            &mut closed_nodes,
        )?;
        Ok(cache[&end_id].clone().into())
    }

    fn process_node(
        &self,
        context: &TContext,
        node: &Node,
        nodes: &HashMap<i64, Node>,
        cache: &mut HashMap<i64, OutputData>,
        closed_nodes: &mut Vec<i64>,
    ) -> Result<OutputData, EngineError> {
        if cache.contains_key(&node.id) {
            return Ok(cache[&node.id].clone().into());
        }
        if closed_nodes.contains(&node.id) {
            return Ok(Rc::new(HashMap::new()).into());
        }

        let mut input_data: Vec<(String, OutputData)> = vec![];
        for (name, input) in node.inputs.clone().unwrap_or_default().inner() {
            for conn in &input.connections {
                if !closed_nodes.contains(&conn.node) {
                    let out =
                        self.process_node(context, &nodes[&conn.node], nodes, cache, closed_nodes)?;
                    input_data.push((name.clone(), out.clone().into()));
                    if !out.clone().contains_key(&conn.output) && conn.output != "action" {
                        Self::disable_node_tree(&nodes[&conn.node], nodes, closed_nodes);
                        Self::disable_node_tree(node, nodes, closed_nodes);
                    }
                }
            }
        }
        let mut output = Rc::new(HashMap::new()).into();
        if !closed_nodes.contains(&node.id) {
            output = self.workers.call(
                &node.name,
                context,
                node,
                input_data
                    .into_iter()
                    .fold(InputDataBuilder::default(), |b, (key, data)| {
                        b.add_data(key, data)
                    })
                    .build(),
            )?;
            cache.insert(node.id, output.clone().into());
        }
        Ok(output)
    }

    fn process_nodes(
        &self,
        context: &TContext,
        node: &Node,
        nodes: &HashMap<i64, Node>,
        cache: &mut HashMap<i64, OutputData>,
        closed_nodes: &mut Vec<i64>,
    ) -> Result<i64, EngineError> {
        let mut id: i64 = node.id;
        if !closed_nodes.contains(&node.id) {
            let outputdata = self.process_node(context, node, nodes, cache, closed_nodes)?;
            for (name, output) in node.outputs.clone().unwrap_or_default().inner() {
                if outputdata.contains_key(name) {
                    for connection in &output.connections {
                        if !closed_nodes.contains(&connection.node) {
                            id = self.process_nodes(
                                context,
                                &nodes[&connection.node],
                                nodes,
                                cache,
                                closed_nodes,
                            )?;
                        }
                    }
                } else if name != "action" {
                    for connection in &output.connections {
                        if connection.input == name.clone()
                            && !closed_nodes.contains(&connection.node)
                        {
                            Self::disable_node_tree(&nodes[&connection.node], nodes, closed_nodes);
                        }
                    }
                }
            }
        }
        Ok(id)
    }

    fn disable_node_tree(
        node: &'_ Node,
        nodes: &HashMap<i64, Node>,
        closed_nodes: &mut Vec<i64>,
    ) {
        match node.inputs.clone().unwrap_or_default().get("action") {
            None => (),
            Some(input) => {
                if input.connections.len() == 1 {
                    if !closed_nodes.contains(&node.id) {
                        closed_nodes.push(node.id);
                    }
                    for output in node.outputs.clone().unwrap_or_default().inner().values() {
                        for connection in &output.connections {
                            let _node = &nodes[&connection.node];
                            if let Some(input) = _node.inputs.clone().unwrap_or_default().get("action") {
                                if input
                                    .connections
                                    .clone()
                                    .into_iter()
                                    .any(|c| c.node == connection.node)
                                {
                                    Self::disable_node_tree(
                                        &nodes[&connection.node],
                                        nodes,
                                        closed_nodes,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
