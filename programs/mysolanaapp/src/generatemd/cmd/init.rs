use std::{
    collections::BTreeMap,
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::Path,
};

use crate::{
    config::{Attribute, Config},
    Init,
};

const EXAMPLE_CONFIG: &str = r#"{
    "name": "NFT Title",
    "symbol": "SNFT",
    "description": "Hello, NFT!",
    "externalUrl": "https://example.com",
    "creators": [
        {
            "address": "BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD",
            "share": 100
        }
    ],
    "royaltyPercentage": 10,
    "collection": {
        "name": "NFT Collection",
        "family": "NFT Family"
    },
    "attributes": {
        "_key": {
            "KEY": 0.01
        },
        "LAYER_NAME": {
            "_": {
                "FILE_NAME.png": 0.01
            }
        },
        "LAYER_NAME_2": {
            "KEY": {
                "FILE_NAME_3.png": 0.01
            },
            "_": {
                "FILE_NAME_2.png": 0.01
            }
        }
    },
    "guaranteedAttributeRolls": [
        [
            "FILE_NAME.png",
            "FILE_NAME_2.png"
        ]
    ],
    "amount": 10
}"#;

pub fn handle(options: Init) {
    println!("Initializing assets directory...");

    match options.from_existing {
        Some(_) => create_from_existing(options),
        None => create_from_scratch(options),
    }
}

fn create_from_scratch(options: Init) {
    let folder_path = Path::new(&options.folder);
    if folder_path.exists() {
        if options.overwrite {
            remove_dir_all(&folder_path)
                .expect("Encountered error removing existing assets directory");
        } else {
            panic!("Folder already exists, pass --overwrite to overwrite");
        }
    }
    create_dir_all(&folder_path).expect("Encountered error creating new assets directory");

    let mut config_file = File::create(folder_path.join("config.json"))
        .expect("Encountered error creating sample file");
    write!(config_file, "{}", EXAMPLE_CONFIG).expect("Encountered error writing config file");

    create_dir_all(folder_path.join("LAYER_NAME"))
        .expect("Encountered error creating assets directory subfolder");
    File::create(folder_path.join("LAYER_NAME").join("FILE_NAME.png"))
        .expect("Encountered error creating sample file");
}

fn create_from_existing(options: Init) {
    let raw_assets_path = options.from_existing.unwrap();
    let assets_path = Path::new(&raw_assets_path);
    if !assets_path.exists() {
        panic!("Folder at path {} does not exist!", raw_assets_path);
    }
    if !assets_path.is_dir() {
        panic!("Path {} is not a directory!", raw_assets_path);
    }
    let config_path = assets_path.join("config.json");
    if config_path.exists() && !options.overwrite {
        panic!(
            "Config already exists at path {}, pass --overwrite to overwrite",
            config_path.display()
        );
    }

    let mut config: Config =
        serde_json::from_str(EXAMPLE_CONFIG).expect("Unable to parse example config");

    config.attributes = BTreeMap::new();

    for attribute in assets_path
        .read_dir()
        .expect("Encountered error reading assets directory")
    {
        let attribute =
            attribute.expect("Encountered error reading attribute folder in assets directory");

        if !attribute.path().is_dir() {
            continue;
        }

        let mut attribute_layers: BTreeMap<String, f32> = BTreeMap::new();

        for layer in attribute.path().read_dir().expect(&format!(
            "Encountered error reading folder in {}",
            attribute.path().display()
        )) {
            let layer = layer.expect(&format!(
                "Encountered error reading layer in attribute directory {:?}",
                attribute.file_name()
            ));
            let layer_path = layer.path();

            if layer_path.is_dir() {
                continue;
            }

            let layer_name = layer_path.file_name().unwrap().to_str().unwrap();
            attribute_layers.insert(layer_name.to_string(), 0.1);
        }

        let mut attributes = BTreeMap::new();
        attributes.insert("_".to_string(), attribute_layers);

        config.attributes.insert(
            attribute
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            Attribute::Keyed(attributes),
        );
    }

    config.guaranteed_attribute_rolls = vec![];

    let serialized_config =
        &serde_json::to_string(&config).expect("Could not serialize generated config JSON");
    let mut config_file =
        File::create(config_path).expect("Encountered error creating config file");
    write!(config_file, "{}", serialized_config).expect("Encountered error writing config file");
}
