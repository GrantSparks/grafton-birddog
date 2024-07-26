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
pub struct Birddogsil2encGet200ResponseRtmp {
    #[serde(rename = "AuthEnable", skip_serializing_if = "Option::is_none")]
    pub auth_enable: Option<String>,
    #[serde(rename = "ConnectionURL", skip_serializing_if = "Option::is_none")]
    pub connection_url: Option<String>,
    #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(rename = "ServerSelection", skip_serializing_if = "Option::is_none")]
    pub server_selection: Option<String>,
    #[serde(rename = "StreamKeyLocal", skip_serializing_if = "Option::is_none")]
    pub stream_key_local: Option<String>,
    #[serde(rename = "StreamKeyRemote", skip_serializing_if = "Option::is_none")]
    pub stream_key_remote: Option<String>,
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl Birddogsil2encGet200ResponseRtmp {
    pub fn new() -> Birddogsil2encGet200ResponseRtmp {
        Birddogsil2encGet200ResponseRtmp {
            auth_enable: None,
            connection_url: None,
            password: None,
            server: None,
            server_selection: None,
            stream_key_local: None,
            stream_key_remote: None,
            user_name: None,
        }
    }
}
