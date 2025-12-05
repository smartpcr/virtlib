// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SerialPortSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SerialPortSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "DebuggerMode")]
    pub debugger_mode: Option<bool>,
}

impl Msvm_SerialPortSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            debugger_mode: None,
        }
    }


    /// Sets the value of DebuggerMode
    pub fn set_debugger_mode(&mut self, value: bool) {
        self.debugger_mode = Some(value);
    }

    /// Gets the value of DebuggerMode
    pub fn get_debugger_mode(&self) -> Option<&bool> {
        self.debugger_mode.as_ref()
    }
}

impl Msvm_SerialPortSettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

    /// Gets the related Msvm_ResourceAllocationSettingData object(s)
    pub fn get_related__resource_allocation_setting_data(&self) -> Result<Msvm_ResourceAllocationSettingData, WmiError> {
        self.get_related("Msvm_ResourceAllocationSettingData")
    }

    /// Gets the related Msvm_SerialPortSettingData object(s)
    pub fn get_related__serial_port_setting_data(&self) -> Result<Vec<Msvm_SerialPortSettingData>, WmiError> {
        self.get_all_related("Msvm_SerialPortSettingData")
    }

}

