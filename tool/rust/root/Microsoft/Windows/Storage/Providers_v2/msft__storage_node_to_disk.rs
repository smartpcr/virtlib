// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNodeToDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNodeToDisk {

/// 
    #[serde(rename = "Disk")]
    pub disk: Option<MSFT_Disk>,

/// The operating system's number for the disk. Disk 0 is typically the boot device. Disk numbers may not necessarily remain the same across reboots.
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// The health status of the Volume.
/// 0 - 'Healthy': The disk is functioning normally.
/// 1 - 'Warning': The disk is still functioning, but has detected errors or issues that require administrator intervention.
/// 2 - 'Unhealthy': The volume is not functioning, due to errors or failures. The volume needs immediate attention from an administrator.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<StorageNodeToDisk_HealthStatus>,

/// 
    #[serde(rename = "IsOffline")]
    pub is_offline: Option<bool>,

/// 
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// If IsOffline is TRUE, this property informs the user of the specific reason for the disk being offline. 
/// 1 - 'Policy': The user requested the disk to be offline. 
/// 2 - 'Redundant Path': The disk is used for multi-path I/O. 
/// 3 - 'Snapshot': The disk is a snapshot disk. 
/// 4 - 'Collision': There was a signature or identifier collision with another disk. 
/// 5 - 'Resource Exhaustion': There were insufficient resources to bring the disk online. 
/// 6 - 'Critical Write Failures': There were critical write failures on the disk. 
/// 7 - 'Data Integrity Scan Required': A data integrity scan is required.
    #[serde(rename = "OfflineReason")]
    pub offline_reason: Option<StorageNodeToDisk_OfflineReason>,

/// An array of values that denote the current operational status of the volume.
/// 0 - 'Unknown': The operational status is unknown.
/// 1 - 'Other': A vendor-specific OperationalStatus has been specified by setting the OtherOperationalStatusDescription property.
/// 2 - 'OK': The disk is responding to commands and is in a normal operating state.
/// 3 - 'Degraded': The disk is responding to commands, but is not running in an optimal operating state.
/// 4 - 'Stressed': The disk is functioning, but needs attention. For example, the disk might be overloaded or overheated.
/// 5 - 'Predictive Failure': The disk is functioning, but a failure is likely to occur in the near future.
/// 6 - 'Error': An error has occurred.
/// 7 - 'Non-Recoverable Error': A non-recoverable error has occurred.
/// 8 - 'Starting': The disk is in the process of starting.
/// 9 - 'Stopping': The disk is in the process of stopping.
/// 10 - 'Stopped': The disk was stopped or shut down in a clean and orderly fashion.
/// 11 - 'In Service': The disk is being configured, maintained, cleaned, or otherwise administered.
/// 12 - 'No Contact': The storage provider has knowledge of the disk, but has never been able to establish communication with it.
/// 13 - 'Lost Communication': The storage provider has knowledge of the disk and has contacted it successfully in the past, but the disk is currently unreachable.
/// 14 - 'Aborted': Similar to Stopped, except that the disk stopped abruptly and may require configuration or maintenance.
/// 15 - 'Dormant': The disk is reachable, but it is inactive.
/// 16 - 'Supporting Entity in Error': This status value does not necessarily indicate trouble with the disk, but it does indicate that another device or connection that the disk depends on may need attention.
/// 17 - 'Completed': The disk has completed an operation. This status value should be combined with OK, Error, or Degraded, depending on the outcome of the operation.
/// 0xD010 - 'Online': In Windows-based storage subsystems, this indicates that the object is online.
/// 0xD011 - 'Not Ready': In Windows-based storage subsystems, this indicates that the object is not ready.
/// 0xD012 - 'No Media': In Windows-based storage subsystems, this indicates that the object has no media present.
/// 0xD013 - 'Offline': In Windows-based storage subsystems, this indicates that the object is offline.
/// 0xD014 - 'Failed': In Windows-based storage subsystems, this indicates that the object is in a failed state.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<StorageNodeToDisk_OperationalStatus>,

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
    pub fn set_health_status(&mut self, value: StorageNodeToDisk_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&StorageNodeToDisk_HealthStatus> {
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
    pub fn set_offline_reason(&mut self, value: StorageNodeToDisk_OfflineReason) {
        self.offline_reason = Some(value);
    }

    /// Gets the value of OfflineReason
    pub fn get_offline_reason(&self) -> Option<&StorageNodeToDisk_OfflineReason> {
        self.offline_reason.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<StorageNodeToDisk_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<StorageNodeToDisk_OperationalStatus> {
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

