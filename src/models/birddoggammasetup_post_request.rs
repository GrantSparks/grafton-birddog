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
pub struct BirddoggammasetupPostRequest {
    /// Range (0 to 14)
    #[serde(rename = "BlackGammaLevel", skip_serializing_if = "Option::is_none")]
    pub black_gamma_level: Option<String>,
    /// Range (0 to 96)
    #[serde(rename = "BlackLevel", skip_serializing_if = "Option::is_none")]
    pub black_level: Option<String>,
    /// LOW, MID, HIGH
    #[serde(rename = "BlackLevelRange", skip_serializing_if = "Option::is_none")]
    pub black_level_range: Option<String>,
    /// Range (-3 to 3)
    #[serde(rename = "Effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Range (0 to 14)
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Range (-64 to 64)
    #[serde(rename = "Offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// Range (0 to 512)
    #[serde(rename = "Pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Range (0 to9)
    #[serde(rename = "PatternFine", skip_serializing_if = "Option::is_none")]
    pub pattern_fine: Option<String>,
    /// PATTERN, STANDARD, STRAIGHT
    #[serde(rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    /// On, Off
    #[serde(rename = "VisibilityEnhancer", skip_serializing_if = "Option::is_none")]
    pub visibility_enhancer: Option<String>,
}

impl BirddoggammasetupPostRequest {
    pub fn new() -> BirddoggammasetupPostRequest {
        BirddoggammasetupPostRequest {
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
