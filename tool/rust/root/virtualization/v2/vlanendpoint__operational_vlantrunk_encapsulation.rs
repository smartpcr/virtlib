// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VLANEndpoint_OperationalVLANTrunkEncapsulation
//////////////////////////////////////////////

/// VLANEndpoint_OperationalVLANTrunkEncapsulation enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VLANEndpoint_OperationalVLANTrunkEncapsulation {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
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
    /// Negotiating
    #[serde(rename = "Negotiating")]
    Negotiating = 5,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 6,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 7,
}

impl Default for VLANEndpoint_OperationalVLANTrunkEncapsulation {
    fn default() -> Self {
        Self::Unknown
    }
}

