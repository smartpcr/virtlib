// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DisplayController_VideoArchitecture
//////////////////////////////////////////////

/// DisplayController_VideoArchitecture enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DisplayController_VideoArchitecture {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// CGA
    #[serde(rename = "CGA")]
    CGA = 2,
    /// EGA
    #[serde(rename = "EGA")]
    EGA = 3,
    /// VGA
    #[serde(rename = "VGA")]
    VGA = 4,
    /// SVGA
    #[serde(rename = "SVGA")]
    SVGA = 5,
    /// MDA
    #[serde(rename = "MDA")]
    MDA = 6,
    /// HGC
    #[serde(rename = "HGC")]
    HGC = 7,
    /// MCGA
    #[serde(rename = "MCGA")]
    MCGA = 8,
    /// _8514A
    #[serde(rename = "_8514A")]
    V8514A = 9,
    /// XGA
    #[serde(rename = "XGA")]
    XGA = 10,
    /// Linear_Frame_Buffer
    #[serde(rename = "Linear_Frame_Buffer")]
    LinearFrameBuffer = 11,
    /// PC_98
    #[serde(rename = "PC_98")]
    PC98 = 160,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 161,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 162,
}

impl Default for DisplayController_VideoArchitecture {
    fn default() -> Self {
        Self::Unknown
    }
}

