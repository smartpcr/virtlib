// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageNode_NameFormat
//////////////////////////////////////////////

/// StorageNode_NameFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageNode_NameFormat {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// IP
    #[serde(rename = "IP")]
    IP = 2,
    /// Dial
    #[serde(rename = "Dial")]
    Dial = 3,
    /// HID
    #[serde(rename = "HID")]
    HID = 4,
    /// NWA
    #[serde(rename = "NWA")]
    NWA = 5,
    /// HWA
    #[serde(rename = "HWA")]
    HWA = 6,
    /// X25
    #[serde(rename = "X25")]
    X25 = 7,
    /// ISDN
    #[serde(rename = "ISDN")]
    ISDN = 8,
    /// IPX
    #[serde(rename = "IPX")]
    IPX = 9,
    /// DCC
    #[serde(rename = "DCC")]
    DCC = 10,
    /// ICD
    #[serde(rename = "ICD")]
    ICD = 11,
    /// E_164
    #[serde(rename = "E_164")]
    E164 = 12,
    /// SNA
    #[serde(rename = "SNA")]
    SNA = 13,
    /// OID_OSI
    #[serde(rename = "OID_OSI")]
    OIDOSI = 14,
    /// WWN
    #[serde(rename = "WWN")]
    WWN = 15,
    /// NAA
    #[serde(rename = "NAA")]
    NAA = 16,
}

impl Default for StorageNode_NameFormat {
    fn default() -> Self {
        Self::Other
    }
}

