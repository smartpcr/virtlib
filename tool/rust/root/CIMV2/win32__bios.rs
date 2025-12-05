// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_BIOS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_BIOS {
    #[serde(flatten)]
    pub base: CIM_BIOSElement,

/// 
    #[serde(rename = "BiosCharacteristics")]
    pub bios_characteristics: Vec<u16>,

/// 
    #[serde(rename = "BIOSVersion")]
    pub biosversion: Vec<String>,

/// 
    #[serde(rename = "CurrentLanguage")]
    pub current_language: Option<String>,

/// 
    #[serde(rename = "EmbeddedControllerMajorVersion")]
    pub embedded_controller_major_version: Option<u8>,

/// 
    #[serde(rename = "EmbeddedControllerMinorVersion")]
    pub embedded_controller_minor_version: Option<u8>,

/// 
    #[serde(rename = "InstallableLanguages")]
    pub installable_languages: Option<u16>,

/// 
    #[serde(rename = "ListOfLanguages")]
    pub list_of_languages: Vec<String>,

/// 
    #[serde(rename = "ReleaseDate")]
    pub release_date: Option<String>,

/// 
    #[serde(rename = "SMBIOSBIOSVersion")]
    pub smbiosbiosversion: Option<String>,

/// 
    #[serde(rename = "SMBIOSMajorVersion")]
    pub smbiosmajor_version: Option<u16>,

/// 
    #[serde(rename = "SMBIOSMinorVersion")]
    pub smbiosminor_version: Option<u16>,

/// 
    #[serde(rename = "SMBIOSPresent")]
    pub smbiospresent: Option<bool>,

/// 
    #[serde(rename = "SystemBiosMajorVersion")]
    pub system_bios_major_version: Option<u8>,

/// 
    #[serde(rename = "SystemBiosMinorVersion")]
    pub system_bios_minor_version: Option<u8>,
}

impl Win32_BIOS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BIOSElement::new(),
            bios_characteristics: Vec::new(),
            biosversion: Vec::new(),
            current_language: None,
            embedded_controller_major_version: None,
            embedded_controller_minor_version: None,
            installable_languages: None,
            list_of_languages: Vec::new(),
            release_date: None,
            smbiosbiosversion: None,
            smbiosmajor_version: None,
            smbiosminor_version: None,
            smbiospresent: None,
            system_bios_major_version: None,
            system_bios_minor_version: None,
        }
    }


    /// Sets the value of BiosCharacteristics
    pub fn set_bios_characteristics(&mut self, value: Vec<u16>) {
        self.bios_characteristics = value;
    }

    /// Gets the value of BiosCharacteristics
    pub fn get_bios_characteristics(&self) -> &Vec<u16> {
        &self.bios_characteristics
    }

    /// Sets the value of BIOSVersion
    pub fn set_biosversion(&mut self, value: Vec<String>) {
        self.biosversion = value;
    }

    /// Gets the value of BIOSVersion
    pub fn get_biosversion(&self) -> &Vec<String> {
        &self.biosversion
    }

    /// Sets the value of CurrentLanguage
    pub fn set_current_language(&mut self, value: String) {
        self.current_language = Some(value);
    }

    /// Gets the value of CurrentLanguage
    pub fn get_current_language(&self) -> Option<&String> {
        self.current_language.as_ref()
    }

    /// Sets the value of EmbeddedControllerMajorVersion
    pub fn set_embedded_controller_major_version(&mut self, value: u8) {
        self.embedded_controller_major_version = Some(value);
    }

    /// Gets the value of EmbeddedControllerMajorVersion
    pub fn get_embedded_controller_major_version(&self) -> Option<&u8> {
        self.embedded_controller_major_version.as_ref()
    }

    /// Sets the value of EmbeddedControllerMinorVersion
    pub fn set_embedded_controller_minor_version(&mut self, value: u8) {
        self.embedded_controller_minor_version = Some(value);
    }

    /// Gets the value of EmbeddedControllerMinorVersion
    pub fn get_embedded_controller_minor_version(&self) -> Option<&u8> {
        self.embedded_controller_minor_version.as_ref()
    }

    /// Sets the value of InstallableLanguages
    pub fn set_installable_languages(&mut self, value: u16) {
        self.installable_languages = Some(value);
    }

    /// Gets the value of InstallableLanguages
    pub fn get_installable_languages(&self) -> Option<&u16> {
        self.installable_languages.as_ref()
    }

    /// Sets the value of ListOfLanguages
    pub fn set_list_of_languages(&mut self, value: Vec<String>) {
        self.list_of_languages = value;
    }

    /// Gets the value of ListOfLanguages
    pub fn get_list_of_languages(&self) -> &Vec<String> {
        &self.list_of_languages
    }

    /// Sets the value of ReleaseDate
    pub fn set_release_date(&mut self, value: String) {
        self.release_date = Some(value);
    }

    /// Gets the value of ReleaseDate
    pub fn get_release_date(&self) -> Option<&String> {
        self.release_date.as_ref()
    }

    /// Sets the value of SMBIOSBIOSVersion
    pub fn set_smbiosbiosversion(&mut self, value: String) {
        self.smbiosbiosversion = Some(value);
    }

    /// Gets the value of SMBIOSBIOSVersion
    pub fn get_smbiosbiosversion(&self) -> Option<&String> {
        self.smbiosbiosversion.as_ref()
    }

    /// Sets the value of SMBIOSMajorVersion
    pub fn set_smbiosmajor_version(&mut self, value: u16) {
        self.smbiosmajor_version = Some(value);
    }

    /// Gets the value of SMBIOSMajorVersion
    pub fn get_smbiosmajor_version(&self) -> Option<&u16> {
        self.smbiosmajor_version.as_ref()
    }

    /// Sets the value of SMBIOSMinorVersion
    pub fn set_smbiosminor_version(&mut self, value: u16) {
        self.smbiosminor_version = Some(value);
    }

    /// Gets the value of SMBIOSMinorVersion
    pub fn get_smbiosminor_version(&self) -> Option<&u16> {
        self.smbiosminor_version.as_ref()
    }

    /// Sets the value of SMBIOSPresent
    pub fn set_smbiospresent(&mut self, value: bool) {
        self.smbiospresent = Some(value);
    }

    /// Gets the value of SMBIOSPresent
    pub fn get_smbiospresent(&self) -> Option<&bool> {
        self.smbiospresent.as_ref()
    }

    /// Sets the value of SystemBiosMajorVersion
    pub fn set_system_bios_major_version(&mut self, value: u8) {
        self.system_bios_major_version = Some(value);
    }

    /// Gets the value of SystemBiosMajorVersion
    pub fn get_system_bios_major_version(&self) -> Option<&u8> {
        self.system_bios_major_version.as_ref()
    }

    /// Sets the value of SystemBiosMinorVersion
    pub fn set_system_bios_minor_version(&mut self, value: u8) {
        self.system_bios_minor_version = Some(value);
    }

    /// Gets the value of SystemBiosMinorVersion
    pub fn get_system_bios_minor_version(&self) -> Option<&u8> {
        self.system_bios_minor_version.as_ref()
    }
}

