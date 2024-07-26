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
pub struct NdiDisServerGet200Response {
    #[serde(rename = "NDIDisServ", skip_serializing_if = "Option::is_none")]
    pub ndi_dis_serv: Option<String>,
    #[serde(rename = "NDIDisServIP", skip_serializing_if = "Option::is_none")]
    pub ndi_dis_serv_ip: Option<String>,
}

impl NdiDisServerGet200Response {
    pub fn new() -> NdiDisServerGet200Response {
        NdiDisServerGet200Response {
            ndi_dis_serv: None,
            ndi_dis_serv_ip: None,
        }
    }
}
