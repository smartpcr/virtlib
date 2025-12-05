// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_DitherType
//////////////////////////////////////////////

/// PrinterConfiguration_DitherType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_DitherType {
    /// No_Dithering
    #[serde(rename = "No_Dithering")]
    NoDithering = 1,
    /// Coarse_Brush
    #[serde(rename = "Coarse_Brush")]
    CoarseBrush = 2,
    /// Fine_Brush
    #[serde(rename = "Fine_Brush")]
    FineBrush = 3,
    /// Line_Art
    #[serde(rename = "Line_Art")]
    LineArt = 4,
    /// Greyscale
    #[serde(rename = "Greyscale")]
    Greyscale = 5,
}

impl Default for PrinterConfiguration_DitherType {
    fn default() -> Self {
        Self::NoDithering
    }
}

