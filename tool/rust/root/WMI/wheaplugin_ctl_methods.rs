// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WHEAPluginCtlMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WHEAPluginCtlMethods {
    #[serde(flatten)]
    pub base: WHEA,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl WHEAPluginCtlMethods {
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

    /// * `input_buffer` -  (u8[])
    /// * `input_length` -  (u32)

    /// * `output_buffer` -  (u8[])
    /// * `output_length` -  (u32)
    /// * `status` -  (u32)
    pub fn whea_plugin_ctl_interface_rtn(&self, input_length: u32, input_buffer: &Vec<u8>, status: &mut u32, output_length: &mut u32, output_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputLength".to_string(), value: input_length.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });

        let result = self.invoke_method("WheaPluginCtlInterfaceRtn", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_length = result.get_value("OutputLength")?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

