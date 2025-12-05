// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source LogicalPort_UsageRestriction
//////////////////////////////////////////////

/// LogicalPort_UsageRestriction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum LogicalPort_UsageRestriction {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Front_end_only
    #[serde(rename = "Front_end_only")]
    FrontEndOnly = 2,
    /// Back_end_only
    #[serde(rename = "Back_end_only")]
    BackEndOnly = 3,
    /// Not_restricted
    #[serde(rename = "Not_restricted")]
    NotRestricted = 4,
}

impl Default for LogicalPort_UsageRestriction {
    fn default() -> Self {
        Self::Unknown
    }
}

