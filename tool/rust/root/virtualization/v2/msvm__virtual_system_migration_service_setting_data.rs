// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemMigrationServiceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemMigrationServiceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: Option<VirtualSystemMigrationServiceSettingData_AuthenticationType>,

/// 
    #[serde(rename = "EnableCompression")]
    pub enable_compression: Option<bool>,

/// 
    #[serde(rename = "EnableSmbTransport")]
    pub enable_smb_transport: Option<bool>,

/// 
    #[serde(rename = "EnableVirtualSystemMigration")]
    pub enable_virtual_system_migration: Option<bool>,

/// 
    #[serde(rename = "MaximumActiveStorageMigration")]
    pub maximum_active_storage_migration: Option<u32>,

/// 
    #[serde(rename = "MaximumActiveVirtualSystemMigration")]
    pub maximum_active_virtual_system_migration: Option<u32>,
}

impl Msvm_VirtualSystemMigrationServiceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            authentication_type: None,
            enable_compression: None,
            enable_smb_transport: None,
            enable_virtual_system_migration: None,
            maximum_active_storage_migration: None,
            maximum_active_virtual_system_migration: None,
        }
    }


    /// Sets the value of AuthenticationType
    pub fn set_authentication_type(&mut self, value: VirtualSystemMigrationServiceSettingData_AuthenticationType) {
        self.authentication_type = Some(value);
    }

    /// Gets the value of AuthenticationType
    pub fn get_authentication_type(&self) -> Option<&VirtualSystemMigrationServiceSettingData_AuthenticationType> {
        self.authentication_type.as_ref()
    }

    /// Sets the value of EnableCompression
    pub fn set_enable_compression(&mut self, value: bool) {
        self.enable_compression = Some(value);
    }

    /// Gets the value of EnableCompression
    pub fn get_enable_compression(&self) -> Option<&bool> {
        self.enable_compression.as_ref()
    }

    /// Sets the value of EnableSmbTransport
    pub fn set_enable_smb_transport(&mut self, value: bool) {
        self.enable_smb_transport = Some(value);
    }

    /// Gets the value of EnableSmbTransport
    pub fn get_enable_smb_transport(&self) -> Option<&bool> {
        self.enable_smb_transport.as_ref()
    }

    /// Sets the value of EnableVirtualSystemMigration
    pub fn set_enable_virtual_system_migration(&mut self, value: bool) {
        self.enable_virtual_system_migration = Some(value);
    }

    /// Gets the value of EnableVirtualSystemMigration
    pub fn get_enable_virtual_system_migration(&self) -> Option<&bool> {
        self.enable_virtual_system_migration.as_ref()
    }

    /// Sets the value of MaximumActiveStorageMigration
    pub fn set_maximum_active_storage_migration(&mut self, value: u32) {
        self.maximum_active_storage_migration = Some(value);
    }

    /// Gets the value of MaximumActiveStorageMigration
    pub fn get_maximum_active_storage_migration(&self) -> Option<&u32> {
        self.maximum_active_storage_migration.as_ref()
    }

    /// Sets the value of MaximumActiveVirtualSystemMigration
    pub fn set_maximum_active_virtual_system_migration(&mut self, value: u32) {
        self.maximum_active_virtual_system_migration = Some(value);
    }

    /// Gets the value of MaximumActiveVirtualSystemMigration
    pub fn get_maximum_active_virtual_system_migration(&self) -> Option<&u32> {
        self.maximum_active_virtual_system_migration.as_ref()
    }
}

impl Msvm_VirtualSystemMigrationServiceSettingData {
    /// Gets the related Msvm_VirtualSystemMigrationNetworkSettingData object(s)
    pub fn get_related__virtual_system_migration_network_setting_data(&self) -> Result<Vec<Msvm_VirtualSystemMigrationNetworkSettingData>, WmiError> {
        self.get_all_related("Msvm_VirtualSystemMigrationNetworkSettingData")
    }

    /// Gets the related Msvm_VirtualSystemMigrationService object(s)
    pub fn get_related__virtual_system_migration_service(&self) -> Result<Msvm_VirtualSystemMigrationService, WmiError> {
        self.get_related("Msvm_VirtualSystemMigrationService")
    }

}

