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
pub struct BirddoggammasetupGet200Response {
    #[serde(rename = "BlackGammaLevel", skip_serializing_if = "Option::is_none")]
    pub black_gamma_level: Option<String>,
    #[serde(rename = "BlackLevel", skip_serializing_if = "Option::is_none")]
    pub black_level: Option<String>,
    #[serde(rename = "BlackLevelRange", skip_serializing_if = "Option::is_none")]
    pub black_level_range: Option<String>,
    #[serde(rename = "Effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "Offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(rename = "Pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "PatternFine", skip_serializing_if = "Option::is_none")]
    pub pattern_fine: Option<String>,
    #[serde(rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[serde(rename = "VisibilityEnhancer", skip_serializing_if = "Option::is_none")]
    pub visibility_enhancer: Option<String>,
}

impl BirddoggammasetupGet200Response {
    pub fn new() -> BirddoggammasetupGet200Response {
        BirddoggammasetupGet200Response {
            black_gamma_level: None,
            black_level: None,
            black_level_range: None,
            effect: None,
            level: None,
            offset: None,
            pattern: None,
            pattern_fine: None,
            settings: None,
            visibility_enhancer: None,
        }
    }
}
