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
pub struct DecodestatusGet200Response {
    #[serde(rename = "Videoresolution", skip_serializing_if = "Option::is_none")]
    pub videoresolution: Option<String>,
    #[serde(rename = "VideoFramerate", skip_serializing_if = "Option::is_none")]
    pub video_framerate: Option<String>,
    #[serde(rename = "VideoSamplerate", skip_serializing_if = "Option::is_none")]
    pub video_samplerate: Option<String>,
    #[serde(rename = "Audiochannels", skip_serializing_if = "Option::is_none")]
    pub audiochannels: Option<String>,
    #[serde(rename = "AudioSamplerate", skip_serializing_if = "Option::is_none")]
    pub audio_samplerate: Option<String>,
    #[serde(rename = "AverageBitrate", skip_serializing_if = "Option::is_none")]
    pub average_bitrate: Option<String>,
}

impl DecodestatusGet200Response {
    pub fn new() -> DecodestatusGet200Response {
        DecodestatusGet200Response {
            videoresolution: None,
            video_framerate: None,
            video_samplerate: None,
            audiochannels: None,
            audio_samplerate: None,
            average_bitrate: None,
        }
    }
}

