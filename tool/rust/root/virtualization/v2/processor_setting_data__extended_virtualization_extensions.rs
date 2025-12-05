// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ProcessorSettingData_ExtendedVirtualizationExtensions
//////////////////////////////////////////////

/// ProcessorSettingData_ExtendedVirtualizationExtensions enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ProcessorSettingData_ExtendedVirtualizationExtensions {
    /// HardwareIsolation
    #[serde(rename = "HardwareIsolation")]
    HardwareIsolation = 1,
}

impl Default for ProcessorSettingData_ExtendedVirtualizationExtensions {
    fn default() -> Self {
        Self::HardwareIsolation
    }
}

