// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SessionStaticInfo_Type
//////////////////////////////////////////////

/// SessionStaticInfo_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SessionStaticInfo_Type {
    /// discoverySession
    #[serde(rename = "discoverySession")]
    DiscoverySession = 0,
    /// informationalSession
    #[serde(rename = "informationalSession")]
    InformationalSession = 1,
    /// dataSession
    #[serde(rename = "dataSession")]
    DataSession = 2,
    /// bootSession
    #[serde(rename = "bootSession")]
    BootSession = 3,
}

impl Default for SessionStaticInfo_Type {
    fn default() -> Self {
        Self::DiscoverySession
    }
}

