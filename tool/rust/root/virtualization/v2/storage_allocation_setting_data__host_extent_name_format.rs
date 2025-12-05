// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageAllocationSettingData_HostExtentNameFormat
//////////////////////////////////////////////

/// StorageAllocationSettingData_HostExtentNameFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageAllocationSettingData_HostExtentNameFormat {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// SNVM
    #[serde(rename = "SNVM")]
    SNVM = 7,
    /// NAA
    #[serde(rename = "NAA")]
    NAA = 9,
    /// EUI64
    #[serde(rename = "EUI64")]
    EUI64 = 10,
    /// T10VID
    #[serde(rename = "T10VID")]
    T10VID = 11,
    /// OS_Device_Name
    #[serde(rename = "OS_Device_Name")]
    OSDeviceName = 12,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 13,
}

impl Default for StorageAllocationSettingData_HostExtentNameFormat {
    fn default() -> Self {
        Self::Unknown
    }
}

