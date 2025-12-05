// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PhysicalDisk_Usage
//////////////////////////////////////////////

/// PhysicalDisk_Usage enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PhysicalDisk_Usage {
    /// Auto_Select
    #[serde(rename = "Auto_Select")]
    AutoSelect = 1,
    /// Manual_Select
    #[serde(rename = "Manual_Select")]
    ManualSelect = 2,
    /// Hot_Spare
    #[serde(rename = "Hot_Spare")]
    HotSpare = 3,
    /// Retired
    #[serde(rename = "Retired")]
    Retired = 4,
    /// Journal
    #[serde(rename = "Journal")]
    Journal = 5,
}

impl Default for PhysicalDisk_Usage {
    fn default() -> Self {
        Self::AutoSelect
    }
}

