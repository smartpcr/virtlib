// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_InstalledEthernetSwitchExtension struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_InstalledEthernetSwitchExtension {
    #[serde(flatten)]
    pub base: CIM_ManagedSystemElement,

/// 
    #[serde(rename = "ExtensionType")]
    pub extension_type: Option<u8>,

/// 
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl Msvm_InstalledEthernetSwitchExtension {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedSystemElement::new(),
            extension_type: None,
            vendor: None,
            version: None,
        }
    }


    /// Sets the value of ExtensionType
    pub fn set_extension_type(&mut self, value: u8) {
        self.extension_type = Some(value);
    }

    /// Gets the value of ExtensionType
    pub fn get_extension_type(&self) -> Option<&u8> {
        self.extension_type.as_ref()
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

impl Msvm_InstalledEthernetSwitchExtension {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_EthernetSwitchExtension object(s)
    pub fn get_related__ethernet_switch_extension(&self) -> Result<Msvm_EthernetSwitchExtension, WmiError> {
        self.get_related("Msvm_EthernetSwitchExtension")
    }

}

