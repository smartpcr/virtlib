// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ResourceAllocationSettingData_ConsumerVisibility
//////////////////////////////////////////////

/// ResourceAllocationSettingData_ConsumerVisibility enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ResourceAllocationSettingData_ConsumerVisibility {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Passed_Through
    #[serde(rename = "Passed_Through")]
    PassedThrough = 2,
    /// Virtualized
    #[serde(rename = "Virtualized")]
    Virtualized = 3,
    /// Not_represented
    #[serde(rename = "Not_represented")]
    NotRepresented = 4,
    /// DMTF_reserved
    #[serde(rename = "DMTF_reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for ResourceAllocationSettingData_ConsumerVisibility {
    fn default() -> Self {
        Self::Unknown
    }
}

