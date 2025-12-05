// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_FlexIoDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_FlexIoDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "EmulatorConfiguration")]
    pub emulator_configuration: Vec<String>,

/// 
    #[serde(rename = "EmulatorId")]
    pub emulator_id: Option<String>,
}

impl Msvm_FlexIoDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            emulator_configuration: Vec::new(),
            emulator_id: None,
        }
    }


    /// Sets the value of EmulatorConfiguration
    pub fn set_emulator_configuration(&mut self, value: Vec<String>) {
        self.emulator_configuration = value;
    }

    /// Gets the value of EmulatorConfiguration
    pub fn get_emulator_configuration(&self) -> &Vec<String> {
        &self.emulator_configuration
    }

    /// Sets the value of EmulatorId
    pub fn set_emulator_id(&mut self, value: String) {
        self.emulator_id = Some(value);
    }

    /// Gets the value of EmulatorId
    pub fn get_emulator_id(&self) -> Option<&String> {
        self.emulator_id.as_ref()
    }
}

