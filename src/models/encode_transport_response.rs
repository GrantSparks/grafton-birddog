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
pub struct EncodeTransportResponse {
    #[serde(rename = "Txpm", skip_serializing_if = "Option::is_none")]
    pub txpm: Option<String>,
    #[serde(rename = "Txnetprefix", skip_serializing_if = "Option::is_none")]
    pub txnetprefix: Option<String>,
    #[serde(rename = "Txnetmask", skip_serializing_if = "Option::is_none")]
    pub txnetmask: Option<String>,
    #[serde(rename = "Txmcttl", skip_serializing_if = "Option::is_none")]
    pub txmcttl: Option<String>,
}

impl EncodeTransportResponse {
    pub fn new() -> EncodeTransportResponse {
        EncodeTransportResponse {
            txpm: None,
            txnetprefix: None,
            txnetmask: None,
            txmcttl: None,
        }
    }
}

