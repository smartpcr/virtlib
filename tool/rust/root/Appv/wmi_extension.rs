// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Appv
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WMI_extension struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WMI_extension {
    #[serde(flatten)]
    pub base: __Win32Provider,

/// 
    #[serde(rename = "AssemblyName")]
    pub assembly_name: Option<String>,

/// 
    #[serde(rename = "AssemblyPath")]
    pub assembly_path: Option<String>,

/// 
    #[serde(rename = "CLRVersion")]
    pub clrversion: Option<String>,
}

impl WMI_extension {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Win32Provider::new(),
            assembly_name: None,
            assembly_path: None,
            clrversion: None,
        }
    }


    /// Sets the value of AssemblyName
    pub fn set_assembly_name(&mut self, value: String) {
        self.assembly_name = Some(value);
    }

    /// Gets the value of AssemblyName
    pub fn get_assembly_name(&self) -> Option<&String> {
        self.assembly_name.as_ref()
    }

    /// Sets the value of AssemblyPath
    pub fn set_assembly_path(&mut self, value: String) {
        self.assembly_path = Some(value);
    }

    /// Gets the value of AssemblyPath
    pub fn get_assembly_path(&self) -> Option<&String> {
        self.assembly_path.as_ref()
    }

    /// Sets the value of CLRVersion
    pub fn set_clrversion(&mut self, value: String) {
        self.clrversion = Some(value);
    }

    /// Gets the value of CLRVersion
    pub fn get_clrversion(&self) -> Option<&String> {
        self.clrversion.as_ref()
    }
}

