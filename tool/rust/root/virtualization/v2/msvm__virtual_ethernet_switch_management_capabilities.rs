// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualEthernetSwitchManagementCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualEthernetSwitchManagementCapabilities {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemManagementCapabilities,

/// A boolean value which indicates whether IOV is supported by the platform.If the value is TRUE, then IOV is supported by the platform and IOVSupportReasons will be empty. Otherwise the IOVSupportReasons property will have the reasons why IOV cannot be supported.
    #[serde(rename = "IOVSupport")]
    pub iovsupport: Option<bool>,

/// An array of strings that indicates the possible reasons why IOV is not supported. If the value of IOVSupport is TRUE this array will be empty. 
    #[serde(rename = "IOVSupportReasons")]
    pub iovsupport_reasons: Vec<String>,
}

impl Msvm_VirtualEthernetSwitchManagementCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemManagementCapabilities::new(),
            iovsupport: None,
            iovsupport_reasons: Vec::new(),
        }
    }


    /// Sets the value of IOVSupport
    pub fn set_iovsupport(&mut self, value: bool) {
        self.iovsupport = Some(value);
    }

    /// Gets the value of IOVSupport
    pub fn get_iovsupport(&self) -> Option<&bool> {
        self.iovsupport.as_ref()
    }

    /// Sets the value of IOVSupportReasons
    pub fn set_iovsupport_reasons(&mut self, value: Vec<String>) {
        self.iovsupport_reasons = value;
    }

    /// Gets the value of IOVSupportReasons
    pub fn get_iovsupport_reasons(&self) -> &Vec<String> {
        &self.iovsupport_reasons
    }
}

impl Msvm_VirtualEthernetSwitchManagementCapabilities {
    /// Gets the related Msvm_VirtualEthernetSwitchManagementService object(s)
    pub fn get_related__virtual_ethernet_switch_management_service(&self) -> Result<Msvm_VirtualEthernetSwitchManagementService, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitchManagementService")
    }

}

