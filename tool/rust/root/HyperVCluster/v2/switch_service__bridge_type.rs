// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SwitchService_BridgeType
//////////////////////////////////////////////

/// SwitchService_BridgeType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SwitchService_BridgeType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 1,
    /// Transparent_only
    #[serde(rename = "Transparent_only")]
    TransparentOnly = 2,
    /// SourceRoute_only
    #[serde(rename = "SourceRoute_only")]
    SourceRouteOnly = 3,
    /// SRT
    #[serde(rename = "SRT")]
    SRT = 4,
}

impl Default for SwitchService_BridgeType {
    fn default() -> Self {
        Self::Unknown
    }
}

