// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ResourcePoolConfigurationCapabilities_SynchronousMethodsSupported
//////////////////////////////////////////////

/// ResourcePoolConfigurationCapabilities_SynchronousMethodsSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ResourcePoolConfigurationCapabilities_SynchronousMethodsSupported {
    /// CreateResourcePool_is_supported
    #[serde(rename = "CreateResourcePool_is_supported")]
    CreateResourcePoolIsSupported = 2,
    /// CreateChild_ResourcePool_is_supported
    #[serde(rename = "CreateChild_ResourcePool_is_supported")]
    CreateChildResourcePoolIsSupported = 3,
    /// DeleteResourcePool_is_supported
    #[serde(rename = "DeleteResourcePool_is_supported")]
    DeleteResourcePoolIsSupported = 4,
    /// AddResourcesToResourcePool_is_supported
    #[serde(rename = "AddResourcesToResourcePool_is_supported")]
    AddResourcesToResourcePoolIsSupported = 5,
    /// RemoveResourcesFromResourcePool_is_supported
    #[serde(rename = "RemoveResourcesFromResourcePool_is_supported")]
    RemoveResourcesFromResourcePoolIsSupported = 6,
    /// CIM_ChangeParentResourcePool_is_supported
    #[serde(rename = "CIM_ChangeParentResourcePool_is_supported")]
    CIMChangeParentResourcePoolIsSupported = 7,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 8,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 9,
}

impl Default for ResourcePoolConfigurationCapabilities_SynchronousMethodsSupported {
    fn default() -> Self {
        Self::CreateResourcePoolIsSupported
    }
}

