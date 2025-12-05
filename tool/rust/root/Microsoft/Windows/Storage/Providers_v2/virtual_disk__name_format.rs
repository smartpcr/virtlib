// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDisk_NameFormat
//////////////////////////////////////////////

/// VirtualDisk_NameFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDisk_NameFormat {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// VPD83NAA6
    #[serde(rename = "VPD83NAA6")]
    VPD83NAA6 = 2,
    /// VPD83NAA5
    #[serde(rename = "VPD83NAA5")]
    VPD83NAA5 = 3,
    /// VPD83Type2
    #[serde(rename = "VPD83Type2")]
    VPD83Type2 = 4,
    /// VPD83Type1
    #[serde(rename = "VPD83Type1")]
    VPD83Type1 = 5,
    /// VPD83Type0
    #[serde(rename = "VPD83Type0")]
    VPD83Type0 = 6,
    /// SNVM
    #[serde(rename = "SNVM")]
    SNVM = 7,
    /// NodeWWN
    #[serde(rename = "NodeWWN")]
    NodeWWN = 8,
    /// NAA
    #[serde(rename = "NAA")]
    NAA = 9,
    /// EUI64
    #[serde(rename = "EUI64")]
    EUI64 = 10,
    /// T10VID
    #[serde(rename = "T10VID")]
    T10VID = 11,
}

impl Default for VirtualDisk_NameFormat {
    fn default() -> Self {
        Self::Unknown
    }
}

