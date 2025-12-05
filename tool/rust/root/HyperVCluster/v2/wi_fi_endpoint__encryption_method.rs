// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WiFiEndpoint_EncryptionMethod
//////////////////////////////////////////////

/// WiFiEndpoint_EncryptionMethod enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WiFiEndpoint_EncryptionMethod {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// WEP
    #[serde(rename = "WEP")]
    WEP = 2,
    /// TKIP
    #[serde(rename = "TKIP")]
    TKIP = 3,
    /// CCMP
    #[serde(rename = "CCMP")]
    CCMP = 4,
    /// None
    #[serde(rename = "None")]
    None = 5,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 6,
}

impl Default for WiFiEndpoint_EncryptionMethod {
    fn default() -> Self {
        Self::Unknown
    }
}

