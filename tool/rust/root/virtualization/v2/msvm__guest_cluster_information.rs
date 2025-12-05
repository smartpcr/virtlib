// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_GuestClusterInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_GuestClusterInformation {

/// 
    #[serde(rename = "ClusterId")]
    pub cluster_id: Option<String>,

/// 
    #[serde(rename = "ClusterSize")]
    pub cluster_size: Option<u16>,

/// 
    #[serde(rename = "IsActiveActive")]
    pub is_active_active: Vec<bool>,

/// 
    #[serde(rename = "IsClustered")]
    pub is_clustered: Vec<bool>,

/// 
    #[serde(rename = "IsOnline")]
    pub is_online: Vec<bool>,

/// 
    #[serde(rename = "IsOwned")]
    pub is_owned: Vec<bool>,

/// 
    #[serde(rename = "LastResourceMoveTime")]
    pub last_resource_move_time: Option<u64>,

/// 
    #[serde(rename = "SharedVirtualHardDiskPaths")]
    pub shared_virtual_hard_disk_paths: Vec<String>,

/// 
    #[serde(rename = "SharedVirtualHardDisks")]
    pub shared_virtual_hard_disks: Vec<String>,
}

impl Msvm_GuestClusterInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_id: None,
            cluster_size: None,
            is_active_active: Vec::new(),
            is_clustered: Vec::new(),
            is_online: Vec::new(),
            is_owned: Vec::new(),
            last_resource_move_time: None,
            shared_virtual_hard_disk_paths: Vec::new(),
            shared_virtual_hard_disks: Vec::new(),
        }
    }


    /// Sets the value of ClusterId
    pub fn set_cluster_id(&mut self, value: String) {
        self.cluster_id = Some(value);
    }

    /// Gets the value of ClusterId
    pub fn get_cluster_id(&self) -> Option<&String> {
        self.cluster_id.as_ref()
    }

    /// Sets the value of ClusterSize
    pub fn set_cluster_size(&mut self, value: u16) {
        self.cluster_size = Some(value);
    }

    /// Gets the value of ClusterSize
    pub fn get_cluster_size(&self) -> Option<&u16> {
        self.cluster_size.as_ref()
    }

    /// Sets the value of IsActiveActive
    pub fn set_is_active_active(&mut self, value: Vec<bool>) {
        self.is_active_active = value;
    }

    /// Gets the value of IsActiveActive
    pub fn get_is_active_active(&self) -> &Vec<bool> {
        &self.is_active_active
    }

    /// Sets the value of IsClustered
    pub fn set_is_clustered(&mut self, value: Vec<bool>) {
        self.is_clustered = value;
    }

    /// Gets the value of IsClustered
    pub fn get_is_clustered(&self) -> &Vec<bool> {
        &self.is_clustered
    }

    /// Sets the value of IsOnline
    pub fn set_is_online(&mut self, value: Vec<bool>) {
        self.is_online = value;
    }

    /// Gets the value of IsOnline
    pub fn get_is_online(&self) -> &Vec<bool> {
        &self.is_online
    }

    /// Sets the value of IsOwned
    pub fn set_is_owned(&mut self, value: Vec<bool>) {
        self.is_owned = value;
    }

    /// Gets the value of IsOwned
    pub fn get_is_owned(&self) -> &Vec<bool> {
        &self.is_owned
    }

    /// Sets the value of LastResourceMoveTime
    pub fn set_last_resource_move_time(&mut self, value: u64) {
        self.last_resource_move_time = Some(value);
    }

    /// Gets the value of LastResourceMoveTime
    pub fn get_last_resource_move_time(&self) -> Option<&u64> {
        self.last_resource_move_time.as_ref()
    }

    /// Sets the value of SharedVirtualHardDiskPaths
    pub fn set_shared_virtual_hard_disk_paths(&mut self, value: Vec<String>) {
        self.shared_virtual_hard_disk_paths = value;
    }

    /// Gets the value of SharedVirtualHardDiskPaths
    pub fn get_shared_virtual_hard_disk_paths(&self) -> &Vec<String> {
        &self.shared_virtual_hard_disk_paths
    }

    /// Sets the value of SharedVirtualHardDisks
    pub fn set_shared_virtual_hard_disks(&mut self, value: Vec<String>) {
        self.shared_virtual_hard_disks = value;
    }

    /// Gets the value of SharedVirtualHardDisks
    pub fn get_shared_virtual_hard_disks(&self) -> &Vec<String> {
        &self.shared_virtual_hard_disks
    }
}

