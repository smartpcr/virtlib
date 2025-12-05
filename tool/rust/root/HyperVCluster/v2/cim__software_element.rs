// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SoftwareElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SoftwareElement {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// The internal identifier for this compilation of SoftwareElement.
    #[serde(rename = "BuildNumber")]
    pub build_number: Option<String>,

/// The code set used by this SoftwareElement. It defines the bit patterns that a system uses to identify characters. ISO defines various code sets such as UTF-8 and ISO8859-1.
    #[serde(rename = "CodeSet")]
    pub code_set: Option<String>,

/// The manufacturer's identifier for this SoftwareElement. Often this will be a stock keeping unit (SKU) or a part number.
    #[serde(rename = "IdentificationCode")]
    pub identification_code: Option<String>,

/// The value of this property identifies the language edition of this SoftwareElement. The language codes defined in ISO 639 should be used. Where the element represents a multi-lingual or international version, the string "Multilingual" should be used.
    #[serde(rename = "LanguageEdition")]
    pub language_edition: Option<String>,

/// Manufacturer of this SoftwareElement.
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// The OtherTargetOS property records the manufacturer and operating system type for a SoftwareElement when the TargetOperatingSystem property has a value of 1 ("Other"). For all other values of TargetOperatingSystem, the OtherTargetOS property is NULL.
    #[serde(rename = "OtherTargetOS")]
    pub other_target_os: Option<String>,

/// The assigned serial number of this SoftwareElement.
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// This is an identifier for the SoftwareElement and is designed to be used in conjunction with other keys to create a unique representation of the element.
    #[serde(rename = "SoftwareElementID")]
    pub software_element_id: Option<String>,

/// The SoftwareElementState is defined in this model to identify various states of a SoftwareElement's life cycle. 
/// - A SoftwareElement in the deployable state describes the details necessary to successfully distribute it and the details (Checks and Actions) required to move it to the installable state (i.e, the next state). 
/// - A SoftwareElement in the installable state describes the details necessary to successfully install it and the details (Checks and Actions) required to create an element in the executable state (i.e., the next state). 
/// - A SoftwareElement in the executable state describes the details necessary to successfully start it and the details (Checks and Actions) required to move it to the running state (i.e., the next state). 
/// - A SoftwareElement in the running state describes the details necessary to manage the started element.
    #[serde(rename = "SoftwareElementState")]
    pub software_element_state: Option<SoftwareElement_SoftwareElementState>,

/// The TargetOperatingSystem property specifies the element's operating system environment. The value of this property does not ensure that it is binary executable. Two other pieces of information are needed. First, the version of the OS needs to be specified using the class, CIM_OSVersion Check. The second piece of information is the architecture that the OS runs on. This information is verified using CIM_ArchitectureCheck. The combination of these constructs clearly identifies the level of OS required for a particular SoftwareElement.
    #[serde(rename = "TargetOperatingSystem")]
    pub target_operating_system: Option<SoftwareElement_TargetOperatingSystem>,

/// Software Version should be in the form <Major>.<Minor>.<Revision> or <Major>.<Minor><letter><revision>.
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
    pub fn set_software_element_state(&mut self, value: SoftwareElement_SoftwareElementState) {
        self.software_element_state = Some(value);
    }

    /// Gets the value of SoftwareElementState
    pub fn get_software_element_state(&self) -> Option<&SoftwareElement_SoftwareElementState> {
        self.software_element_state.as_ref()
    }

    /// Sets the value of TargetOperatingSystem
    pub fn set_target_operating_system(&mut self, value: SoftwareElement_TargetOperatingSystem) {
        self.target_operating_system = Some(value);
    }

    /// Gets the value of TargetOperatingSystem
    pub fn get_target_operating_system(&self) -> Option<&SoftwareElement_TargetOperatingSystem> {
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

