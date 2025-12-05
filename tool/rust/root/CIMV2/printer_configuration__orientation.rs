// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_Orientation
//////////////////////////////////////////////

/// PrinterConfiguration_Orientation enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_Orientation {
    /// Portrait
    #[serde(rename = "Portrait")]
    Portrait = 1,
    /// Landscape
    #[serde(rename = "Landscape")]
    Landscape = 2,
}

impl Default for PrinterConfiguration_Orientation {
    fn default() -> Self {
        Self::Portrait
    }
}

