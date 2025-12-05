// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_VirtualSlitType
//////////////////////////////////////////////

/// VirtualSystemSettingData_VirtualSlitType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_VirtualSlitType {
    /// None
    #[serde(rename = "None")]
    None = 0,
    /// Firmware
    #[serde(rename = "Firmware")]
    Firmware = 1,
    /// Measured
    #[serde(rename = "Measured")]
    Measured = 2,
    /// FirmwareFallbackMeasured
    #[serde(rename = "FirmwareFallbackMeasured")]
    FirmwareFallbackMeasured = 3,
}

impl Default for VirtualSystemSettingData_VirtualSlitType {
    fn default() -> Self {
        Self::None
    }
}

