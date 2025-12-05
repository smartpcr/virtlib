// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TPM_RequestedTPMState
//////////////////////////////////////////////

/// TPM_RequestedTPMState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TPM_RequestedTPMState {
    /// S1_Enabled_Active_Owned
    #[serde(rename = "S1_Enabled_Active_Owned")]
    S1EnabledActiveOwned = 2,
    /// S2_Disabled_Active_Owned
    #[serde(rename = "S2_Disabled_Active_Owned")]
    S2DisabledActiveOwned = 3,
    /// S3_Enabled_Inactive_Owned
    #[serde(rename = "S3_Enabled_Inactive_Owned")]
    S3EnabledInactiveOwned = 4,
    /// S4_Disabled_Inactive_Owned
    #[serde(rename = "S4_Disabled_Inactive_Owned")]
    S4DisabledInactiveOwned = 5,
    /// S5_Enabled_Active_Unowned
    #[serde(rename = "S5_Enabled_Active_Unowned")]
    S5EnabledActiveUnowned = 6,
    /// S6_Disabled_Active_Unowned
    #[serde(rename = "S6_Disabled_Active_Unowned")]
    S6DisabledActiveUnowned = 7,
    /// S7_Enabled_Inactive_Unowned
    #[serde(rename = "S7_Enabled_Inactive_Unowned")]
    S7EnabledInactiveUnowned = 8,
    /// S8_Disabled_Inactive_Unowned
    #[serde(rename = "S8_Disabled_Inactive_Unowned")]
    S8DisabledInactiveUnowned = 9,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 10,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 11,
}

impl Default for TPM_RequestedTPMState {
    fn default() -> Self {
        Self::S1EnabledActiveOwned
    }
}

