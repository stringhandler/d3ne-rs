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
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Group {
    id: i64,
    nodes: Vec<i64>,
    min_width: f32,
    max_widht: f32,
    position: [f32; 2],
    width: f32,
    height: f32,
}
#[allow(dead_code)]
pub type Groups = HashMap<i64, Group>;
