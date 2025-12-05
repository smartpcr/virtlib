// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageExtent_NameNamespace
//////////////////////////////////////////////

/// StorageExtent_NameNamespace enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageExtent_NameNamespace {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// VPD83Type3
    #[serde(rename = "VPD83Type3")]
    VPD83Type3 = 2,
    /// VPD83Type2
    #[serde(rename = "VPD83Type2")]
    VPD83Type2 = 3,
    /// VPD83Type1
    #[serde(rename = "VPD83Type1")]
    VPD83Type1 = 4,
    /// VPD80
    #[serde(rename = "VPD80")]
    VPD80 = 5,
    /// NodeWWN
    #[serde(rename = "NodeWWN")]
    NodeWWN = 6,
    /// SNVM
    #[serde(rename = "SNVM")]
    SNVM = 7,
    /// OS_Device_Namespace
    #[serde(rename = "OS_Device_Namespace")]
    OSDeviceNamespace = 8,
}

impl Default for StorageExtent_NameNamespace {
    fn default() -> Self {
        Self::Unknown
    }
}

