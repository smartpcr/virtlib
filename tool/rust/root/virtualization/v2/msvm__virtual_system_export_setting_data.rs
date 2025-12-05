// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemExportSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemExportSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BackupIntent")]
    pub backup_intent: Option<u8>,

/// 
    #[serde(rename = "CaptureLiveState")]
    pub capture_live_state: Option<u8>,

/// 
    #[serde(rename = "CopySnapshotConfiguration")]
    pub copy_snapshot_configuration: Option<u8>,

/// 
    #[serde(rename = "CopyVmRuntimeInformation")]
    pub copy_vm_runtime_information: Option<bool>,

/// 
    #[serde(rename = "CopyVmStorage")]
    pub copy_vm_storage: Option<bool>,

/// 
    #[serde(rename = "CreateVmExportSubdirectory")]
    pub create_vm_export_subdirectory: Option<bool>,

/// 
    #[serde(rename = "DifferentialBackupBase")]
    pub differential_backup_base: Option<String>,

/// 
    #[serde(rename = "DisableDifferentialOfIgnoredStorage")]
    pub disable_differential_of_ignored_storage: Option<bool>,

/// 
    #[serde(rename = "ExcludedVirtualHardDisks")]
    pub excluded_virtual_hard_disks: Vec<String>,

/// 
    #[serde(rename = "ExportForLiveMigration")]
    pub export_for_live_migration: Option<bool>,

/// 
    #[serde(rename = "GuestDebugStateFilePath")]
    pub guest_debug_state_file_path: Option<String>,

/// 
    #[serde(rename = "SnapshotVirtualSystem")]
    pub snapshot_virtual_system: Option<String>,
}

impl Msvm_VirtualSystemExportSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            backup_intent: None,
            capture_live_state: None,
            copy_snapshot_configuration: None,
            copy_vm_runtime_information: None,
            copy_vm_storage: None,
            create_vm_export_subdirectory: None,
            differential_backup_base: None,
            disable_differential_of_ignored_storage: None,
            excluded_virtual_hard_disks: Vec::new(),
            export_for_live_migration: None,
            guest_debug_state_file_path: None,
            snapshot_virtual_system: None,
        }
    }


    /// Sets the value of BackupIntent
    pub fn set_backup_intent(&mut self, value: u8) {
        self.backup_intent = Some(value);
    }

    /// Gets the value of BackupIntent
    pub fn get_backup_intent(&self) -> Option<&u8> {
        self.backup_intent.as_ref()
    }

    /// Sets the value of CaptureLiveState
    pub fn set_capture_live_state(&mut self, value: u8) {
        self.capture_live_state = Some(value);
    }

    /// Gets the value of CaptureLiveState
    pub fn get_capture_live_state(&self) -> Option<&u8> {
        self.capture_live_state.as_ref()
    }

    /// Sets the value of CopySnapshotConfiguration
    pub fn set_copy_snapshot_configuration(&mut self, value: u8) {
        self.copy_snapshot_configuration = Some(value);
    }

    /// Gets the value of CopySnapshotConfiguration
    pub fn get_copy_snapshot_configuration(&self) -> Option<&u8> {
        self.copy_snapshot_configuration.as_ref()
    }

    /// Sets the value of CopyVmRuntimeInformation
    pub fn set_copy_vm_runtime_information(&mut self, value: bool) {
        self.copy_vm_runtime_information = Some(value);
    }

    /// Gets the value of CopyVmRuntimeInformation
    pub fn get_copy_vm_runtime_information(&self) -> Option<&bool> {
        self.copy_vm_runtime_information.as_ref()
    }

    /// Sets the value of CopyVmStorage
    pub fn set_copy_vm_storage(&mut self, value: bool) {
        self.copy_vm_storage = Some(value);
    }

    /// Gets the value of CopyVmStorage
    pub fn get_copy_vm_storage(&self) -> Option<&bool> {
        self.copy_vm_storage.as_ref()
    }

    /// Sets the value of CreateVmExportSubdirectory
    pub fn set_create_vm_export_subdirectory(&mut self, value: bool) {
        self.create_vm_export_subdirectory = Some(value);
    }

    /// Gets the value of CreateVmExportSubdirectory
    pub fn get_create_vm_export_subdirectory(&self) -> Option<&bool> {
        self.create_vm_export_subdirectory.as_ref()
    }

    /// Sets the value of DifferentialBackupBase
    pub fn set_differential_backup_base(&mut self, value: String) {
        self.differential_backup_base = Some(value);
    }

    /// Gets the value of DifferentialBackupBase
    pub fn get_differential_backup_base(&self) -> Option<&String> {
        self.differential_backup_base.as_ref()
    }

    /// Sets the value of DisableDifferentialOfIgnoredStorage
    pub fn set_disable_differential_of_ignored_storage(&mut self, value: bool) {
        self.disable_differential_of_ignored_storage = Some(value);
    }

    /// Gets the value of DisableDifferentialOfIgnoredStorage
    pub fn get_disable_differential_of_ignored_storage(&self) -> Option<&bool> {
        self.disable_differential_of_ignored_storage.as_ref()
    }

    /// Sets the value of ExcludedVirtualHardDisks
    pub fn set_excluded_virtual_hard_disks(&mut self, value: Vec<String>) {
        self.excluded_virtual_hard_disks = value;
    }

    /// Gets the value of ExcludedVirtualHardDisks
    pub fn get_excluded_virtual_hard_disks(&self) -> &Vec<String> {
        &self.excluded_virtual_hard_disks
    }

    /// Sets the value of ExportForLiveMigration
    pub fn set_export_for_live_migration(&mut self, value: bool) {
        self.export_for_live_migration = Some(value);
    }

    /// Gets the value of ExportForLiveMigration
    pub fn get_export_for_live_migration(&self) -> Option<&bool> {
        self.export_for_live_migration.as_ref()
    }

    /// Sets the value of GuestDebugStateFilePath
    pub fn set_guest_debug_state_file_path(&mut self, value: String) {
        self.guest_debug_state_file_path = Some(value);
    }

    /// Gets the value of GuestDebugStateFilePath
    pub fn get_guest_debug_state_file_path(&self) -> Option<&String> {
        self.guest_debug_state_file_path.as_ref()
    }

    /// Sets the value of SnapshotVirtualSystem
    pub fn set_snapshot_virtual_system(&mut self, value: String) {
        self.snapshot_virtual_system = Some(value);
    }

    /// Gets the value of SnapshotVirtualSystem
    pub fn get_snapshot_virtual_system(&self) -> Option<&String> {
        self.snapshot_virtual_system.as_ref()
    }
}

impl Msvm_VirtualSystemExportSettingData {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

