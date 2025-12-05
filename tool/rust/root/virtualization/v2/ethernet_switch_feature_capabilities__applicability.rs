// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EthernetSwitchFeatureCapabilities_Applicability
//////////////////////////////////////////////

/// EthernetSwitchFeatureCapabilities_Applicability enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EthernetSwitchFeatureCapabilities_Applicability {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Port
    #[serde(rename = "Port")]
    Port = 1,
    /// Switch
    #[serde(rename = "Switch")]
    Switch = 2,
}

impl Default for EthernetSwitchFeatureCapabilities_Applicability {
    fn default() -> Self {
        Self::Unknown
    }
}

