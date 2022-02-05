use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::BTreeMap, fs};

pub fn parse(location: &str) -> Result<Config> {
    let config_file = fs::read_to_string(location).expect("Could not read configuration file");
    let config: Config = serde_json::from_str(&config_file)?;
    Ok(config)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub external_url: String,
    pub creators: Vec<Creator>,
    pub royalty_percentage: u8,
    pub collection: Collection,
    pub attributes: BTreeMap<String, Attribute>,
    pub guaranteed_attribute_rolls: Vec<Vec<String>>,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Collection {
    pub name: String,
    pub family: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attribute {
    Keyed(BTreeMap<String, BTreeMap<String, f32>>),
    Standard(BTreeMap<String, f32>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Creator {
    pub address: String,
    pub share: u8,
}
