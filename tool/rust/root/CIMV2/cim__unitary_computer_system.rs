// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_UnitaryComputerSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_UnitaryComputerSystem {
    #[serde(flatten)]
    pub base: CIM_ComputerSystem,

/// 
    #[serde(rename = "InitialLoadInfo")]
    pub initial_load_info: Vec<String>,

/// 
    #[serde(rename = "LastLoadInfo")]
    pub last_load_info: Option<String>,

/// 
    #[serde(rename = "PowerManagementCapabilities")]
    pub power_management_capabilities: Vec<u16>,

/// 
    #[serde(rename = "PowerManagementSupported")]
    pub power_management_supported: Option<bool>,

/// 
    #[serde(rename = "PowerState")]
    pub power_state: Option<u16>,

/// 
    #[serde(rename = "ResetCapability")]
    pub reset_capability: Option<u16>,
}

impl CIM_UnitaryComputerSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ComputerSystem::new(),
            initial_load_info: Vec::new(),
            last_load_info: None,
            power_management_capabilities: Vec::new(),
            power_management_supported: None,
            power_state: None,
            reset_capability: None,
        }
    }


    /// Sets the value of InitialLoadInfo
    pub fn set_initial_load_info(&mut self, value: Vec<String>) {
        self.initial_load_info = value;
    }

    /// Gets the value of InitialLoadInfo
    pub fn get_initial_load_info(&self) -> &Vec<String> {
        &self.initial_load_info
    }

    /// Sets the value of LastLoadInfo
    pub fn set_last_load_info(&mut self, value: String) {
        self.last_load_info = Some(value);
    }

    /// Gets the value of LastLoadInfo
    pub fn get_last_load_info(&self) -> Option<&String> {
        self.last_load_info.as_ref()
    }

    /// Sets the value of PowerManagementCapabilities
    pub fn set_power_management_capabilities(&mut self, value: Vec<u16>) {
        self.power_management_capabilities = value;
    }

    /// Gets the value of PowerManagementCapabilities
    pub fn get_power_management_capabilities(&self) -> &Vec<u16> {
        &self.power_management_capabilities
    }

    /// Sets the value of PowerManagementSupported
    pub fn set_power_management_supported(&mut self, value: bool) {
        self.power_management_supported = Some(value);
    }

    /// Gets the value of PowerManagementSupported
    pub fn get_power_management_supported(&self) -> Option<&bool> {
        self.power_management_supported.as_ref()
    }

    /// Sets the value of PowerState
    pub fn set_power_state(&mut self, value: u16) {
        self.power_state = Some(value);
    }

    /// Gets the value of PowerState
    pub fn get_power_state(&self) -> Option<&u16> {
        self.power_state.as_ref()
    }

    /// Sets the value of ResetCapability
    pub fn set_reset_capability(&mut self, value: u16) {
        self.reset_capability = Some(value);
    }

    /// Gets the value of ResetCapability
    pub fn get_reset_capability(&self) -> Option<&u16> {
        self.reset_capability.as_ref()
    }

/// 

    /// * `power_state` -  (u16)
    /// * `time` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_power_state(&self, power_state: u16, time: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PowerState".to_string(), value: power_state.into() });
        args.push(MethodParameter { name: "Time".to_string(), value: time.into() });
        self.invoke_method("SetPowerState", &args)

    }

}

