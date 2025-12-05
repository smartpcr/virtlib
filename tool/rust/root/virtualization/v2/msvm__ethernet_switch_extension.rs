// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchExtension struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchExtension {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "ExtensionType")]
    pub extension_type: Option<u8>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,

/// 
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl Msvm_EthernetSwitchExtension {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            creation_class_name: None,
            extension_type: None,
            system_creation_class_name: None,
            system_name: None,
            vendor: None,
            version: None,
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

    /// Sets the value of ExtensionType
    pub fn set_extension_type(&mut self, value: u8) {
        self.extension_type = Some(value);
    }

    /// Gets the value of ExtensionType
    pub fn get_extension_type(&self) -> Option<&u8> {
        self.extension_type.as_ref()
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

    /// Sets the value of Vendor
    pub fn set_vendor(&mut self, value: String) {
        self.vendor = Some(value);
    }

    /// Gets the value of Vendor
    pub fn get_vendor(&self) -> Option<&String> {
        self.vendor.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

impl Msvm_EthernetSwitchExtension {
    /// Gets the related Msvm_VirtualEthernetSwitch object(s)
    pub fn get_related__virtual_ethernet_switch(&self) -> Result<Msvm_VirtualEthernetSwitch, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitch")
    }

    /// Gets the related Msvm_InstalledEthernetSwitchExtension object(s)
    pub fn get_related__installed_ethernet_switch_extension(&self) -> Result<Msvm_InstalledEthernetSwitchExtension, WmiError> {
        self.get_related("Msvm_InstalledEthernetSwitchExtension")
    }

}

