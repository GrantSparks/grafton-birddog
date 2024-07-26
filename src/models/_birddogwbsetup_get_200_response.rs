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
pub struct BirddogwbsetupGet200Response {
    #[serde(rename = "BG", skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    #[serde(rename = "BR", skip_serializing_if = "Option::is_none")]
    pub br: Option<String>,
    #[serde(rename = "BlueGain", skip_serializing_if = "Option::is_none")]
    pub blue_gain: Option<String>,
    #[serde(rename = "ColorTemp", skip_serializing_if = "Option::is_none")]
    pub color_temp: Option<String>,
    #[serde(rename = "GB", skip_serializing_if = "Option::is_none")]
    pub gb: Option<String>,
    #[serde(rename = "GR", skip_serializing_if = "Option::is_none")]
    pub gr: Option<String>,
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "Matrix", skip_serializing_if = "Option::is_none")]
    pub matrix: Option<String>,
    #[serde(rename = "Offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(rename = "Phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(rename = "RB", skip_serializing_if = "Option::is_none")]
    pub rb: Option<String>,
    #[serde(rename = "RG", skip_serializing_if = "Option::is_none")]
    pub rg: Option<String>,
    #[serde(rename = "RedGain", skip_serializing_if = "Option::is_none")]
    pub red_gain: Option<String>,
    #[serde(rename = "Select", skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    #[serde(rename = "Speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<String>,
    #[serde(rename = "WbMode", skip_serializing_if = "Option::is_none")]
    pub wb_mode: Option<String>,
}

impl BirddogwbsetupGet200Response {
    pub fn new() -> BirddogwbsetupGet200Response {
        BirddogwbsetupGet200Response {
            bg: None,
            br: None,
            blue_gain: None,
            color_temp: None,
            gb: None,
            gr: None,
            level: None,
            matrix: None,
            offset: None,
            phase: None,
            rb: None,
            rg: None,
            red_gain: None,
            select: None,
            speed: None,
            wb_mode: None,
        }
    }
}

