// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SyntheticFcPortSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SyntheticFcPortSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "ChapEnabled")]
    pub chap_enabled: Option<bool>,

/// 
    #[serde(rename = "SecondaryWWNN")]
    pub secondary_wwnn: Option<String>,

/// 
    #[serde(rename = "SecondaryWWPN")]
    pub secondary_wwpn: Option<String>,

/// 
    #[serde(rename = "VirtualPortWWNN")]
    pub virtual_port_wwnn: Option<String>,

/// 
    #[serde(rename = "VirtualPortWWPN")]
    pub virtual_port_wwpn: Option<String>,

/// 
    #[serde(rename = "VirtualSystemIdentifiers")]
    pub virtual_system_identifiers: Vec<String>,
}

impl Msvm_SyntheticFcPortSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            chap_enabled: None,
            secondary_wwnn: None,
            secondary_wwpn: None,
            virtual_port_wwnn: None,
            virtual_port_wwpn: None,
            virtual_system_identifiers: Vec::new(),
        }
    }


    /// Sets the value of ChapEnabled
    pub fn set_chap_enabled(&mut self, value: bool) {
        self.chap_enabled = Some(value);
    }

    /// Gets the value of ChapEnabled
    pub fn get_chap_enabled(&self) -> Option<&bool> {
        self.chap_enabled.as_ref()
    }

    /// Sets the value of SecondaryWWNN
    pub fn set_secondary_wwnn(&mut self, value: String) {
        self.secondary_wwnn = Some(value);
    }

    /// Gets the value of SecondaryWWNN
    pub fn get_secondary_wwnn(&self) -> Option<&String> {
        self.secondary_wwnn.as_ref()
    }

    /// Sets the value of SecondaryWWPN
    pub fn set_secondary_wwpn(&mut self, value: String) {
        self.secondary_wwpn = Some(value);
    }

    /// Gets the value of SecondaryWWPN
    pub fn get_secondary_wwpn(&self) -> Option<&String> {
        self.secondary_wwpn.as_ref()
    }

    /// Sets the value of VirtualPortWWNN
    pub fn set_virtual_port_wwnn(&mut self, value: String) {
        self.virtual_port_wwnn = Some(value);
    }

    /// Gets the value of VirtualPortWWNN
    pub fn get_virtual_port_wwnn(&self) -> Option<&String> {
        self.virtual_port_wwnn.as_ref()
    }

    /// Sets the value of VirtualPortWWPN
    pub fn set_virtual_port_wwpn(&mut self, value: String) {
        self.virtual_port_wwpn = Some(value);
    }

    /// Gets the value of VirtualPortWWPN
    pub fn get_virtual_port_wwpn(&self) -> Option<&String> {
        self.virtual_port_wwpn.as_ref()
    }

    /// Sets the value of VirtualSystemIdentifiers
    pub fn set_virtual_system_identifiers(&mut self, value: Vec<String>) {
        self.virtual_system_identifiers = value;
    }

    /// Gets the value of VirtualSystemIdentifiers
    pub fn get_virtual_system_identifiers(&self) -> &Vec<String> {
        &self.virtual_system_identifiers
    }
}

impl Msvm_SyntheticFcPortSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

