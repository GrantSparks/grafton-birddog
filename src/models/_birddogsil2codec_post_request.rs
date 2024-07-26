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
pub struct Birddogsil2codecPostRequest {
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "BitrateControl", skip_serializing_if = "Option::is_none")]
    pub bitrate_control: Option<String>,
    #[serde(rename = "ModeSel", skip_serializing_if = "Option::is_none")]
    pub mode_sel: Option<String>,
    #[serde(rename = "QuantFactorI", skip_serializing_if = "Option::is_none")]
    pub quant_factor_i: Option<String>,
    #[serde(rename = "QuantFactorP", skip_serializing_if = "Option::is_none")]
    pub quant_factor_p: Option<String>,
    #[serde(rename = "GOPSize", skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<String>,
    #[serde(rename = "Bitrate", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<String>,
}

impl Birddogsil2codecPostRequest {
    pub fn new() -> Birddogsil2codecPostRequest {
        Birddogsil2codecPostRequest {
            protocol: None,
            bitrate_control: None,
            mode_sel: None,
            quant_factor_i: None,
            quant_factor_p: None,
            gop_size: None,
            bitrate: None,
        }
    }
}
