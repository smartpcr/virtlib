// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SecurityElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SecurityElement {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// CreationClassName indicates the name of the class or the subclass used in the creation of an instance. When used with the other key properties of this class, this property allows all instances of this class and its subclasses to be uniquely identified.
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "EncryptStateAndVmMigrationTrafficEnabled")]
    pub encrypt_state_and_vm_migration_traffic_enabled: Option<bool>,

/// 
    #[serde(rename = "Shielded")]
    pub shielded: Option<bool>,

/// The scoping System's CreationClassName.
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// The scoping System's Name.
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl Msvm_SecurityElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            creation_class_name: None,
            encrypt_state_and_vm_migration_traffic_enabled: None,
            shielded: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of EncryptStateAndVmMigrationTrafficEnabled
    pub fn set_encrypt_state_and_vm_migration_traffic_enabled(&mut self, value: bool) {
        self.encrypt_state_and_vm_migration_traffic_enabled = Some(value);
    }

    /// Gets the value of EncryptStateAndVmMigrationTrafficEnabled
    pub fn get_encrypt_state_and_vm_migration_traffic_enabled(&self) -> Option<&bool> {
        self.encrypt_state_and_vm_migration_traffic_enabled.as_ref()
    }

    /// Sets the value of Shielded
    pub fn set_shielded(&mut self, value: bool) {
        self.shielded = Some(value);
    }

    /// Gets the value of Shielded
    pub fn get_shielded(&self) -> Option<&bool> {
        self.shielded.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }
}

