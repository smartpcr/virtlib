// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ComputerSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ComputerSystem {
    #[serde(flatten)]
    pub base: CIM_System,

/// 
    #[serde(rename = "Dedicated")]
    pub dedicated: Vec<u16>,

/// 
    #[serde(rename = "IdentifyingDescriptions")]
    pub identifying_descriptions: Vec<String>,

/// 
    #[serde(rename = "OtherDedicatedDescriptions")]
    pub other_dedicated_descriptions: Vec<String>,

/// 
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,

/// 
    #[serde(rename = "PowerManagementCapabilities")]
    pub power_management_capabilities: Vec<u16>,

/// 
    #[serde(rename = "ResetCapability")]
    pub reset_capability: Option<u16>,
}

impl CIM_ComputerSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_System::new(),
            dedicated: Vec::new(),
            identifying_descriptions: Vec::new(),
            other_dedicated_descriptions: Vec::new(),
            other_identifying_info: Vec::new(),
            power_management_capabilities: Vec::new(),
            reset_capability: None,
        }
    }


    /// Sets the value of Dedicated
    pub fn set_dedicated(&mut self, value: Vec<u16>) {
        self.dedicated = value;
    }

    /// Gets the value of Dedicated
    pub fn get_dedicated(&self) -> &Vec<u16> {
        &self.dedicated
    }

    /// Sets the value of IdentifyingDescriptions
    pub fn set_identifying_descriptions(&mut self, value: Vec<String>) {
        self.identifying_descriptions = value;
    }

    /// Gets the value of IdentifyingDescriptions
    pub fn get_identifying_descriptions(&self) -> &Vec<String> {
        &self.identifying_descriptions
    }

    /// Sets the value of OtherDedicatedDescriptions
    pub fn set_other_dedicated_descriptions(&mut self, value: Vec<String>) {
        self.other_dedicated_descriptions = value;
    }

    /// Gets the value of OtherDedicatedDescriptions
    pub fn get_other_dedicated_descriptions(&self) -> &Vec<String> {
        &self.other_dedicated_descriptions
    }

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: Vec<String>) {
        self.other_identifying_info = value;
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> &Vec<String> {
        &self.other_identifying_info
    }

    /// Sets the value of PowerManagementCapabilities
    pub fn set_power_management_capabilities(&mut self, value: Vec<u16>) {
        self.power_management_capabilities = value;
    }

    /// Gets the value of PowerManagementCapabilities
    pub fn get_power_management_capabilities(&self) -> &Vec<u16> {
        &self.power_management_capabilities
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

    /// * `power_state` -  (u32)
    /// * `time` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_power_state(&self, power_state: u32, time: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PowerState".to_string(), value: power_state.into() });
        args.push(MethodParameter { name: "Time".to_string(), value: time.into() });
        self.invoke_method("SetPowerState", &args)

    }

}

