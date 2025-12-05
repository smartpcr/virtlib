// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_TTOption
//////////////////////////////////////////////

/// PrinterConfiguration_TTOption enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_TTOption {
    /// Bitmap
    #[serde(rename = "Bitmap")]
    Bitmap = 1,
    /// Download
    #[serde(rename = "Download")]
    Download = 2,
    /// Substitute
    #[serde(rename = "Substitute")]
    Substitute = 3,
}

impl Default for PrinterConfiguration_TTOption {
    fn default() -> Self {
        Self::Bitmap
    }
}

