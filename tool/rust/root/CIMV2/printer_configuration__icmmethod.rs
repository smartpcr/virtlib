// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_ICMMethod
//////////////////////////////////////////////

/// PrinterConfiguration_ICMMethod enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_ICMMethod {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 1,
    /// Windows
    #[serde(rename = "Windows")]
    Windows = 2,
    /// Device_Driver
    #[serde(rename = "Device_Driver")]
    DeviceDriver = 3,
    /// Device
    #[serde(rename = "Device")]
    Device = 4,
}

impl Default for PrinterConfiguration_ICMMethod {
    fn default() -> Self {
        Self::Disabled
    }
}

