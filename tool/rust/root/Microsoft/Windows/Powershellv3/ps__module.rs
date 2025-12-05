// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Powershellv3
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_Module struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_Module {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "moduleManifestFileData")]
    pub module_manifest_file_data: Vec<u8>,

/// 
    #[serde(rename = "ModuleName")]
    pub module_name: Option<String>,

/// 
    #[serde(rename = "moduleType")]
    pub module_type: Option<u16>,
}

impl PS_Module {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            module_manifest_file_data: Vec::new(),
            module_name: None,
            module_type: None,
        }
    }


    /// Sets the value of moduleManifestFileData
    pub fn set_module_manifest_file_data(&mut self, value: Vec<u8>) {
        self.module_manifest_file_data = value;
    }

    /// Gets the value of moduleManifestFileData
    pub fn get_module_manifest_file_data(&self) -> &Vec<u8> {
        &self.module_manifest_file_data
    }

    /// Sets the value of ModuleName
    pub fn set_module_name(&mut self, value: String) {
        self.module_name = Some(value);
    }

    /// Gets the value of ModuleName
    pub fn get_module_name(&self) -> Option<&String> {
        self.module_name.as_ref()
    }

    /// Sets the value of moduleType
    pub fn set_module_type(&mut self, value: u16) {
        self.module_type = Some(value);
    }

    /// Gets the value of moduleType
    pub fn get_module_type(&self) -> Option<&u16> {
        self.module_type.as_ref()
    }
}

