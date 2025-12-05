// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WiFiEndpoint_AuthenticationMethod
//////////////////////////////////////////////

/// WiFiEndpoint_AuthenticationMethod enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WiFiEndpoint_AuthenticationMethod {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Open_System
    #[serde(rename = "Open_System")]
    OpenSystem = 2,
    /// Shared_Key
    #[serde(rename = "Shared_Key")]
    SharedKey = 3,
    /// WPA_PSK
    #[serde(rename = "WPA_PSK")]
    WPAPSK = 4,
    /// WPA_IEEE_802_1x
    #[serde(rename = "WPA_IEEE_802_1x")]
    WPAIEEE8021x = 5,
    /// WPA2_PSK
    #[serde(rename = "WPA2_PSK")]
    WPA2PSK = 6,
    /// WPA2_IEEE_802_1x
    #[serde(rename = "WPA2_IEEE_802_1x")]
    WPA2IEEE8021x = 7,
    /// CCKM_IEEE_802_1x
    #[serde(rename = "CCKM_IEEE_802_1x")]
    CCKMIEEE8021x = 8,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 9,
}

impl Default for WiFiEndpoint_AuthenticationMethod {
    fn default() -> Self {
        Self::Unknown
    }
}

