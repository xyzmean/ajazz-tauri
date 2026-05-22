//! Keyboard model lookup, loaded from the embedded models.json
//! (generated from reverse-source by extract_models.py).

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Model {
    vendor_id: u16,
    product_id: u16,
    name: String,
}

static MODELS: OnceLock<Vec<Model>> = OnceLock::new();

fn models() -> &'static [Model] {
    MODELS.get_or_init(|| {
        serde_json::from_str(include_str!("../models.json")).unwrap_or_default()
    })
}

/// Resolve a model name by USB vendor/product id, if known.
pub fn find(vendor_id: u16, product_id: u16) -> Option<String> {
    models()
        .iter()
        .find(|m| m.vendor_id == vendor_id && m.product_id == product_id)
        .map(|m| m.name.clone())
}
