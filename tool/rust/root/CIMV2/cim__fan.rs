// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Fan struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Fan {
    #[serde(flatten)]
    pub base: CIM_CoolingDevice,

/// 
    #[serde(rename = "DesiredSpeed")]
    pub desired_speed: Option<u64>,

/// 
    #[serde(rename = "VariableSpeed")]
    pub variable_speed: Option<bool>,
}

impl CIM_Fan {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_CoolingDevice::new(),
            desired_speed: None,
            variable_speed: None,
        }
    }


    /// Sets the value of DesiredSpeed
    pub fn set_desired_speed(&mut self, value: u64) {
        self.desired_speed = Some(value);
    }

    /// Gets the value of DesiredSpeed
    pub fn get_desired_speed(&self) -> Option<&u64> {
        self.desired_speed.as_ref()
    }

    /// Sets the value of VariableSpeed
    pub fn set_variable_speed(&mut self, value: bool) {
        self.variable_speed = Some(value);
    }

    /// Gets the value of VariableSpeed
    pub fn get_variable_speed(&self) -> Option<&bool> {
        self.variable_speed.as_ref()
    }

/// 

    /// * `desired_speed` -  (u64)

    /// * `return_value` -  (u32)
    pub fn set_speed(&self, desired_speed: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DesiredSpeed".to_string(), value: desired_speed.into() });
        self.invoke_method("SetSpeed", &args)

    }

}

