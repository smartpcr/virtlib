// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PhysicalDisk_UniqueIdFormat
//////////////////////////////////////////////

/// PhysicalDisk_UniqueIdFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PhysicalDisk_UniqueIdFormat {
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 0,
    /// Vendor_Id
    #[serde(rename = "Vendor_Id")]
    VendorId = 1,
    /// EUI64
    #[serde(rename = "EUI64")]
    EUI64 = 2,
    /// FCPH_Name
    #[serde(rename = "FCPH_Name")]
    FCPHName = 3,
    /// SCSI_Name_String
    #[serde(rename = "SCSI_Name_String")]
    SCSINameString = 8,
}

impl Default for PhysicalDisk_UniqueIdFormat {
    fn default() -> Self {
        Self::VendorSpecific
    }
}

