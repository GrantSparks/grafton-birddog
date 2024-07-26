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
pub struct EncodesetupGet200Response {
    #[serde(rename = "ChNum", skip_serializing_if = "Option::is_none")]
    pub ch_num: Option<String>,
    #[serde(rename = "VideoFormat", skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>,
    #[serde(rename = "VideoSampleRate", skip_serializing_if = "Option::is_none")]
    pub video_sample_rate: Option<String>,
    #[serde(rename = "ColorBitDepth", skip_serializing_if = "Option::is_none")]
    pub color_bit_depth: Option<String>,
    #[serde(rename = "StreamName", skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "NDIAudio", skip_serializing_if = "Option::is_none")]
    pub ndi_audio: Option<String>,
    #[serde(rename = "ScreenSaverMode", skip_serializing_if = "Option::is_none")]
    pub screen_saver_mode: Option<String>,
    #[serde(rename = "BandwidthMode", skip_serializing_if = "Option::is_none")]
    pub bandwidth_mode: Option<String>,
    #[serde(rename = "BandwidthSelect", skip_serializing_if = "Option::is_none")]
    pub bandwidth_select: Option<String>,
    #[serde(rename = "LoopTally", skip_serializing_if = "Option::is_none")]
    pub loop_tally: Option<String>,
    #[serde(rename = "TallyMode", skip_serializing_if = "Option::is_none")]
    pub tally_mode: Option<String>,
    #[serde(rename = "VideoCSC", skip_serializing_if = "Option::is_none")]
    pub video_csc: Option<String>,
    #[serde(rename = "NDIGroup", skip_serializing_if = "Option::is_none")]
    pub ndi_group: Option<String>,
    #[serde(rename = "NDIGroupName", skip_serializing_if = "Option::is_none")]
    pub ndi_group_name: Option<String>,
}

impl EncodesetupGet200Response {
    pub fn new() -> EncodesetupGet200Response {
        EncodesetupGet200Response {
            ch_num: None,
            video_format: None,
            video_sample_rate: None,
            color_bit_depth: None,
            stream_name: None,
            ndi_audio: None,
            screen_saver_mode: None,
            bandwidth_mode: None,
            bandwidth_select: None,
            loop_tally: None,
            tally_mode: None,
            video_csc: None,
            ndi_group: None,
            ndi_group_name: None,
        }
    }
}
