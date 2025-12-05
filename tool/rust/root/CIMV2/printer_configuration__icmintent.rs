// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_ICMIntent
//////////////////////////////////////////////

/// PrinterConfiguration_ICMIntent enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_ICMIntent {
    /// Saturation
    #[serde(rename = "Saturation")]
    Saturation = 1,
    /// Contrast
    #[serde(rename = "Contrast")]
    Contrast = 2,
    /// Exact_Color
    #[serde(rename = "Exact_Color")]
    ExactColor = 3,
}

impl Default for PrinterConfiguration_ICMIntent {
    fn default() -> Self {
        Self::Saturation
    }
}

