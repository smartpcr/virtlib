// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ImapiV1Shim_Flags
//////////////////////////////////////////////

/// ImapiV1Shim_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ImapiV1Shim_Flags {
    /// General
    #[serde(rename = "General")]
    General = 1,
}

impl Default for ImapiV1Shim_Flags {
    fn default() -> Self {
        Self::General
    }
}

