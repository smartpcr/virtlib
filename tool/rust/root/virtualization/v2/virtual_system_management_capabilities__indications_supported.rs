// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemManagementCapabilities_IndicationsSupported
//////////////////////////////////////////////

/// VirtualSystemManagementCapabilities_IndicationsSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemManagementCapabilities_IndicationsSupported {
    /// VirtualResourceStateChangeIndicationsSupported
    #[serde(rename = "VirtualResourceStateChangeIndicationsSupported")]
    VirtualResourceStateChangeIndicationsSupported = 2,
    /// ConcreteJobStateChangeIndicationsSupported
    #[serde(rename = "ConcreteJobStateChangeIndicationsSupported")]
    ConcreteJobStateChangeIndicationsSupported = 3,
    /// VirtualSystemStateChangeIndicationsSupported
    #[serde(rename = "VirtualSystemStateChangeIndicationsSupported")]
    VirtualSystemStateChangeIndicationsSupported = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 6,
}

impl Default for VirtualSystemManagementCapabilities_IndicationsSupported {
    fn default() -> Self {
        Self::VirtualResourceStateChangeIndicationsSupported
    }
}

