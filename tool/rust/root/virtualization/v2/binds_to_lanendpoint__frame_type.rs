// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BindsToLANEndpoint_FrameType
//////////////////////////////////////////////

/// BindsToLANEndpoint_FrameType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BindsToLANEndpoint_FrameType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Ethernet
    #[serde(rename = "Ethernet")]
    Ethernet = 1,
    /// _802_2
    #[serde(rename = "_802_2")]
    V8022 = 2,
    /// SNAP
    #[serde(rename = "SNAP")]
    SNAP = 3,
    /// Raw802_3
    #[serde(rename = "Raw802_3")]
    Raw8023 = 4,
}

impl Default for BindsToLANEndpoint_FrameType {
    fn default() -> Self {
        Self::Unknown
    }
}

