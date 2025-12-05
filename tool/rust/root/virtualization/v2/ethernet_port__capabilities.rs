// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EthernetPort_Capabilities
//////////////////////////////////////////////

/// EthernetPort_Capabilities enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EthernetPort_Capabilities {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// AlertOnLan
    #[serde(rename = "AlertOnLan")]
    AlertOnLan = 2,
    /// WakeOnLan
    #[serde(rename = "WakeOnLan")]
    WakeOnLan = 3,
    /// FailOver
    #[serde(rename = "FailOver")]
    FailOver = 4,
    /// LoadBalancing
    #[serde(rename = "LoadBalancing")]
    LoadBalancing = 5,
}

impl Default for EthernetPort_Capabilities {
    fn default() -> Self {
        Self::Unknown
    }
}

