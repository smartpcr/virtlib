// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemReferencePointExportSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemReferencePointExportSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BaseReferencePoint")]
    pub base_reference_point: Option<String>,

/// 
    #[serde(rename = "DisksToExport")]
    pub disks_to_export: Vec<String>,
}

impl Msvm_VirtualSystemReferencePointExportSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            base_reference_point: None,
            disks_to_export: Vec::new(),
        }
    }


    /// Sets the value of BaseReferencePoint
    pub fn set_base_reference_point(&mut self, value: String) {
        self.base_reference_point = Some(value);
    }

    /// Gets the value of BaseReferencePoint
    pub fn get_base_reference_point(&self) -> Option<&String> {
        self.base_reference_point.as_ref()
    }

    /// Sets the value of DisksToExport
    pub fn set_disks_to_export(&mut self, value: Vec<String>) {
        self.disks_to_export = value;
    }

    /// Gets the value of DisksToExport
    pub fn get_disks_to_export(&self) -> &Vec<String> {
        &self.disks_to_export
    }
}

