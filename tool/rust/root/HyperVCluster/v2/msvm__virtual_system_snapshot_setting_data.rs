// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemSnapshotSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemSnapshotSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "ConsistencyLevel")]
    pub consistency_level: Option<VirtualSystemSnapshotSettingData_ConsistencyLevel>,

/// 
    #[serde(rename = "GuestBackupType")]
    pub guest_backup_type: Option<VirtualSystemSnapshotSettingData_GuestBackupType>,

/// 
    #[serde(rename = "IgnoreNonSnapshottableDisks")]
    pub ignore_non_snapshottable_disks: Option<bool>,
}

impl Msvm_VirtualSystemSnapshotSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            consistency_level: None,
            guest_backup_type: None,
            ignore_non_snapshottable_disks: None,
        }
    }


    /// Sets the value of ConsistencyLevel
    pub fn set_consistency_level(&mut self, value: VirtualSystemSnapshotSettingData_ConsistencyLevel) {
        self.consistency_level = Some(value);
    }

    /// Gets the value of ConsistencyLevel
    pub fn get_consistency_level(&self) -> Option<&VirtualSystemSnapshotSettingData_ConsistencyLevel> {
        self.consistency_level.as_ref()
    }

    /// Sets the value of GuestBackupType
    pub fn set_guest_backup_type(&mut self, value: VirtualSystemSnapshotSettingData_GuestBackupType) {
        self.guest_backup_type = Some(value);
    }

    /// Gets the value of GuestBackupType
    pub fn get_guest_backup_type(&self) -> Option<&VirtualSystemSnapshotSettingData_GuestBackupType> {
        self.guest_backup_type.as_ref()
    }

    /// Sets the value of IgnoreNonSnapshottableDisks
    pub fn set_ignore_non_snapshottable_disks(&mut self, value: bool) {
        self.ignore_non_snapshottable_disks = Some(value);
    }

    /// Gets the value of IgnoreNonSnapshottableDisks
    pub fn get_ignore_non_snapshottable_disks(&self) -> Option<&bool> {
        self.ignore_non_snapshottable_disks.as_ref()
    }
}

