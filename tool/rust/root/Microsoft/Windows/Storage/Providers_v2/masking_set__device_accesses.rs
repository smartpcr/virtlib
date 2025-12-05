// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MaskingSet_DeviceAccesses
//////////////////////////////////////////////

/// MaskingSet_DeviceAccesses enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MaskingSet_DeviceAccesses {
    /// Read_Write
    #[serde(rename = "Read_Write")]
    ReadWrite = 2,
    /// Read_Only
    #[serde(rename = "Read_Only")]
    ReadOnly = 3,
    /// No_Access
    #[serde(rename = "No_Access")]
    NoAccess = 4,
}

impl Default for MaskingSet_DeviceAccesses {
    fn default() -> Self {
        Self::ReadWrite
    }
}

