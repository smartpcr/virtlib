// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_InstalledWin32Program struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_InstalledWin32Program {

/// 
    #[serde(rename = "Language")]
    pub language: Option<String>,

/// 
    #[serde(rename = "MsiPackageCode")]
    pub msi_package_code: Option<String>,

/// 
    #[serde(rename = "MsiProductCode")]
    pub msi_product_code: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ProgramId")]
    pub program_id: Option<String>,

/// 
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl Win32_InstalledWin32Program {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            language: None,
            msi_package_code: None,
            msi_product_code: None,
            name: None,
            program_id: None,
            vendor: None,
            version: None,
        }
    }


    /// Sets the value of Language
    pub fn set_language(&mut self, value: String) {
        self.language = Some(value);
    }

    /// Gets the value of Language
    pub fn get_language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Sets the value of MsiPackageCode
    pub fn set_msi_package_code(&mut self, value: String) {
        self.msi_package_code = Some(value);
    }

    /// Gets the value of MsiPackageCode
    pub fn get_msi_package_code(&self) -> Option<&String> {
        self.msi_package_code.as_ref()
    }

    /// Sets the value of MsiProductCode
    pub fn set_msi_product_code(&mut self, value: String) {
        self.msi_product_code = Some(value);
    }

    /// Gets the value of MsiProductCode
    pub fn get_msi_product_code(&self) -> Option<&String> {
        self.msi_product_code.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProgramId
    pub fn set_program_id(&mut self, value: String) {
        self.program_id = Some(value);
    }

    /// Gets the value of ProgramId
    pub fn get_program_id(&self) -> Option<&String> {
        self.program_id.as_ref()
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

