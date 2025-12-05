// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DynamicForwardingEntry_DynamicStatus
//////////////////////////////////////////////

/// DynamicForwardingEntry_DynamicStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DynamicForwardingEntry_DynamicStatus {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Invalid
    #[serde(rename = "Invalid")]
    Invalid = 2,
    /// Learned
    #[serde(rename = "Learned")]
    Learned = 3,
    /// Self
    #[serde(rename = "Self")]
    SelfValue = 4,
    /// Mgmt
    #[serde(rename = "Mgmt")]
    Mgmt = 5,
}

impl Default for DynamicForwardingEntry_DynamicStatus {
    fn default() -> Self {
        Self::Other
    }
}

