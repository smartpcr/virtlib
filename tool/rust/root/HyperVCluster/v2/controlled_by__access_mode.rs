// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ControlledBy_AccessMode
//////////////////////////////////////////////

/// ControlledBy_AccessMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ControlledBy_AccessMode {
    /// ReadWrite
    #[serde(rename = "ReadWrite")]
    ReadWrite = 2,
    /// ReadOnly
    #[serde(rename = "ReadOnly")]
    ReadOnly = 3,
    /// NoAccess
    #[serde(rename = "NoAccess")]
    NoAccess = 4,
}

impl Default for ControlledBy_AccessMode {
    fn default() -> Self {
        Self::ReadWrite
    }
}

