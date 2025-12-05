// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemUpgradeSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemUpgradeSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "TargetVersion")]
    pub target_version: Option<String>,
}

impl Msvm_VirtualSystemUpgradeSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            target_version: None,
        }
    }


    /// Sets the value of TargetVersion
    pub fn set_target_version(&mut self, value: String) {
        self.target_version = Some(value);
    }

    /// Gets the value of TargetVersion
    pub fn get_target_version(&self) -> Option<&String> {
        self.target_version.as_ref()
    }
}

