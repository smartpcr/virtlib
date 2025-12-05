// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Memory_ErrorInfo
//////////////////////////////////////////////

/// Memory_ErrorInfo enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Memory_ErrorInfo {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// OK
    #[serde(rename = "OK")]
    OK = 3,
    /// Bad_Read
    #[serde(rename = "Bad_Read")]
    BadRead = 4,
    /// Parity_Error
    #[serde(rename = "Parity_Error")]
    ParityError = 5,
    /// Single_Bit_Error
    #[serde(rename = "Single_Bit_Error")]
    SingleBitError = 6,
    /// Double_Bit_Error
    #[serde(rename = "Double_Bit_Error")]
    DoubleBitError = 7,
    /// Multi_Bit_Error
    #[serde(rename = "Multi_Bit_Error")]
    MultiBitError = 8,
    /// Nibble_Error
    #[serde(rename = "Nibble_Error")]
    NibbleError = 9,
    /// Checksum_Error
    #[serde(rename = "Checksum_Error")]
    ChecksumError = 10,
    /// CRC_Error
    #[serde(rename = "CRC_Error")]
    CRCError = 11,
    /// Undefined
    #[serde(rename = "Undefined")]
    Undefined = 12,
    /// Undefined1
    #[serde(rename = "Undefined1")]
    Undefined1 = 13,
    /// Undefined2
    #[serde(rename = "Undefined2")]
    Undefined2 = 14,
}

impl Default for Memory_ErrorInfo {
    fn default() -> Self {
        Self::Other
    }
}

