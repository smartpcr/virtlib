// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SoftwareElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SoftwareElement {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "BuildNumber")]
    pub build_number: Option<String>,

/// 
    #[serde(rename = "CodeSet")]
    pub code_set: Option<String>,

/// 
    #[serde(rename = "IdentificationCode")]
    pub identification_code: Option<String>,

/// 
    #[serde(rename = "LanguageEdition")]
    pub language_edition: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "OtherTargetOS")]
    pub other_target_os: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

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

impl CIM_SoftwareElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            build_number: None,
            code_set: None,
            identification_code: None,
            language_edition: None,
            manufacturer: None,
            other_target_os: None,
            serial_number: None,
            software_element_id: None,
            software_element_state: None,
            target_operating_system: None,
            version: None,
        }
    }


    /// Sets the value of BuildNumber
    pub fn set_build_number(&mut self, value: String) {
        self.build_number = Some(value);
    }

    /// Gets the value of BuildNumber
    pub fn get_build_number(&self) -> Option<&String> {
        self.build_number.as_ref()
    }

    /// Sets the value of CodeSet
    pub fn set_code_set(&mut self, value: String) {
        self.code_set = Some(value);
    }

    /// Gets the value of CodeSet
    pub fn get_code_set(&self) -> Option<&String> {
        self.code_set.as_ref()
    }

    /// Sets the value of IdentificationCode
    pub fn set_identification_code(&mut self, value: String) {
        self.identification_code = Some(value);
    }

    /// Gets the value of IdentificationCode
    pub fn get_identification_code(&self) -> Option<&String> {
        self.identification_code.as_ref()
    }

    /// Sets the value of LanguageEdition
    pub fn set_language_edition(&mut self, value: String) {
        self.language_edition = Some(value);
    }

    /// Gets the value of LanguageEdition
    pub fn get_language_edition(&self) -> Option<&String> {
        self.language_edition.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of OtherTargetOS
    pub fn set_other_target_os(&mut self, value: String) {
        self.other_target_os = Some(value);
    }

    /// Gets the value of OtherTargetOS
    pub fn get_other_target_os(&self) -> Option<&String> {
        self.other_target_os.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
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
}

