// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EthernetPortAllocationSettingData_DesiredVLANEndpointMode
//////////////////////////////////////////////

/// EthernetPortAllocationSettingData_DesiredVLANEndpointMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EthernetPortAllocationSettingData_DesiredVLANEndpointMode {
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Access
    #[serde(rename = "Access")]
    Access = 2,
    /// Dynamic_Auto
    #[serde(rename = "Dynamic_Auto")]
    DynamicAuto = 3,
    /// Dynamic_Desirable
    #[serde(rename = "Dynamic_Desirable")]
    DynamicDesirable = 4,
    /// Trunk
    #[serde(rename = "Trunk")]
    Trunk = 5,
    /// Dot1Q_Tunnel
    #[serde(rename = "Dot1Q_Tunnel")]
    Dot1QTunnel = 6,
    /// DMTF_Reserved1
    #[serde(rename = "DMTF_Reserved1")]
    DMTFReserved1 = 7,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 8,
}

impl Default for EthernetPortAllocationSettingData_DesiredVLANEndpointMode {
    fn default() -> Self {
        Self::DMTFReserved
    }
}

