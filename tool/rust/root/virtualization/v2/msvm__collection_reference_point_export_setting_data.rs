// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionReferencePointExportSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionReferencePointExportSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BaseReferencePointCollection")]
    pub base_reference_point_collection: Option<String>,

/// 
    #[serde(rename = "VirtualMachinesToDisksToExport")]
    pub virtual_machines_to_disks_to_export: Vec<String>,
}

impl Msvm_CollectionReferencePointExportSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            base_reference_point_collection: None,
            virtual_machines_to_disks_to_export: Vec::new(),
        }
    }


    /// Sets the value of BaseReferencePointCollection
    pub fn set_base_reference_point_collection(&mut self, value: String) {
        self.base_reference_point_collection = Some(value);
    }

    /// Gets the value of BaseReferencePointCollection
    pub fn get_base_reference_point_collection(&self) -> Option<&String> {
        self.base_reference_point_collection.as_ref()
    }

    /// Sets the value of VirtualMachinesToDisksToExport
    pub fn set_virtual_machines_to_disks_to_export(&mut self, value: Vec<String>) {
        self.virtual_machines_to_disks_to_export = value;
    }

    /// Gets the value of VirtualMachinesToDisksToExport
    pub fn get_virtual_machines_to_disks_to_export(&self) -> &Vec<String> {
        &self.virtual_machines_to_disks_to_export
    }
}

