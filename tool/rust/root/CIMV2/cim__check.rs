// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Check struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Check {

/// 
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "CheckID")]
    pub check_id: Option<String>,

/// 
    #[serde(rename = "CheckMode")]
    pub check_mode: Option<bool>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "SoftwareElementID")]
    pub software_element_id: Option<String>,

/// 
    #[serde(rename = "SoftwareElementState")]
    pub software_element_state: Option<u16>,

/// 
    #[serde(rename = "TargetOperatingSystem")]
    pub target_operating_system: Option<u16>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl CIM_Check {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            check_id: None,
            check_mode: None,
            description: None,
            name: None,
            software_element_id: None,
            software_element_state: None,
            target_operating_system: None,
            version: None,
        }
    }


    /// Sets the value of Caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of Caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of CheckID
    pub fn set_check_id(&mut self, value: String) {
        self.check_id = Some(value);
    }

    /// Gets the value of CheckID
    pub fn get_check_id(&self) -> Option<&String> {
        self.check_id.as_ref()
    }

    /// Sets the value of CheckMode
    pub fn set_check_mode(&mut self, value: bool) {
        self.check_mode = Some(value);
    }

    /// Gets the value of CheckMode
    pub fn get_check_mode(&self) -> Option<&bool> {
        self.check_mode.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of SoftwareElementID
    pub fn set_software_element_id(&mut self, value: String) {
        self.software_element_id = Some(value);
    }

    /// Gets the value of SoftwareElementID
    pub fn get_software_element_id(&self) -> Option<&String> {
        self.software_element_id.as_ref()
    }

    /// Sets the value of SoftwareElementState
    pub fn set_software_element_state(&mut self, value: u16) {
        self.software_element_state = Some(value);
    }

    /// Gets the value of SoftwareElementState
    pub fn get_software_element_state(&self) -> Option<&u16> {
        self.software_element_state.as_ref()
    }

    /// Sets the value of TargetOperatingSystem
    pub fn set_target_operating_system(&mut self, value: u16) {
        self.target_operating_system = Some(value);
    }

    /// Gets the value of TargetOperatingSystem
    pub fn get_target_operating_system(&self) -> Option<&u16> {
        self.target_operating_system.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn invoke(&self) -> Result<(), WmiError> {
        self.invoke_method("Invoke", &[])

    }

}

