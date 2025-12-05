// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_Architecture
//////////////////////////////////////////////

/// VirtualSystemSettingData_Architecture enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_Architecture {
    /// x64
    #[serde(rename = "x64")]
    X64 = 0,
    /// arm64
    #[serde(rename = "arm64")]
    Arm64 = 1,
}

impl Default for VirtualSystemSettingData_Architecture {
    fn default() -> Self {
        Self::X64
    }
}

