// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_MaskingPortsPerView
//////////////////////////////////////////////

/// StorageSubSystem_MaskingPortsPerView enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_MaskingPortsPerView {
    /// One_TargetPort_per_view
    #[serde(rename = "One_TargetPort_per_view")]
    OneTargetPortPerView = 2,
    /// Multiple_target_ports_per_view
    #[serde(rename = "Multiple_target_ports_per_view")]
    MultipleTargetPortsPerView = 3,
    /// All_target_ports_share_the_same_view
    #[serde(rename = "All_target_ports_share_the_same_view")]
    AllTargetPortsShareTheSameView = 4,
}

impl Default for StorageSubSystem_MaskingPortsPerView {
    fn default() -> Self {
        Self::OneTargetPortPerView
    }
}

