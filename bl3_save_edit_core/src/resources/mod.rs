use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::bl3_item::{BalancePart, InvDataPart, ManufacturerPart};
use crate::models::inventory_serial_db::InventorySerialDb;

type InventoryPartsAll = HashMap<String, ResourceItem>;
type InventorySerialDbCategorizedParts = HashMap<String, Vec<ResourceCategorizedParts>>;

pub(crate) const INVENTORY_SERIAL_DB_JSON_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_SERIAL_DB.json.sz");

const INVENTORY_PARTS_ALL_CATEGORIZED_RON_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_PARTS_ALL_CATEGORIZED.ron.sz");

const INVENTORY_SERIAL_DB_PARTS_CATEGORIZED_RON_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_SERIAL_DB_PARTS_CATEGORIZED.ron.sz");

const INVENTORY_BALANCE_PARTS_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_BALANCE_PARTS.ron.sz");

const INVENTORY_INV_DATA_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_INV_DATA_PARTS.ron.sz");

const INVENTORY_MANUFACTURER_PARTS_COMPRESSED: &[u8] =
    include_bytes!("../../resources/INVENTORY_MANUFACTURER_PARTS.ron.sz");

pub static INVENTORY_SERIAL_DB: Lazy<InventorySerialDb> =
    Lazy::new(|| InventorySerialDb::load().expect("failed to load inventory serial db"));

pub static INVENTORY_PARTS_ALL_CATEGORIZED: Lazy<InventoryPartsAll> =
    Lazy::new(|| load_compressed_data(INVENTORY_PARTS_ALL_CATEGORIZED_RON_COMPRESSED));

pub static INVENTORY_SERIAL_DB_PARTS_CATEGORIZED: Lazy<InventorySerialDbCategorizedParts> =
    Lazy::new(|| load_compressed_data(INVENTORY_SERIAL_DB_PARTS_CATEGORIZED_RON_COMPRESSED));

pub static INVENTORY_BALANCE_PARTS: Lazy<Vec<BalancePart>> =
    Lazy::new(|| load_compressed_data(INVENTORY_BALANCE_PARTS_COMPRESSED));

pub static INVENTORY_INV_DATA_PARTS: Lazy<Vec<InvDataPart>> =
    Lazy::new(|| load_compressed_data(INVENTORY_INV_DATA_COMPRESSED));

pub static INVENTORY_MANUFACTURER_PARTS: Lazy<Vec<ManufacturerPart>> =
    Lazy::new(|| load_compressed_data(INVENTORY_MANUFACTURER_PARTS_COMPRESSED));

pub fn load_compressed_data<T: DeserializeOwned>(input: &'static [u8]) -> T {
    let mut rdr = snap::read::FrameDecoder::new(input);

    ron::de::from_reader(&mut rdr).expect("failed to read compressed data")
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ResourceItem {
    pub manufacturer: String,
    pub rarity: String,
    pub inventory_categorized_parts: Vec<ResourceCategorizedParts>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ResourceCategorizedParts {
    pub category: String,
    pub parts: Vec<ResourcePart>,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize)]
pub struct ResourcePart {
    pub name: String,
    pub min_parts: u8,
    pub max_parts: u8,
    pub dependencies: Option<Vec<String>>,
    pub excluders: Option<Vec<String>>,
    pub info: ResourcePartInfo,
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize)]
pub struct ResourcePartInfo {
    pub positives: Option<String>,
    pub negatives: Option<String>,
    pub effects: Option<String>,
}
