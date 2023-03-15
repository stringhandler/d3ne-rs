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
use crate::node::*;
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkerError {
    #[error("Worker Not Found: `{0}`")]
    WorkerNotFound(String),
    #[error("Node[{0}]: {1}")]
    NodeRunError(i64, anyhow::Error),
}

pub trait Worker<TContext> {
    fn name(&self) -> &str;
    fn work(&self, context: &TContext, node: &Node, input_data: InputData) -> Result<OutputData>;
}

pub struct Workers<TContext>(HashMap<String, Box<dyn Worker<TContext>>>);

impl<TContext> Workers<TContext> {
    pub fn call(
        &self,
        name: &str,
        context: &TContext,
        node: &Node,
        input: InputData,
    ) -> Result<OutputData> {
        self.0
            .get(name)
            .map(|worker| {
                worker
                    .work(context, node, input)
                    .map_err(|e| anyhow!(WorkerError::NodeRunError(node.id, e)))
            })
            .ok_or(WorkerError::WorkerNotFound(name.into()))?
    }
}

#[derive(Default)]
pub struct WorkersBuilder<TContext> {
    data: Vec<(String, Box<dyn Worker<TContext>>)>,
}

#[allow(dead_code)]
impl<TContext> WorkersBuilder<TContext> {

    pub fn add<A>(&mut self, worker: A) -> &mut Self
    where
        A: Worker<TContext> + 'static,
    {
        self.data
            .push((worker.name().to_string(), Box::new(worker)));
        self
    }

    pub fn build(self) -> Workers<TContext> {
        Workers(self.data.into_iter().collect::<HashMap<_, _>>())
    }
}
