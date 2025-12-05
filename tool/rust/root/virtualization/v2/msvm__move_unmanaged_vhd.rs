// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_MoveUnmanagedVhd struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_MoveUnmanagedVhd {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "VhdDestinationPath")]
    pub vhd_destination_path: Option<String>,

/// 
    #[serde(rename = "VhdSourcePath")]
    pub vhd_source_path: Option<String>,
}

impl Msvm_MoveUnmanagedVhd {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            vhd_destination_path: None,
            vhd_source_path: None,
        }
    }


    /// Sets the value of VhdDestinationPath
    pub fn set_vhd_destination_path(&mut self, value: String) {
        self.vhd_destination_path = Some(value);
    }

    /// Gets the value of VhdDestinationPath
    pub fn get_vhd_destination_path(&self) -> Option<&String> {
        self.vhd_destination_path.as_ref()
    }

    /// Sets the value of VhdSourcePath
    pub fn set_vhd_source_path(&mut self, value: String) {
        self.vhd_source_path = Some(value);
    }

    /// Gets the value of VhdSourcePath
    pub fn get_vhd_source_path(&self) -> Option<&String> {
        self.vhd_source_path.as_ref()
    }
}

