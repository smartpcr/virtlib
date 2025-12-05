// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemReferencePointService_ReferencePointType
//////////////////////////////////////////////

/// VirtualSystemReferencePointService_ReferencePointType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemReferencePointService_ReferencePointType {
    /// Log_based
    #[serde(rename = "Log_based")]
    LogBased = 0,
    /// RCT_based
    #[serde(rename = "RCT_based")]
    RCTBased = 1,
}

impl Default for VirtualSystemReferencePointService_ReferencePointType {
    fn default() -> Self {
        Self::LogBased
    }
}

