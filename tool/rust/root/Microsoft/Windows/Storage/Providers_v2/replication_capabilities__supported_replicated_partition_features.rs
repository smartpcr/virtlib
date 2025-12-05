// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ReplicationCapabilities_SupportedReplicatedPartitionFeatures
//////////////////////////////////////////////

/// ReplicationCapabilities_SupportedReplicatedPartitionFeatures enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ReplicationCapabilities_SupportedReplicatedPartitionFeatures {
    /// Supports_GUID_Partition_Table
    #[serde(rename = "Supports_GUID_Partition_Table")]
    SupportsGUIDPartitionTable = 2,
    /// Supports_MBR_Partition
    #[serde(rename = "Supports_MBR_Partition")]
    SupportsMBRPartition = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 5,
}

impl Default for ReplicationCapabilities_SupportedReplicatedPartitionFeatures {
    fn default() -> Self {
        Self::SupportsGUIDPartitionTable
    }
}

