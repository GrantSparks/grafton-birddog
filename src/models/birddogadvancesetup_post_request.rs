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
pub struct BirddogadvancesetupPostRequest {
    /// Range (0 to 6)
    #[serde(rename = "Brightness", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<String>,
    /// VERY DARK, DARK, STANDARD, BRIGHT
    #[serde(rename = "BrightnessComp", skip_serializing_if = "Option::is_none")]
    pub brightness_comp: Option<String>,
    /// LOW, MID, HIGH
    #[serde(rename = "CompLevel", skip_serializing_if = "Option::is_none")]
    pub comp_level: Option<String>,
    /// Range (16 to 64)
    #[serde(rename = "GammaOffset", skip_serializing_if = "Option::is_none")]
    pub gamma_offset: Option<String>,
    /// On, Off
    #[serde(rename = "HighResolution", skip_serializing_if = "Option::is_none")]
    pub high_resolution: Option<String>,
    /// On, Off
    #[serde(rename = "VideoEnhancement", skip_serializing_if = "Option::is_none")]
    pub video_enhancement: Option<String>,
}

impl BirddogadvancesetupPostRequest {
    pub fn new() -> BirddogadvancesetupPostRequest {
        BirddogadvancesetupPostRequest {
            brightness: None,
            brightness_comp: None,
            comp_level: None,
            gamma_offset: None,
            high_resolution: None,
            video_enhancement: None,
        }
    }
}
