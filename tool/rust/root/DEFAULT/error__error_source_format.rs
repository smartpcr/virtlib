// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Error_ErrorSourceFormat
//////////////////////////////////////////////

/// Error_ErrorSourceFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Error_ErrorSourceFormat {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// CIMObjectPath
    #[serde(rename = "CIMObjectPath")]
    CIMObjectPath = 2,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 3,
}

impl Default for Error_ErrorSourceFormat {
    fn default() -> Self {
        Self::Unknown
    }
}

