// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ReplicationCapabilities_SupportedLogVolumeFeatures
//////////////////////////////////////////////

/// ReplicationCapabilities_SupportedLogVolumeFeatures enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ReplicationCapabilities_SupportedLogVolumeFeatures {
    /// Supports_GUID_Partition_Table
    #[serde(rename = "Supports_GUID_Partition_Table")]
    SupportsGUIDPartitionTable = 2,
    /// Supports_MBR_Partition
    #[serde(rename = "Supports_MBR_Partition")]
    SupportsMBRPartition = 3,
    /// Supports_NTFS_File_System
    #[serde(rename = "Supports_NTFS_File_System")]
    SupportsNTFSFileSystem = 4,
    /// Supports_ReFS_File_System
    #[serde(rename = "Supports_ReFS_File_System")]
    SupportsReFSFileSystem = 5,
    /// Supports_Cluster_Shared_Volume
    #[serde(rename = "Supports_Cluster_Shared_Volume")]
    SupportsClusterSharedVolume = 6,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 7,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 8,
}

impl Default for ReplicationCapabilities_SupportedLogVolumeFeatures {
    fn default() -> Self {
        Self::SupportsGUIDPartitionTable
    }
}

