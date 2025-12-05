// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_NodeSupportedVersion struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_NodeSupportedVersion {

/// 
    #[serde(rename = "ClusterFunctionalLevel")]
    pub cluster_functional_level: Option<u32>,

/// 
    #[serde(rename = "ClusterUpgradeVersion")]
    pub cluster_upgrade_version: Option<u32>,
}

impl MSCluster_NodeSupportedVersion {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_functional_level: None,
            cluster_upgrade_version: None,
        }
    }


    /// Sets the value of ClusterFunctionalLevel
    pub fn set_cluster_functional_level(&mut self, value: u32) {
        self.cluster_functional_level = Some(value);
    }

    /// Gets the value of ClusterFunctionalLevel
    pub fn get_cluster_functional_level(&self) -> Option<&u32> {
        self.cluster_functional_level.as_ref()
    }

    /// Sets the value of ClusterUpgradeVersion
    pub fn set_cluster_upgrade_version(&mut self, value: u32) {
        self.cluster_upgrade_version = Some(value);
    }

    /// Gets the value of ClusterUpgradeVersion
    pub fn get_cluster_upgrade_version(&self) -> Option<&u32> {
        self.cluster_upgrade_version.as_ref()
    }
}

