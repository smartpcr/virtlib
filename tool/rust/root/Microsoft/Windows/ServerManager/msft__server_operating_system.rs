// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerOperatingSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerOperatingSystem {

/// 
    #[serde(rename = "Architecture")]
    pub architecture: Option<u8>,

/// 
    #[serde(rename = "BuildNumber")]
    pub build_number: Option<u32>,

/// 
    #[serde(rename = "Language")]
    pub language: Option<String>,

/// 
    #[serde(rename = "MajorVersion")]
    pub major_version: Option<u32>,

/// 
    #[serde(rename = "MinorVersion")]
    pub minor_version: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "SKU")]
    pub sku: Option<String>,

/// 
    #[serde(rename = "SKUId")]
    pub skuid: Option<u32>,

/// 
    #[serde(rename = "SPMajorVersion")]
    pub spmajor_version: Option<u16>,

/// 
    #[serde(rename = "SPMinorVersion")]
    pub spminor_version: Option<u16>,
}

impl MSFT_ServerOperatingSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            architecture: None,
            build_number: None,
            language: None,
            major_version: None,
            minor_version: None,
            name: None,
            sku: None,
            skuid: None,
            spmajor_version: None,
            spminor_version: None,
        }
    }


    /// Sets the value of Architecture
    pub fn set_architecture(&mut self, value: u8) {
        self.architecture = Some(value);
    }

    /// Gets the value of Architecture
    pub fn get_architecture(&self) -> Option<&u8> {
        self.architecture.as_ref()
    }

    /// Sets the value of BuildNumber
    pub fn set_build_number(&mut self, value: u32) {
        self.build_number = Some(value);
    }

    /// Gets the value of BuildNumber
    pub fn get_build_number(&self) -> Option<&u32> {
        self.build_number.as_ref()
    }

    /// Sets the value of Language
    pub fn set_language(&mut self, value: String) {
        self.language = Some(value);
    }

    /// Gets the value of Language
    pub fn get_language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Sets the value of MajorVersion
    pub fn set_major_version(&mut self, value: u32) {
        self.major_version = Some(value);
    }

    /// Gets the value of MajorVersion
    pub fn get_major_version(&self) -> Option<&u32> {
        self.major_version.as_ref()
    }

    /// Sets the value of MinorVersion
    pub fn set_minor_version(&mut self, value: u32) {
        self.minor_version = Some(value);
    }

    /// Gets the value of MinorVersion
    pub fn get_minor_version(&self) -> Option<&u32> {
        self.minor_version.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of SKU
    pub fn set_sku(&mut self, value: String) {
        self.sku = Some(value);
    }

    /// Gets the value of SKU
    pub fn get_sku(&self) -> Option<&String> {
        self.sku.as_ref()
    }

    /// Sets the value of SKUId
    pub fn set_skuid(&mut self, value: u32) {
        self.skuid = Some(value);
    }

    /// Gets the value of SKUId
    pub fn get_skuid(&self) -> Option<&u32> {
        self.skuid.as_ref()
    }

    /// Sets the value of SPMajorVersion
    pub fn set_spmajor_version(&mut self, value: u16) {
        self.spmajor_version = Some(value);
    }

    /// Gets the value of SPMajorVersion
    pub fn get_spmajor_version(&self) -> Option<&u16> {
        self.spmajor_version.as_ref()
    }

    /// Sets the value of SPMinorVersion
    pub fn set_spminor_version(&mut self, value: u16) {
        self.spminor_version = Some(value);
    }

    /// Gets the value of SPMinorVersion
    pub fn get_spminor_version(&self) -> Option<&u16> {
        self.spminor_version.as_ref()
    }
}

