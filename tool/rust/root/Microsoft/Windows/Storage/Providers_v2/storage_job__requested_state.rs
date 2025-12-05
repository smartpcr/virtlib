// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageJob_RequestedState
//////////////////////////////////////////////

/// StorageJob_RequestedState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageJob_RequestedState {
    /// Start
    #[serde(rename = "Start")]
    Start = 2,
    /// Suspend
    #[serde(rename = "Suspend")]
    Suspend = 3,
    /// Terminate
    #[serde(rename = "Terminate")]
    Terminate = 4,
    /// Kill
    #[serde(rename = "Kill")]
    Kill = 5,
    /// Service
    #[serde(rename = "Service")]
    Service = 6,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 7,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 8,
}

impl Default for StorageJob_RequestedState {
    fn default() -> Self {
        Self::Start
    }
}

