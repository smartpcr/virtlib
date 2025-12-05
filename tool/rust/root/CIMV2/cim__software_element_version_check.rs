// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SoftwareElementVersionCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SoftwareElementVersionCheck {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "LowerSoftwareElementVersion")]
    pub lower_software_element_version: Option<String>,

/// 
    #[serde(rename = "SoftwareElementName")]
    pub software_element_name: Option<String>,

/// 
    #[serde(rename = "SoftwareElementStateDesired")]
    pub software_element_state_desired: Option<u16>,

/// 
    #[serde(rename = "TargetOperatingSystemDesired")]
    pub target_operating_system_desired: Option<u16>,

/// 
    #[serde(rename = "UpperSoftwareElementVersion")]
    pub upper_software_element_version: Option<String>,
}

impl CIM_SoftwareElementVersionCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            lower_software_element_version: None,
            software_element_name: None,
            software_element_state_desired: None,
            target_operating_system_desired: None,
            upper_software_element_version: None,
        }
    }


    /// Sets the value of LowerSoftwareElementVersion
    pub fn set_lower_software_element_version(&mut self, value: String) {
        self.lower_software_element_version = Some(value);
    }

    /// Gets the value of LowerSoftwareElementVersion
    pub fn get_lower_software_element_version(&self) -> Option<&String> {
        self.lower_software_element_version.as_ref()
    }

    /// Sets the value of SoftwareElementName
    pub fn set_software_element_name(&mut self, value: String) {
        self.software_element_name = Some(value);
    }

    /// Gets the value of SoftwareElementName
    pub fn get_software_element_name(&self) -> Option<&String> {
        self.software_element_name.as_ref()
    }

    /// Sets the value of SoftwareElementStateDesired
    pub fn set_software_element_state_desired(&mut self, value: u16) {
        self.software_element_state_desired = Some(value);
    }

    /// Gets the value of SoftwareElementStateDesired
    pub fn get_software_element_state_desired(&self) -> Option<&u16> {
        self.software_element_state_desired.as_ref()
    }

    /// Sets the value of TargetOperatingSystemDesired
    pub fn set_target_operating_system_desired(&mut self, value: u16) {
        self.target_operating_system_desired = Some(value);
    }

    /// Gets the value of TargetOperatingSystemDesired
    pub fn get_target_operating_system_desired(&self) -> Option<&u16> {
        self.target_operating_system_desired.as_ref()
    }

    /// Sets the value of UpperSoftwareElementVersion
    pub fn set_upper_software_element_version(&mut self, value: String) {
        self.upper_software_element_version = Some(value);
    }

    /// Gets the value of UpperSoftwareElementVersion
    pub fn get_upper_software_element_version(&self) -> Option<&String> {
        self.upper_software_element_version.as_ref()
    }
}

