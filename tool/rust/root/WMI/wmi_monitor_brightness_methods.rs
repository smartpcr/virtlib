// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WmiMonitorBrightnessMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WmiMonitorBrightnessMethods {
    #[serde(flatten)]
    pub base: MSMonitorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl WmiMonitorBrightnessMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSMonitorClass::new(),
            active: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// 

    /// * `brightness` -  (u8)
    /// * `timeout` -  (u32)
    pub fn wmi_set_brightness(&self, timeout: u32, brightness: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        args.push(MethodParameter { name: "Brightness".to_string(), value: brightness.into() });
        self.invoke_method("WmiSetBrightness", &args)

    }


/// 
    pub fn wmi_revert_to_policy_brightness(&self) -> Result<(), WmiError> {
        self.invoke_method("WmiRevertToPolicyBrightness", &[])

    }


/// 

    /// * `state` -  (bool)
    pub fn wmi_set_alsbrightness_state(&self, state: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "State".to_string(), value: state.into() });
        self.invoke_method("WmiSetALSBrightnessState", &args)

    }


/// 

    /// * `brightness` -  (u8)
    pub fn wmi_set_alsbrightness(&self, brightness: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Brightness".to_string(), value: brightness.into() });
        self.invoke_method("WmiSetALSBrightness", &args)

    }

}

