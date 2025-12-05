// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNodeToDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNodeToDisk {

/// 
    #[serde(rename = "Disk")]
    pub disk: Option<MSFT_Disk>,

/// 
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "IsOffline")]
    pub is_offline: Option<bool>,

/// 
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// 
    #[serde(rename = "OfflineReason")]
    pub offline_reason: Option<u16>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "StorageNode")]
    pub storage_node: Option<MSFT_StorageNode>,
}

impl MSFT_StorageNodeToDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk: None,
            disk_number: None,
            health_status: None,
            is_offline: None,
            is_read_only: None,
            offline_reason: None,
            operational_status: Vec::new(),
            storage_node: None,
        }
    }


    /// Sets the value of Disk
    pub fn set_disk(&mut self, value: MSFT_Disk) {
        self.disk = Some(value);
    }

    /// Gets the value of Disk
    pub fn get_disk(&self) -> Option<&MSFT_Disk> {
        self.disk.as_ref()
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
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of IsOffline
    pub fn set_is_offline(&mut self, value: bool) {
        self.is_offline = Some(value);
    }

    /// Gets the value of IsOffline
    pub fn get_is_offline(&self) -> Option<&bool> {
        self.is_offline.as_ref()
    }

    /// Sets the value of IsReadOnly
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = Some(value);
    }

    /// Gets the value of IsReadOnly
    pub fn get_is_read_only(&self) -> Option<&bool> {
        self.is_read_only.as_ref()
    }

    /// Sets the value of OfflineReason
    pub fn set_offline_reason(&mut self, value: u16) {
        self.offline_reason = Some(value);
    }

    /// Gets the value of OfflineReason
    pub fn get_offline_reason(&self) -> Option<&u16> {
        self.offline_reason.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
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

