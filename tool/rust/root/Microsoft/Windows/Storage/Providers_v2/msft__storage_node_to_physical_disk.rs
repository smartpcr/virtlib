// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNodeToPhysicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNodeToPhysicalDisk {

/// The operating system's number for the disk on this StorageNode. Disk 0 is typically the boot device. Disk numbers may not necessarily remain the same across reboot, and are not necessarily the same on different nodes.
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// Denotes the health status of the PhysicalDisk on this StorageNode.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<StorageNodeToPhysicalDisk_HealthStatus>,

/// Indicates whether the physical disk uses MPIO.
    #[serde(rename = "IsMpioEnabled")]
    pub is_mpio_enabled: Option<bool>,

/// Indicates whether the physical disk is physically connected to this storage node.
    #[serde(rename = "IsPhysicallyConnected")]
    pub is_physically_connected: Option<bool>,

/// The MPIO load balance policy being used by the disk.
    #[serde(rename = "LoadBalancePolicy")]
    pub load_balance_policy: Option<StorageNodeToPhysicalDisk_LoadBalancePolicy>,

/// Denotes the operational status of the PhysicalDisk.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<StorageNodeToPhysicalDisk_OperationalStatus>,

/// Collection of MPIO path IDs, reported by the MPIO DSM, when applicable.
    #[serde(rename = "PathId")]
    pub path_id: Vec<String>,

/// The current state of MPIO paths between the node and physical disk.
    #[serde(rename = "PathState")]
    pub path_state: Vec<StorageNodeToPhysicalDisk_PathState>,

/// 
    #[serde(rename = "PhysicalDisk")]
    pub physical_disk: Option<MSFT_PhysicalDisk>,

/// 
    #[serde(rename = "StorageNode")]
    pub storage_node: Option<MSFT_StorageNode>,
}

impl MSFT_StorageNodeToPhysicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk_number: None,
            health_status: None,
            is_mpio_enabled: None,
            is_physically_connected: None,
            load_balance_policy: None,
            operational_status: Vec::new(),
            path_id: Vec::new(),
            path_state: Vec::new(),
            physical_disk: None,
            storage_node: None,
        }
    }


    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: StorageNodeToPhysicalDisk_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&StorageNodeToPhysicalDisk_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of IsMpioEnabled
    pub fn set_is_mpio_enabled(&mut self, value: bool) {
        self.is_mpio_enabled = Some(value);
    }

    /// Gets the value of IsMpioEnabled
    pub fn get_is_mpio_enabled(&self) -> Option<&bool> {
        self.is_mpio_enabled.as_ref()
    }

    /// Sets the value of IsPhysicallyConnected
    pub fn set_is_physically_connected(&mut self, value: bool) {
        self.is_physically_connected = Some(value);
    }

    /// Gets the value of IsPhysicallyConnected
    pub fn get_is_physically_connected(&self) -> Option<&bool> {
        self.is_physically_connected.as_ref()
    }

    /// Sets the value of LoadBalancePolicy
    pub fn set_load_balance_policy(&mut self, value: StorageNodeToPhysicalDisk_LoadBalancePolicy) {
        self.load_balance_policy = Some(value);
    }

    /// Gets the value of LoadBalancePolicy
    pub fn get_load_balance_policy(&self) -> Option<&StorageNodeToPhysicalDisk_LoadBalancePolicy> {
        self.load_balance_policy.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<StorageNodeToPhysicalDisk_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<StorageNodeToPhysicalDisk_OperationalStatus> {
        &self.operational_status
    }

    /// Sets the value of PathId
    pub fn set_path_id(&mut self, value: Vec<String>) {
        self.path_id = value;
    }

    /// Gets the value of PathId
    pub fn get_path_id(&self) -> &Vec<String> {
        &self.path_id
    }

    /// Sets the value of PathState
    pub fn set_path_state(&mut self, value: Vec<StorageNodeToPhysicalDisk_PathState>) {
        self.path_state = value;
    }

    /// Gets the value of PathState
    pub fn get_path_state(&self) -> &Vec<StorageNodeToPhysicalDisk_PathState> {
        &self.path_state
    }

    /// Sets the value of PhysicalDisk
    pub fn set_physical_disk(&mut self, value: MSFT_PhysicalDisk) {
        self.physical_disk = Some(value);
    }

    /// Gets the value of PhysicalDisk
    pub fn get_physical_disk(&self) -> Option<&MSFT_PhysicalDisk> {
        self.physical_disk.as_ref()
    }

    /// Sets the value of StorageNode
    pub fn set_storage_node(&mut self, value: MSFT_StorageNode) {
        self.storage_node = Some(value);
    }

    /// Gets the value of StorageNode
    pub fn get_storage_node(&self) -> Option<&MSFT_StorageNode> {
        self.storage_node.as_ref()
    }
}

