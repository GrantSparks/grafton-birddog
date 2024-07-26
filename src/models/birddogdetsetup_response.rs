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
pub struct BirddogdetsetupResponse {
    #[serde(rename = "Bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    #[serde(rename = "BwBalance", skip_serializing_if = "Option::is_none")]
    pub bw_balance: Option<String>,
    #[serde(rename = "Crispening", skip_serializing_if = "Option::is_none")]
    pub crispening: Option<String>,
    #[serde(rename = "Detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "HighLightDetail", skip_serializing_if = "Option::is_none")]
    pub high_light_detail: Option<String>,
    #[serde(rename = "HvBalance", skip_serializing_if = "Option::is_none")]
    pub hv_balance: Option<String>,
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "Limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(rename = "SuperLow", skip_serializing_if = "Option::is_none")]
    pub super_low: Option<String>,
}

impl BirddogdetsetupResponse {
    pub fn new() -> BirddogdetsetupResponse {
        BirddogdetsetupResponse {
            bandwidth: None,
            bw_balance: None,
            crispening: None,
            detail: None,
            high_light_detail: None,
            hv_balance: None,
            level: None,
            limit: None,
            super_low: None,
        }
    }
}
