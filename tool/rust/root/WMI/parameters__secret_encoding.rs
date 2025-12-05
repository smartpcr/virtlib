// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Parameters_SecretEncoding
//////////////////////////////////////////////

/// Parameters_SecretEncoding enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Parameters_SecretEncoding {
    /// Printable_ASCII
    #[serde(rename = "Printable_ASCII")]
    PrintableASCII = 1,
    /// Binary
    #[serde(rename = "Binary")]
    Binary = 2,
}

impl Default for Parameters_SecretEncoding {
    fn default() -> Self {
        Self::PrintableASCII
    }
}

