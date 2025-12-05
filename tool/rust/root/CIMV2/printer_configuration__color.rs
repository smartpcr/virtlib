// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_Color
//////////////////////////////////////////////

/// PrinterConfiguration_Color enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_Color {
    /// Monochrome
    #[serde(rename = "Monochrome")]
    Monochrome = 1,
    /// Color
    #[serde(rename = "Color")]
    Color = 2,
}

impl Default for PrinterConfiguration_Color {
    fn default() -> Self {
        Self::Monochrome
    }
}

