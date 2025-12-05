// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ResourceAllocationSettingData_MappingBehavior
//////////////////////////////////////////////

/// ResourceAllocationSettingData_MappingBehavior enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ResourceAllocationSettingData_MappingBehavior {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Not_Supported
    #[serde(rename = "Not_Supported")]
    NotSupported = 2,
    /// Dedicated
    #[serde(rename = "Dedicated")]
    Dedicated = 3,
    /// Soft_Affinity
    #[serde(rename = "Soft_Affinity")]
    SoftAffinity = 4,
    /// Hard_Affinity
    #[serde(rename = "Hard_Affinity")]
    HardAffinity = 5,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 6,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 7,
}

impl Default for ResourceAllocationSettingData_MappingBehavior {
    fn default() -> Self {
        Self::Unknown
    }
}

