/*
 * BirdDog RESTful API
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
pub struct ConnectToPostRequest {
    /// None
    #[serde(rename = "sourceName", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// Status can be `Encode` or `Decode`
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ConnectToPostRequest {
    pub fn new() -> ConnectToPostRequest {
        ConnectToPostRequest {
            source_name: None,
            status: None,
        }
    }
}
