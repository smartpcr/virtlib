// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetPortData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetPortData {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "DeviceCreationClassName")]
    pub device_creation_class_name: Option<String>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl Msvm_EthernetPortData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            creation_class_name: None,
            device_creation_class_name: None,
            device_id: None,
            name: None,
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

    /// Sets the value of DeviceCreationClassName
    pub fn set_device_creation_class_name(&mut self, value: String) {
        self.device_creation_class_name = Some(value);
    }

    /// Gets the value of DeviceCreationClassName
    pub fn get_device_creation_class_name(&self) -> Option<&String> {
        self.device_creation_class_name.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
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

impl Msvm_EthernetPortData {
    /// Gets the related Msvm_EthernetSwitchPort object(s)
    pub fn get_related__ethernet_switch_port(&self) -> Result<Msvm_EthernetSwitchPort, WmiError> {
        self.get_related("Msvm_EthernetSwitchPort")
    }

}

