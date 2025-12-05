// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SwitchService_BridgeAddressType
//////////////////////////////////////////////

/// SwitchService_BridgeAddressType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SwitchService_BridgeAddressType {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// IPv4
    #[serde(rename = "IPv4")]
    IPv4 = 2,
    /// IPv6
    #[serde(rename = "IPv6")]
    IPv6 = 3,
    /// MAC
    #[serde(rename = "MAC")]
    MAC = 4,
    /// MAC_plus_Spanning_Tree_Priority
    #[serde(rename = "MAC_plus_Spanning_Tree_Priority")]
    MACPlusSpanningTreePriority = 5,
}

impl Default for SwitchService_BridgeAddressType {
    fn default() -> Self {
        Self::Other
    }
}

