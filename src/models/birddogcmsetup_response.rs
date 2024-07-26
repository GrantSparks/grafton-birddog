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
pub struct BirddogcmsetupResponse {
    #[serde(rename = "BlueGain", skip_serializing_if = "Option::is_none")]
    pub blue_gain: Option<String>,
    #[serde(rename = "BlueHue", skip_serializing_if = "Option::is_none")]
    pub blue_hue: Option<String>,
    #[serde(rename = "ColourGain", skip_serializing_if = "Option::is_none")]
    pub colour_gain: Option<String>,
    #[serde(rename = "CyanGain", skip_serializing_if = "Option::is_none")]
    pub cyan_gain: Option<String>,
    #[serde(rename = "CyanHue", skip_serializing_if = "Option::is_none")]
    pub cyan_hue: Option<String>,
    #[serde(rename = "GreenGain", skip_serializing_if = "Option::is_none")]
    pub green_gain: Option<String>,
    #[serde(rename = "GreenHue", skip_serializing_if = "Option::is_none")]
    pub green_hue: Option<String>,
    #[serde(rename = "HuePhase", skip_serializing_if = "Option::is_none")]
    pub hue_phase: Option<String>,
    #[serde(rename = "MagGain", skip_serializing_if = "Option::is_none")]
    pub mag_gain: Option<String>,
    #[serde(rename = "MagHue", skip_serializing_if = "Option::is_none")]
    pub mag_hue: Option<String>,
    #[serde(rename = "RedGain", skip_serializing_if = "Option::is_none")]
    pub red_gain: Option<String>,
    #[serde(rename = "RedHue", skip_serializing_if = "Option::is_none")]
    pub red_hue: Option<String>,
    #[serde(rename = "YellowGain", skip_serializing_if = "Option::is_none")]
    pub yellow_gain: Option<String>,
    #[serde(rename = "YellowHue", skip_serializing_if = "Option::is_none")]
    pub yellow_hue: Option<String>,
}

impl BirddogcmsetupResponse {
    pub fn new() -> BirddogcmsetupResponse {
        BirddogcmsetupResponse {
            blue_gain: None,
            blue_hue: None,
            colour_gain: None,
            cyan_gain: None,
            cyan_hue: None,
            green_gain: None,
            green_hue: None,
            hue_phase: None,
            mag_gain: None,
            mag_hue: None,
            red_gain: None,
            red_hue: None,
            yellow_gain: None,
            yellow_hue: None,
        }
    }
}

