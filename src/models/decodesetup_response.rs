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
pub struct DecodesetupResponse {
    #[serde(rename = "ColorSpace", skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    #[serde(rename = "NDIAudio", skip_serializing_if = "Option::is_none")]
    pub ndi_audio: Option<String>,
    #[serde(rename = "ScreenSaverMode", skip_serializing_if = "Option::is_none")]
    pub screen_saver_mode: Option<String>,
    #[serde(rename = "TallyMode", skip_serializing_if = "Option::is_none")]
    pub tally_mode: Option<String>,
    #[serde(rename = "ChNum", skip_serializing_if = "Option::is_none")]
    pub ch_num: Option<String>,
}

impl DecodesetupResponse {
    pub fn new() -> DecodesetupResponse {
        DecodesetupResponse {
            color_space: None,
            ndi_audio: None,
            screen_saver_mode: None,
            tally_mode: None,
            ch_num: None,
        }
    }
}
