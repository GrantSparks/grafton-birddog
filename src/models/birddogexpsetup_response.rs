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
pub struct BirddogexpsetupResponse {
    #[serde(rename = "AeResponse", skip_serializing_if = "Option::is_none")]
    pub ae_response: Option<String>,
    #[serde(rename = "BackLight", skip_serializing_if = "Option::is_none")]
    pub back_light: Option<String>,
    #[serde(rename = "BrightLevel", skip_serializing_if = "Option::is_none")]
    pub bright_level: Option<String>,
    #[serde(rename = "ExpCompEn", skip_serializing_if = "Option::is_none")]
    pub exp_comp_en: Option<String>,
    #[serde(rename = "ExpCompLvl", skip_serializing_if = "Option::is_none")]
    pub exp_comp_lvl: Option<String>,
    #[serde(rename = "ExpMode", skip_serializing_if = "Option::is_none")]
    pub exp_mode: Option<String>,
    #[serde(rename = "GainLevel", skip_serializing_if = "Option::is_none")]
    pub gain_level: Option<String>,
    #[serde(rename = "GainLimit", skip_serializing_if = "Option::is_none")]
    pub gain_limit: Option<String>,
    #[serde(rename = "GainPoint", skip_serializing_if = "Option::is_none")]
    pub gain_point: Option<String>,
    #[serde(rename = "GainPointPosition", skip_serializing_if = "Option::is_none")]
    pub gain_point_position: Option<String>,
    #[serde(rename = "HighSensitivity", skip_serializing_if = "Option::is_none")]
    pub high_sensitivity: Option<String>,
    #[serde(rename = "IrisLevel", skip_serializing_if = "Option::is_none")]
    pub iris_level: Option<String>,
    #[serde(rename = "ShutterControlOverwrite", skip_serializing_if = "Option::is_none")]
    pub shutter_control_overwrite: Option<String>,
    #[serde(rename = "ShutterMaxSpeed", skip_serializing_if = "Option::is_none")]
    pub shutter_max_speed: Option<String>,
    #[serde(rename = "ShutterMinSpeed", skip_serializing_if = "Option::is_none")]
    pub shutter_min_speed: Option<String>,
    #[serde(rename = "ShutterSpeed", skip_serializing_if = "Option::is_none")]
    pub shutter_speed: Option<String>,
    #[serde(rename = "ShutterSpeedOverwrite", skip_serializing_if = "Option::is_none")]
    pub shutter_speed_overwrite: Option<String>,
    #[serde(rename = "SlowShutterEn", skip_serializing_if = "Option::is_none")]
    pub slow_shutter_en: Option<String>,
    #[serde(rename = "SlowShutterLimit", skip_serializing_if = "Option::is_none")]
    pub slow_shutter_limit: Option<String>,
    #[serde(rename = "Spotlight", skip_serializing_if = "Option::is_none")]
    pub spotlight: Option<String>,
}

impl BirddogexpsetupResponse {
    pub fn new() -> BirddogexpsetupResponse {
        BirddogexpsetupResponse {
            ae_response: None,
            back_light: None,
            bright_level: None,
            exp_comp_en: None,
            exp_comp_lvl: None,
            exp_mode: None,
            gain_level: None,
            gain_limit: None,
            gain_point: None,
            gain_point_position: None,
            high_sensitivity: None,
            iris_level: None,
            shutter_control_overwrite: None,
            shutter_max_speed: None,
            shutter_min_speed: None,
            shutter_speed: None,
            shutter_speed_overwrite: None,
            slow_shutter_en: None,
            slow_shutter_limit: None,
            spotlight: None,
        }
    }
}
