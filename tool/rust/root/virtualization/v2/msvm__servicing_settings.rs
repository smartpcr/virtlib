// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ServicingSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ServicingSettings {

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl Msvm_ServicingSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            version: None,
        }
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

