// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionSnapshotExportSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionSnapshotExportSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BackupIntent")]
    pub backup_intent: Option<u16>,

/// 
    #[serde(rename = "CopyVmStorage")]
    pub copy_vm_storage: Option<bool>,

/// 
    #[serde(rename = "DifferentialBackupBase")]
    pub differential_backup_base: Option<String>,
}

impl Msvm_CollectionSnapshotExportSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            backup_intent: None,
            copy_vm_storage: None,
            differential_backup_base: None,
        }
    }


    /// Sets the value of BackupIntent
    pub fn set_backup_intent(&mut self, value: u16) {
        self.backup_intent = Some(value);
    }

    /// Gets the value of BackupIntent
    pub fn get_backup_intent(&self) -> Option<&u16> {
        self.backup_intent.as_ref()
    }

    /// Sets the value of CopyVmStorage
    pub fn set_copy_vm_storage(&mut self, value: bool) {
        self.copy_vm_storage = Some(value);
    }

    /// Gets the value of CopyVmStorage
    pub fn get_copy_vm_storage(&self) -> Option<&bool> {
        self.copy_vm_storage.as_ref()
    }

    /// Sets the value of DifferentialBackupBase
    pub fn set_differential_backup_base(&mut self, value: String) {
        self.differential_backup_base = Some(value);
    }

    /// Gets the value of DifferentialBackupBase
    pub fn get_differential_backup_base(&self) -> Option<&String> {
        self.differential_backup_base.as_ref()
    }
}

