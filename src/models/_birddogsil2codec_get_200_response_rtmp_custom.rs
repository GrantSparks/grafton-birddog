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
pub struct Birddogsil2codecGet200ResponseRtmpCustom {
    #[serde(rename = "Bitrate", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<String>,
    #[serde(rename = "GOPSize", skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<String>,
    #[serde(rename = "QuantFactorI", skip_serializing_if = "Option::is_none")]
    pub quant_factor_i: Option<String>,
    #[serde(rename = "QuantFactorP", skip_serializing_if = "Option::is_none")]
    pub quant_factor_p: Option<String>,
}

impl Birddogsil2codecGet200ResponseRtmpCustom {
    pub fn new() -> Birddogsil2codecGet200ResponseRtmpCustom {
        Birddogsil2codecGet200ResponseRtmpCustom {
            bitrate: None,
            gop_size: None,
            quant_factor_i: None,
            quant_factor_p: None,
        }
    }
}
