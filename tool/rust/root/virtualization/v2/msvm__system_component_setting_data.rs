// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SystemComponentSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SystemComponentSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,
}

impl Msvm_SystemComponentSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
        }
    }

}

impl Msvm_SystemComponentSettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

