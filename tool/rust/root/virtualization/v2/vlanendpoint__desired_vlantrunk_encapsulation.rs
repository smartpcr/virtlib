// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VLANEndpoint_DesiredVLANTrunkEncapsulation
//////////////////////////////////////////////

/// VLANEndpoint_DesiredVLANTrunkEncapsulation enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VLANEndpoint_DesiredVLANTrunkEncapsulation {
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 2,
    /// _802_1q
    #[serde(rename = "_802_1q")]
    V8021q = 3,
    /// Cisco_ISL
    #[serde(rename = "Cisco_ISL")]
    CiscoISL = 4,
    /// Negotiate
    #[serde(rename = "Negotiate")]
    Negotiate = 5,
    /// DMTF_Reserved1
    #[serde(rename = "DMTF_Reserved1")]
    DMTFReserved1 = 6,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 7,
}

impl Default for VLANEndpoint_DesiredVLANTrunkEncapsulation {
    fn default() -> Self {
        Self::DMTFReserved
    }
}

