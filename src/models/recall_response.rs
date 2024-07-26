/*
 * BirdDog RESTful API 2.0
 *
 * Unofficial idiomatic rust bindings for the BirdDog RESTful API 2.0 generated using OpenAPI Generator.
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecallResponse {
    #[serde(rename = "Preset", skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
}

impl RecallResponse {
    pub fn new() -> RecallResponse {
        RecallResponse {
            preset: None,
        }
    }
}
