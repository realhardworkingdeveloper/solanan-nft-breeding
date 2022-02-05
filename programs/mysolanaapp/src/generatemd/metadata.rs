use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::config::{self, Attribute, Creator};

pub fn generate(config_location: &String, _assets_directory: &String, output_directory: &String) {
    println!("Generating metadata...");

    let config = config::parse(config_location.as_str()).expect("Error parsing config");

    create_dir_all(output_directory).expect(&format!(
        "Could not create output directory at {}",
        output_directory
    ));

    let mut guaranteed_rolls = config.guaranteed_attribute_rolls.clone();
    let attribute_names: Vec<&String> = config.attributes.keys().collect();
    // How often to insert a guaranteed roll into generated rolls
    let insert_frequency = config.amount / (config.guaranteed_attribute_rolls.len() as u32 + 1);
    for i in 0..config.amount {
        if i > 0 && guaranteed_rolls.len() > 0 && i % insert_frequency == 0 {
            let roll_attributes = {
                guaranteed_rolls[0]
                    .iter()
                    .enumerate()
                    .map(|(i, t)| Trait {
                        trait_type: attribute_names[i].clone(),
                        value: t.to_owned(),
                    })
                    .collect()
            };
            create_metadata(i, roll_attributes, &config, output_directory);
            guaranteed_rolls.remove(0);
        } else {
            generate_attributes(i, &config, output_directory);
        }
    }
}

fn generate_attributes(n: u32, config: &config::Config, output_directory: &String) {
    let mut attributes = Vec::new();
    let mut rng = thread_rng();

    for (attribute_name, keys) in &config.attributes {
        let subattribute;
        match keys {
            Attribute::Keyed(attribute) => {
                let mut attributes_iter = attributes.iter();
                let mut computed_key = "_";
                for raw_key in attribute.keys() {
                    if raw_key == "_" {
                        continue;
                    }
                    let (key, value) = raw_key.split_once(":").unwrap_or(("_key", raw_key));

                    if attributes_iter.any(|t: &Trait| t.trait_type == key && t.value == value) {
                        computed_key = raw_key;
                        break;
                    }
                }

                subattribute = attribute.get(computed_key).expect(&format!(
                    "Could not get attribute {} for key {}",
                    attribute_name, computed_key
                ));
            }
            Attribute::Standard(attribute) => subattribute = attribute,
        }
        calculate_rng_for_attribute(attribute_name, subattribute, &mut attributes, &mut rng);
    }

    create_metadata(n, attributes, config, output_directory)
}

fn calculate_rng_for_attribute(
    attribute_name: &String,
    attribute: &BTreeMap<String, f32>,
    attributes: &mut Vec<Trait>,
    rng: &mut ThreadRng,
) {
    let choices: Vec<&String> = attribute.keys().collect();
    let weights: Vec<&f32> = attribute.values().collect();

    let dist = WeightedIndex::new(weights)
        .expect("Could not create weighted index, are any odds less than 0?");

    let result = dist.sample(rng);

    // Remove file extension (.png)
    let name = choices[result]
        .strip_suffix(".png")
        .unwrap_or(choices[result]);

    attributes.push(Trait {
        trait_type: attribute_name.to_string(),
        value: name.to_string(),
    });
}

fn create_metadata(
    id: u32,
    mut attributes: Vec<Trait>,
    config: &config::Config,
    output_directory: &String,
) {
    let image_name = &format!("{}.png", id);
    let generated_metadata = NFTMetadata {
        name: &format!("{} #{}", &config.name, id),
        symbol: &config.symbol,
        description: &config.description,
        seller_fee_basis_points: 0,
        image: image_name,
        external_url: &config.external_url,
        edition: 0,
        attributes: attributes
            .drain(..)
            .filter(|attribute| !attribute.trait_type.starts_with("_"))
            .collect(),
        properties: Properties {
            files: vec![PropertyFile {
                uri: image_name,
                r#type: "image/png",
            }],
            category: "image",
            creators: config.creators.clone(),
        },
        collection: config.collection.clone(),
    };
    write_metadata(
        id,
        &serde_json::to_string(&generated_metadata).expect("Could not serialize generated JSON"),
        output_directory,
    )
}

fn write_metadata(id: u32, data: &str, output_directory: &String) {
    let path_buffer = Path::new(output_directory).join(format!("{}.json", id));

    let mut file = File::create(&path_buffer).expect(&format!(
        "Could not create file at path {}",
        path_buffer.display()
    ));
    write!(file, "{}", data).expect(&format!(
        "Could not write to file at path {}",
        path_buffer.display()
    ));
}

#[derive(Serialize, Deserialize)]
pub struct NFTMetadata<'a> {
    name: &'a str,
    symbol: &'a str,
    description: &'a str,
    seller_fee_basis_points: u32,
    image: &'a str,
    external_url: &'a str,
    edition: u16,
    pub attributes: Vec<Trait>,
    properties: Properties<'a>,
    collection: config::Collection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct Properties<'a> {
    files: Vec<PropertyFile<'a>>,
    category: &'a str,
    creators: Vec<Creator>,
}

#[derive(Serialize, Deserialize)]
struct PropertyFile<'a> {
    uri: &'a str,
    r#type: &'a str,
}
