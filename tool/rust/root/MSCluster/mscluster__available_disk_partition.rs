// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_AvailableDiskPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_AvailableDiskPartition {
    #[serde(flatten)]
    pub base: MSCluster_ClusterDiskPartition,
}

impl MSCluster_AvailableDiskPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_ClusterDiskPartition::new(),
        }
    }

}

