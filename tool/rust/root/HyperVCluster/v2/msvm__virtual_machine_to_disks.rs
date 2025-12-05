// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualMachineToDisks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualMachineToDisks {

/// 
    #[serde(rename = "DisksToExport")]
    pub disks_to_export: Vec<String>,

/// 
    #[serde(rename = "VirtualMachineId")]
    pub virtual_machine_id: Option<String>,
}

impl Msvm_VirtualMachineToDisks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disks_to_export: Vec::new(),
            virtual_machine_id: None,
        }
    }


    /// Sets the value of DisksToExport
    pub fn set_disks_to_export(&mut self, value: Vec<String>) {
        self.disks_to_export = value;
    }

    /// Gets the value of DisksToExport
    pub fn get_disks_to_export(&self) -> &Vec<String> {
        &self.disks_to_export
    }

    /// Sets the value of VirtualMachineId
    pub fn set_virtual_machine_id(&mut self, value: String) {
        self.virtual_machine_id = Some(value);
    }

    /// Gets the value of VirtualMachineId
    pub fn get_virtual_machine_id(&self) -> Option<&String> {
        self.virtual_machine_id.as_ref()
    }
}

