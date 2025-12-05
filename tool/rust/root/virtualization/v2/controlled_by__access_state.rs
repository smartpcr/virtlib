// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ControlledBy_AccessState
//////////////////////////////////////////////

/// ControlledBy_AccessState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ControlledBy_AccessState {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Active
    #[serde(rename = "Active")]
    Active = 1,
    /// Inactive
    #[serde(rename = "Inactive")]
    Inactive = 2,
}

impl Default for ControlledBy_AccessState {
    fn default() -> Self {
        Self::Unknown
    }
}

