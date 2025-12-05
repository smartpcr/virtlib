// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServerComponent_HyperV struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerComponent_HyperV {
    #[serde(flatten)]
    pub base: MSFT_ServerManagerServerComponentDescriptor,

/// 
    #[serde(rename = "DefaultVirtualHardDiskPath")]
    pub default_virtual_hard_disk_path: Option<String>,

/// 
    #[serde(rename = "DefaultVirtualMachinePath")]
    pub default_virtual_machine_path: Option<String>,

/// 
    #[serde(rename = "EnableVirtualMachineMigration")]
    pub enable_virtual_machine_migration: Option<bool>,

/// 
    #[serde(rename = "VirtualMachineMigrationAuthenticationType")]
    pub virtual_machine_migration_authentication_type: Option<String>,

/// 
    #[serde(rename = "VirtualSwitchNetworkAdapters")]
    pub virtual_switch_network_adapters: Vec<String>,
}

impl ServerComponent_HyperV {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_ServerManagerServerComponentDescriptor::new(),
            default_virtual_hard_disk_path: None,
            default_virtual_machine_path: None,
            enable_virtual_machine_migration: None,
            virtual_machine_migration_authentication_type: None,
            virtual_switch_network_adapters: Vec::new(),
        }
    }


    /// Sets the value of DefaultVirtualHardDiskPath
    pub fn set_default_virtual_hard_disk_path(&mut self, value: String) {
        self.default_virtual_hard_disk_path = Some(value);
    }

    /// Gets the value of DefaultVirtualHardDiskPath
    pub fn get_default_virtual_hard_disk_path(&self) -> Option<&String> {
        self.default_virtual_hard_disk_path.as_ref()
    }

    /// Sets the value of DefaultVirtualMachinePath
    pub fn set_default_virtual_machine_path(&mut self, value: String) {
        self.default_virtual_machine_path = Some(value);
    }

    /// Gets the value of DefaultVirtualMachinePath
    pub fn get_default_virtual_machine_path(&self) -> Option<&String> {
        self.default_virtual_machine_path.as_ref()
    }

    /// Sets the value of EnableVirtualMachineMigration
    pub fn set_enable_virtual_machine_migration(&mut self, value: bool) {
        self.enable_virtual_machine_migration = Some(value);
    }

    /// Gets the value of EnableVirtualMachineMigration
    pub fn get_enable_virtual_machine_migration(&self) -> Option<&bool> {
        self.enable_virtual_machine_migration.as_ref()
    }

    /// Sets the value of VirtualMachineMigrationAuthenticationType
    pub fn set_virtual_machine_migration_authentication_type(&mut self, value: String) {
        self.virtual_machine_migration_authentication_type = Some(value);
    }

    /// Gets the value of VirtualMachineMigrationAuthenticationType
    pub fn get_virtual_machine_migration_authentication_type(&self) -> Option<&String> {
        self.virtual_machine_migration_authentication_type.as_ref()
    }

    /// Sets the value of VirtualSwitchNetworkAdapters
    pub fn set_virtual_switch_network_adapters(&mut self, value: Vec<String>) {
        self.virtual_switch_network_adapters = value;
    }

    /// Gets the value of VirtualSwitchNetworkAdapters
    pub fn get_virtual_switch_network_adapters(&self) -> &Vec<String> {
        &self.virtual_switch_network_adapters
    }
}

