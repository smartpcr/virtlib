// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ReplicationSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ReplicationSettings {

/// A set of volumes where the replication journal for the ReplicationGroup is hosted.
    #[serde(rename = "LogDevices")]
    pub log_devices: Vec<MSFT_Volume>,

/// Size of replication journal in units of bytes. Size must be in multiples of gigabytes.
    #[serde(rename = "LogSizeInBytes")]
    pub log_size_in_bytes: Option<u64>,

/// Minimum number of synchronous replication partnerships that are in synchronous replication state for I/O to continue on source Replication Group.
    #[serde(rename = "ReplicationQuorum")]
    pub replication_quorum: Option<u16>,

/// Mode describes whether the target elements will be updated synchronously or asynchronously. If NULL, implementation decides the mode.
    #[serde(rename = "SyncMode")]
    pub sync_mode: Option<ReplicationSettings_SyncMode>,

/// TODO
    #[serde(rename = "TargetElementSupplier")]
    pub target_element_supplier: Option<u16>,

/// TODO
    #[serde(rename = "ThinProvisioningPolicy")]
    pub thin_provisioning_policy: Option<u16>,
}

impl MSFT_ReplicationSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            log_devices: Vec::new(),
            log_size_in_bytes: None,
            replication_quorum: None,
            sync_mode: None,
            target_element_supplier: None,
            thin_provisioning_policy: None,
        }
    }


    /// Sets the value of LogDevices
    pub fn set_log_devices(&mut self, value: Vec<MSFT_Volume>) {
        self.log_devices = value;
    }

    /// Gets the value of LogDevices
    pub fn get_log_devices(&self) -> &Vec<MSFT_Volume> {
        &self.log_devices
    }

    /// Sets the value of LogSizeInBytes
    pub fn set_log_size_in_bytes(&mut self, value: u64) {
        self.log_size_in_bytes = Some(value);
    }

    /// Gets the value of LogSizeInBytes
    pub fn get_log_size_in_bytes(&self) -> Option<&u64> {
        self.log_size_in_bytes.as_ref()
    }

    /// Sets the value of ReplicationQuorum
    pub fn set_replication_quorum(&mut self, value: u16) {
        self.replication_quorum = Some(value);
    }

    /// Gets the value of ReplicationQuorum
    pub fn get_replication_quorum(&self) -> Option<&u16> {
        self.replication_quorum.as_ref()
    }

    /// Sets the value of SyncMode
    pub fn set_sync_mode(&mut self, value: ReplicationSettings_SyncMode) {
        self.sync_mode = Some(value);
    }

    /// Gets the value of SyncMode
    pub fn get_sync_mode(&self) -> Option<&ReplicationSettings_SyncMode> {
        self.sync_mode.as_ref()
    }

    /// Sets the value of TargetElementSupplier
    pub fn set_target_element_supplier(&mut self, value: u16) {
        self.target_element_supplier = Some(value);
    }

    /// Gets the value of TargetElementSupplier
    pub fn get_target_element_supplier(&self) -> Option<&u16> {
        self.target_element_supplier.as_ref()
    }

    /// Sets the value of ThinProvisioningPolicy
    pub fn set_thin_provisioning_policy(&mut self, value: u16) {
        self.thin_provisioning_policy = Some(value);
    }

    /// Gets the value of ThinProvisioningPolicy
    pub fn get_thin_provisioning_policy(&self) -> Option<&u16> {
        self.thin_provisioning_policy.as_ref()
    }
}

