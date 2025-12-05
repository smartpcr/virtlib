// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrinterConfiguration_MediaType
//////////////////////////////////////////////

/// PrinterConfiguration_MediaType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrinterConfiguration_MediaType {
    /// Standard
    #[serde(rename = "Standard")]
    Standard = 1,
    /// Transparency
    #[serde(rename = "Transparency")]
    Transparency = 2,
    /// Glossy
    #[serde(rename = "Glossy")]
    Glossy = 3,
}

impl Default for PrinterConfiguration_MediaType {
    fn default() -> Self {
        Self::Standard
    }
}

