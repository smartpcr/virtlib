// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ProtocolControllerForUnit_DeviceAccess
//////////////////////////////////////////////

/// ProtocolControllerForUnit_DeviceAccess enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ProtocolControllerForUnit_DeviceAccess {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Read_Write
    #[serde(rename = "Read_Write")]
    ReadWrite = 2,
    /// Read_Only
    #[serde(rename = "Read_Only")]
    ReadOnly = 3,
    /// No_Access
    #[serde(rename = "No_Access")]
    NoAccess = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for ProtocolControllerForUnit_DeviceAccess {
    fn default() -> Self {
        Self::Unknown
    }
}

