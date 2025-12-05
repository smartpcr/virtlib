// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_ConsoleMode
//////////////////////////////////////////////

/// VirtualSystemSettingData_ConsoleMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_ConsoleMode {
    /// Default
    #[serde(rename = "Default")]
    Default = 0,
    /// COM1
    #[serde(rename = "COM1")]
    COM1 = 1,
    /// COM2
    #[serde(rename = "COM2")]
    COM2 = 2,
    /// None
    #[serde(rename = "None")]
    None = 3,
}

impl Default for VirtualSystemSettingData_ConsoleMode {
    fn default() -> Self {
        Self::Default
    }
}

