// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VLANEndpoint_GVRPStatus
//////////////////////////////////////////////

/// VLANEndpoint_GVRPStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VLANEndpoint_GVRPStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 2,
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 3,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 4,
}

impl Default for VLANEndpoint_GVRPStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

