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
pub struct AnalogaudiosetupPostRequest {
    /// Audio in gain Range 0 to 100
    #[serde(rename = "AnalogAudioInGain", skip_serializing_if = "Option::is_none")]
    pub analog_audio_in_gain: Option<String>,
    /// Audio out gain Range 0 to 100
    #[serde(rename = "AnalogAudioOutGain", skip_serializing_if = "Option::is_none")]
    pub analog_audio_out_gain: Option<String>,
    /// DecodeMain, DecodeComms, DecodeLoop
    #[serde(rename = "AnalogAudiooutputselect", skip_serializing_if = "Option::is_none")]
    pub analog_audiooutputselect: Option<String>,
}

impl AnalogaudiosetupPostRequest {
    pub fn new() -> AnalogaudiosetupPostRequest {
        AnalogaudiosetupPostRequest {
            analog_audio_in_gain: None,
            analog_audio_out_gain: None,
            analog_audiooutputselect: None,
        }
    }
}
