// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Synthetic3DServiceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Synthetic3DServiceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "GPUOvercommitEnabled")]
    pub gpuovercommit_enabled: Option<bool>,
}

impl Msvm_Synthetic3DServiceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            gpuovercommit_enabled: None,
        }
    }


    /// Sets the value of GPUOvercommitEnabled
    pub fn set_gpuovercommit_enabled(&mut self, value: bool) {
        self.gpuovercommit_enabled = Some(value);
    }

    /// Gets the value of GPUOvercommitEnabled
    pub fn get_gpuovercommit_enabled(&self) -> Option<&bool> {
        self.gpuovercommit_enabled.as_ref()
    }
}

impl Msvm_Synthetic3DServiceSettingData {
    /// Gets the related Msvm_Synthetic3DService object(s)
    pub fn get_related__synthetic3_dservice(&self) -> Result<Msvm_Synthetic3DService, WmiError> {
        self.get_related("Msvm_Synthetic3DService")
    }

}

