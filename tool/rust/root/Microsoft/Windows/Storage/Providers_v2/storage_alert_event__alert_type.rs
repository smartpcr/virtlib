// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageAlertEvent_AlertType
//////////////////////////////////////////////

/// StorageAlertEvent_AlertType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageAlertEvent_AlertType {
    /// Thin_provisioning_threshold_reached
    #[serde(rename = "Thin_provisioning_threshold_reached")]
    ThinProvisioningThresholdReached = 1,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 2,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 3,
}

impl Default for StorageAlertEvent_AlertType {
    fn default() -> Self {
        Self::ThinProvisioningThresholdReached
    }
}

