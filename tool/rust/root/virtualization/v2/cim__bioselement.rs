// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BIOSElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BIOSElement {
    #[serde(flatten)]
    pub base: CIM_SoftwareElement,

/// The currently selected language for the BIOS. This information can be obtained from SMBIOS, using the Current Language attribute of the Type 13 structure, to index into the string list following the structure. The property is formatted using the ISO 639 Language Name, and may be followed by the ISO 3166 Territory Name and the encoding method.
    #[serde(rename = "CurrentLanguage")]
    pub current_language: Option<String>,

/// A list of installable languages for the BIOS. This information can be obtained from SMBIOS, from the string list that follows the Type 13 structure. An ISO 639 Language Name should be used to specify the BIOS' installable languages. The ISO 3166 Territory Name and the encoding method may also be specified, following the Language Name.
    #[serde(rename = "ListOfLanguages")]
    pub list_of_languages: Vec<String>,

/// The ending address of the memory which this BIOS occupies.
    #[serde(rename = "LoadedEndingAddress")]
    pub loaded_ending_address: Option<u64>,

/// The starting address of the memory which this BIOS occupies.
    #[serde(rename = "LoadedStartingAddress")]
    pub loaded_starting_address: Option<u64>,

/// A free form string describing the BIOS flash/load utility that is required to update the BIOSElement. Version and other information may be indicated in this property.
    #[serde(rename = "LoadUtilityInformation")]
    pub load_utility_information: Option<String>,

/// If true, this is the primary BIOS of the ComputerSystem.
    #[serde(rename = "PrimaryBIOS")]
    pub primary_bios: Option<bool>,

/// A string representing the publication location of the BIOS Attribute registry or registries the implementation complies to.
    #[serde(rename = "RegistryURIs")]
    pub registry_uris: Vec<String>,

/// Date that this BIOS was released.
    #[serde(rename = "ReleaseDate")]
    pub release_date: Option<String>,
}

impl CIM_BIOSElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SoftwareElement::new(),
            current_language: None,
            list_of_languages: Vec::new(),
            loaded_ending_address: None,
            loaded_starting_address: None,
            load_utility_information: None,
            primary_bios: None,
            registry_uris: Vec::new(),
            release_date: None,
        }
    }


    /// Sets the value of CurrentLanguage
    pub fn set_current_language(&mut self, value: String) {
        self.current_language = Some(value);
    }

    /// Gets the value of CurrentLanguage
    pub fn get_current_language(&self) -> Option<&String> {
        self.current_language.as_ref()
    }

    /// Sets the value of ListOfLanguages
    pub fn set_list_of_languages(&mut self, value: Vec<String>) {
        self.list_of_languages = value;
    }

    /// Gets the value of ListOfLanguages
    pub fn get_list_of_languages(&self) -> &Vec<String> {
        &self.list_of_languages
    }

    /// Sets the value of LoadedEndingAddress
    pub fn set_loaded_ending_address(&mut self, value: u64) {
        self.loaded_ending_address = Some(value);
    }

    /// Gets the value of LoadedEndingAddress
    pub fn get_loaded_ending_address(&self) -> Option<&u64> {
        self.loaded_ending_address.as_ref()
    }

    /// Sets the value of LoadedStartingAddress
    pub fn set_loaded_starting_address(&mut self, value: u64) {
        self.loaded_starting_address = Some(value);
    }

    /// Gets the value of LoadedStartingAddress
    pub fn get_loaded_starting_address(&self) -> Option<&u64> {
        self.loaded_starting_address.as_ref()
    }

    /// Sets the value of LoadUtilityInformation
    pub fn set_load_utility_information(&mut self, value: String) {
        self.load_utility_information = Some(value);
    }

    /// Gets the value of LoadUtilityInformation
    pub fn get_load_utility_information(&self) -> Option<&String> {
        self.load_utility_information.as_ref()
    }

    /// Sets the value of PrimaryBIOS
    pub fn set_primary_bios(&mut self, value: bool) {
        self.primary_bios = Some(value);
    }

    /// Gets the value of PrimaryBIOS
    pub fn get_primary_bios(&self) -> Option<&bool> {
        self.primary_bios.as_ref()
    }

    /// Sets the value of RegistryURIs
    pub fn set_registry_uris(&mut self, value: Vec<String>) {
        self.registry_uris = value;
    }

    /// Gets the value of RegistryURIs
    pub fn get_registry_uris(&self) -> &Vec<String> {
        &self.registry_uris
    }

    /// Sets the value of ReleaseDate
    pub fn set_release_date(&mut self, value: String) {
        self.release_date = Some(value);
    }

    /// Gets the value of ReleaseDate
    pub fn get_release_date(&self) -> Option<&String> {
        self.release_date.as_ref()
    }
}

