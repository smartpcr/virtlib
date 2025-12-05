// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source LANEndpoint_LANType
//////////////////////////////////////////////

/// LANEndpoint_LANType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum LANEndpoint_LANType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Ethernet
    #[serde(rename = "Ethernet")]
    Ethernet = 2,
    /// TokenRing
    #[serde(rename = "TokenRing")]
    TokenRing = 3,
    /// FDDI
    #[serde(rename = "FDDI")]
    FDDI = 4,
}

impl Default for LANEndpoint_LANType {
    fn default() -> Self {
        Self::Unknown
    }
}

