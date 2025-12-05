// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PortalInfo_PortalType
//////////////////////////////////////////////

/// PortalInfo_PortalType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PortalInfo_PortalType {
    /// Initiator
    #[serde(rename = "Initiator")]
    Initiator = 0,
    /// Target
    #[serde(rename = "Target")]
    Target = 1,
}

impl Default for PortalInfo_PortalType {
    fn default() -> Self {
        Self::Initiator
    }
}

