// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_PrintQuality
//////////////////////////////////////////////

/// PrinterConfiguration_PrintQuality enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_PrintQuality {
    /// Draft
    #[serde(rename = "Draft")]
    Draft = -1,
    /// Low
    #[serde(rename = "Low")]
    Low = -2,
    /// Medium
    #[serde(rename = "Medium")]
    Medium = -3,
    /// High
    #[serde(rename = "High")]
    High = -4,
}

impl Default for PrinterConfiguration_PrintQuality {
    fn default() -> Self {
        Self::Draft
    }
}

