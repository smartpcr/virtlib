// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_StorageAllocationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_StorageAllocationSettingData {
    #[serde(flatten)]
    pub base: CIM_StorageAllocationSettingData,

/// 
    #[serde(rename = "CachingMode")]
    pub caching_mode: Option<u16>,

/// 
    #[serde(rename = "IgnoreFlushes")]
    pub ignore_flushes: Option<bool>,

/// 
    #[serde(rename = "IOPSAllocationUnits")]
    pub iopsallocation_units: Option<String>,

/// 
    #[serde(rename = "IOPSLimit")]
    pub iopslimit: Option<u64>,

/// 
    #[serde(rename = "IOPSReservation")]
    pub iopsreservation: Option<u64>,

/// 
    #[serde(rename = "PersistentReservationsSupported")]
    pub persistent_reservations_supported: Option<bool>,

/// 
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,

/// 
    #[serde(rename = "StorageQoSPolicyID")]
    pub storage_qo_spolicy_id: Option<String>,

/// 
    #[serde(rename = "WriteHardeningMethod")]
    pub write_hardening_method: Option<u16>,
}

impl Msvm_StorageAllocationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageAllocationSettingData::new(),
            caching_mode: None,
            ignore_flushes: None,
            iopsallocation_units: None,
            iopslimit: None,
            iopsreservation: None,
            persistent_reservations_supported: None,
            snapshot_id: None,
            storage_qo_spolicy_id: None,
            write_hardening_method: None,
        }
    }


    /// Sets the value of CachingMode
    pub fn set_caching_mode(&mut self, value: u16) {
        self.caching_mode = Some(value);
    }

    /// Gets the value of CachingMode
    pub fn get_caching_mode(&self) -> Option<&u16> {
        self.caching_mode.as_ref()
    }

    /// Sets the value of IgnoreFlushes
    pub fn set_ignore_flushes(&mut self, value: bool) {
        self.ignore_flushes = Some(value);
    }

    /// Gets the value of IgnoreFlushes
    pub fn get_ignore_flushes(&self) -> Option<&bool> {
        self.ignore_flushes.as_ref()
    }

    /// Sets the value of IOPSAllocationUnits
    pub fn set_iopsallocation_units(&mut self, value: String) {
        self.iopsallocation_units = Some(value);
    }

    /// Gets the value of IOPSAllocationUnits
    pub fn get_iopsallocation_units(&self) -> Option<&String> {
        self.iopsallocation_units.as_ref()
    }

    /// Sets the value of IOPSLimit
    pub fn set_iopslimit(&mut self, value: u64) {
        self.iopslimit = Some(value);
    }

    /// Gets the value of IOPSLimit
    pub fn get_iopslimit(&self) -> Option<&u64> {
        self.iopslimit.as_ref()
    }

    /// Sets the value of IOPSReservation
    pub fn set_iopsreservation(&mut self, value: u64) {
        self.iopsreservation = Some(value);
    }

    /// Gets the value of IOPSReservation
    pub fn get_iopsreservation(&self) -> Option<&u64> {
        self.iopsreservation.as_ref()
    }

    /// Sets the value of PersistentReservationsSupported
    pub fn set_persistent_reservations_supported(&mut self, value: bool) {
        self.persistent_reservations_supported = Some(value);
    }

    /// Gets the value of PersistentReservationsSupported
    pub fn get_persistent_reservations_supported(&self) -> Option<&bool> {
        self.persistent_reservations_supported.as_ref()
    }

    /// Sets the value of SnapshotId
    pub fn set_snapshot_id(&mut self, value: String) {
        self.snapshot_id = Some(value);
    }

    /// Gets the value of SnapshotId
    pub fn get_snapshot_id(&self) -> Option<&String> {
        self.snapshot_id.as_ref()
    }

    /// Sets the value of StorageQoSPolicyID
    pub fn set_storage_qo_spolicy_id(&mut self, value: String) {
        self.storage_qo_spolicy_id = Some(value);
    }

    /// Gets the value of StorageQoSPolicyID
    pub fn get_storage_qo_spolicy_id(&self) -> Option<&String> {
        self.storage_qo_spolicy_id.as_ref()
    }

    /// Sets the value of WriteHardeningMethod
    pub fn set_write_hardening_method(&mut self, value: u16) {
        self.write_hardening_method = Some(value);
    }

    /// Gets the value of WriteHardeningMethod
    pub fn get_write_hardening_method(&self) -> Option<&u16> {
        self.write_hardening_method.as_ref()
    }
}

impl Msvm_StorageAllocationSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

