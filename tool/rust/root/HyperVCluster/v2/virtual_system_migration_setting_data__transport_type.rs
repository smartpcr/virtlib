// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemMigrationSettingData_TransportType
//////////////////////////////////////////////

/// VirtualSystemMigrationSettingData_TransportType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemMigrationSettingData_TransportType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// SSH
    #[serde(rename = "SSH")]
    SSH = 2,
    /// TLS
    #[serde(rename = "TLS")]
    TLS = 3,
    /// TLS_Strict
    #[serde(rename = "TLS_Strict")]
    TLSStrict = 4,
    /// TCP
    #[serde(rename = "TCP")]
    TCP = 5,
    /// IPC
    #[serde(rename = "IPC")]
    IPC = 6,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 7,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 8,
}

impl Default for VirtualSystemMigrationSettingData_TransportType {
    fn default() -> Self {
        Self::Unknown
    }
}

