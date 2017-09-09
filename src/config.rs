// Copyright 2017 CoreOS, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use errors::*;
use std::fs::File;
use std::io::Read;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub access_token: String,
    pub repos: Vec<Repo>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub owner: String,
    pub repo: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    pub description: String,
    pub expression: String,
}

pub fn get_config() -> Result<(Config)> {
    let mut config_string = String::new();
    let mut config_file = File::open("tailor.yaml").chain_err(
        || "Could not open file",
    )?;
    config_file.read_to_string(&mut config_string).chain_err(
        || "Could not read config to string",
    )?;

    let config: Config = serde_yaml::from_str(&config_string).chain_err(
        || "Failed to deserialize config",
    )?;

    Ok(config)
}
