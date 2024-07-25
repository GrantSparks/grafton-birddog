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
pub struct BirddogexpsetupPostRequest {
    /// Range (1 to 48)
    #[serde(rename = "AeResponse", skip_serializing_if = "Option::is_none")]
    pub ae_response: Option<String>,
    /// On, Off
    #[serde(rename = "BackLight", skip_serializing_if = "Option::is_none")]
    pub back_light: Option<String>,
    /// Range (0, 5 to 31)
    #[serde(rename = "BrightLevel", skip_serializing_if = "Option::is_none")]
    pub bright_level: Option<String>,
    /// On, Off
    #[serde(rename = "ExpCompEn", skip_serializing_if = "Option::is_none")]
    pub exp_comp_en: Option<String>,
    /// Range (0 to 14, -128 to 127)
    #[serde(rename = "ExpCompLvl", skip_serializing_if = "Option::is_none")]
    pub exp_comp_lvl: Option<String>,
    /// FULL-AUTO, MANUAL, SHUTTER-PRI, IRIS-PRI, BRIGHT
    #[serde(rename = "ExpMode", skip_serializing_if = "Option::is_none")]
    pub exp_mode: Option<String>,
    /// Range (1 to GainLimit)
    #[serde(rename = "GainLevel", skip_serializing_if = "Option::is_none")]
    pub gain_level: Option<String>,
    /// Range (4 to 15)
    #[serde(rename = "GainLimit", skip_serializing_if = "Option::is_none")]
    pub gain_limit: Option<String>,
    /// On, Off
    #[serde(rename = "GainPoint", skip_serializing_if = "Option::is_none")]
    pub gain_point: Option<String>,
    /// Range (1 to 13)
    #[serde(rename = "GainPointPosition", skip_serializing_if = "Option::is_none")]
    pub gain_point_position: Option<String>,
    /// On, Off
    #[serde(rename = "HighSensitivity", skip_serializing_if = "Option::is_none")]
    pub high_sensitivity: Option<String>,
    /// Range (0, 5 to 17)
    #[serde(rename = "IrisLevel", skip_serializing_if = "Option::is_none")]
    pub iris_level: Option<String>,
    /// On, Off
    #[serde(rename = "ShutterControlOverwrite", skip_serializing_if = "Option::is_none")]
    pub shutter_control_overwrite: Option<String>,
    /// Range (20 to 33)
    #[serde(rename = "ShutterMaxSpeed", skip_serializing_if = "Option::is_none")]
    pub shutter_max_speed: Option<String>,
    /// Range (16 to ShutterMaxSpeed)
    #[serde(rename = "ShutterMinSpeed", skip_serializing_if = "Option::is_none")]
    pub shutter_min_speed: Option<String>,
    /// Range (0 to 21)
    #[serde(rename = "ShutterSpeed", skip_serializing_if = "Option::is_none")]
    pub shutter_speed: Option<String>,
    /// Range(30 - 110)
    #[serde(rename = "ShutterSpeedOverwrite", skip_serializing_if = "Option::is_none")]
    pub shutter_speed_overwrite: Option<String>,
    /// On, Off
    #[serde(rename = "SlowShutterEn", skip_serializing_if = "Option::is_none")]
    pub slow_shutter_en: Option<String>,
    /// Range (1 to 6)
    #[serde(rename = "SlowShutterLimit", skip_serializing_if = "Option::is_none")]
    pub slow_shutter_limit: Option<String>,
    /// On, Off
    #[serde(rename = "Spotlight", skip_serializing_if = "Option::is_none")]
    pub spotlight: Option<String>,
}

impl BirddogexpsetupPostRequest {
    pub fn new() -> BirddogexpsetupPostRequest {
        BirddogexpsetupPostRequest {
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

