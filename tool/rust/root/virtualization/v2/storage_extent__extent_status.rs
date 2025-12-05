// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageExtent_ExtentStatus
//////////////////////////////////////////////

/// StorageExtent_ExtentStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageExtent_ExtentStatus {
    /// Other
    #[serde(rename = "Other")]
    Other = 0,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 1,
    /// None_Not_Applicable
    #[serde(rename = "None_Not_Applicable")]
    NoneNotApplicable = 2,
    /// Broken
    #[serde(rename = "Broken")]
    Broken = 3,
    /// Data_Lost
    #[serde(rename = "Data_Lost")]
    DataLost = 4,
    /// Dynamic_Reconfig
    #[serde(rename = "Dynamic_Reconfig")]
    DynamicReconfig = 5,
    /// Exposed
    #[serde(rename = "Exposed")]
    Exposed = 6,
    /// Fractionally_Exposed
    #[serde(rename = "Fractionally_Exposed")]
    FractionallyExposed = 7,
    /// Partially_Exposed
    #[serde(rename = "Partially_Exposed")]
    PartiallyExposed = 8,
    /// Protection_Disabled
    #[serde(rename = "Protection_Disabled")]
    ProtectionDisabled = 9,
    /// Readying
    #[serde(rename = "Readying")]
    Readying = 10,
    /// Rebuild
    #[serde(rename = "Rebuild")]
    Rebuild = 11,
    /// Recalculate
    #[serde(rename = "Recalculate")]
    Recalculate = 12,
    /// Spare_in_Use
    #[serde(rename = "Spare_in_Use")]
    SpareInUse = 13,
    /// Verify_In_Progress
    #[serde(rename = "Verify_In_Progress")]
    VerifyInProgress = 14,
    /// In_Band_Access_Granted
    #[serde(rename = "In_Band_Access_Granted")]
    InBandAccessGranted = 15,
    /// Imported
    #[serde(rename = "Imported")]
    Imported = 16,
    /// Exported
    #[serde(rename = "Exported")]
    Exported = 17,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 18,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 19,
}

impl Default for StorageExtent_ExtentStatus {
    fn default() -> Self {
        Self::Other
    }
}

