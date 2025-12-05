// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WHEAErrorInjectionMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WHEAErrorInjectionMethods {
    #[serde(flatten)]
    pub base: WHEA,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl WHEAErrorInjectionMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WHEA::new(),
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

    /// * `capabilities` -  (u32)
    /// * `status` -  (u32)
    pub fn get_error_injection_capabilities_rtn(&self, status: &mut u32, capabilities: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetErrorInjectionCapabilitiesRtn", &[])?;
        let capabilities = result.get_value("Capabilities")?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `error_type` -  (u32)
    /// * `parameter1` -  (u64)
    /// * `parameter2` -  (u64)
    /// * `parameter3` -  (u64)
    /// * `parameter4` -  (u64)

    /// * `status` -  (u32)
    pub fn inject_error_rtn(&self, error_type: u32, parameter1: u64, parameter2: u64, parameter3: u64, parameter4: u64, status: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ErrorType".to_string(), value: error_type.into() });
        args.push(MethodParameter { name: "Parameter1".to_string(), value: parameter1.into() });
        args.push(MethodParameter { name: "Parameter2".to_string(), value: parameter2.into() });
        args.push(MethodParameter { name: "Parameter3".to_string(), value: parameter3.into() });
        args.push(MethodParameter { name: "Parameter4".to_string(), value: parameter4.into() });

        let result = self.invoke_method("InjectErrorRtn", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

