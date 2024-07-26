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
pub struct ListResponse {
    #[serde(rename = "None", skip_serializing_if = "Option::is_none")]
    pub none: Option<String>,
}

impl ListResponse {
    pub fn new() -> ListResponse {
        ListResponse { none: None }
    }
}
