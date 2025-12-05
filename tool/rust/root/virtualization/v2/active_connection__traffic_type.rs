// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ActiveConnection_TrafficType
//////////////////////////////////////////////

/// ActiveConnection_TrafficType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ActiveConnection_TrafficType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unicast
    #[serde(rename = "Unicast")]
    Unicast = 2,
    /// Broadcast
    #[serde(rename = "Broadcast")]
    Broadcast = 3,
    /// Multicast
    #[serde(rename = "Multicast")]
    Multicast = 4,
    /// Anycast
    #[serde(rename = "Anycast")]
    Anycast = 5,
}

impl Default for ActiveConnection_TrafficType {
    fn default() -> Self {
        Self::Unknown
    }
}

