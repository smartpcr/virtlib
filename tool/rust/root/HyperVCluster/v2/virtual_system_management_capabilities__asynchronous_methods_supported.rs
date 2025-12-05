// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemManagementCapabilities_AsynchronousMethodsSupported
//////////////////////////////////////////////

/// VirtualSystemManagementCapabilities_AsynchronousMethodsSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemManagementCapabilities_AsynchronousMethodsSupported {
    /// DefineSystemSupported
    #[serde(rename = "DefineSystemSupported")]
    DefineSystemSupported = 2,
    /// DestroySystemSupported
    #[serde(rename = "DestroySystemSupported")]
    DestroySystemSupported = 3,
    /// DestroySystemConfigurationSupported
    #[serde(rename = "DestroySystemConfigurationSupported")]
    DestroySystemConfigurationSupported = 4,
    /// ModifyResourceSettingsSupported
    #[serde(rename = "ModifyResourceSettingsSupported")]
    ModifyResourceSettingsSupported = 5,
    /// ModifySystemSettingsSupported
    #[serde(rename = "ModifySystemSettingsSupported")]
    ModifySystemSettingsSupported = 6,
    /// RemoveResourcesSupported
    #[serde(rename = "RemoveResourcesSupported")]
    RemoveResourcesSupported = 7,
    /// SelectSystemConfigurationSupported
    #[serde(rename = "SelectSystemConfigurationSupported")]
    SelectSystemConfigurationSupported = 8,
    /// SnapshotSystemSupported
    #[serde(rename = "SnapshotSystemSupported")]
    SnapshotSystemSupported = 9,
    /// AddResourcesSupported
    #[serde(rename = "AddResourcesSupported")]
    AddResourcesSupported = 10,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 11,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 12,
}

impl Default for VirtualSystemManagementCapabilities_AsynchronousMethodsSupported {
    fn default() -> Self {
        Self::DefineSystemSupported
    }
}

