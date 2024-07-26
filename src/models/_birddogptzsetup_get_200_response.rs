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
pub struct BirddogptzsetupGet200Response {
    #[serde(rename = "PanSpeed", skip_serializing_if = "Option::is_none")]
    pub pan_speed: Option<String>,
    #[serde(rename = "TiltSpeed", skip_serializing_if = "Option::is_none")]
    pub tilt_speed: Option<String>,
    #[serde(rename = "ZoomSpeed", skip_serializing_if = "Option::is_none")]
    pub zoom_speed: Option<String>,
}

impl BirddogptzsetupGet200Response {
    pub fn new() -> BirddogptzsetupGet200Response {
        BirddogptzsetupGet200Response {
            pan_speed: None,
            tilt_speed: None,
            zoom_speed: None,
        }
    }
}
