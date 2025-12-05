// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_SetInterruptModeration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_SetInterruptModeration {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSNdis_SetInterruptModeration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
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

    /// * `interrupt_moderation` -  (MSNdis_InterruptModerationParameters)
    /// * `method_header` -  (MSNdis_WmiMethodHeader)

    /// * `output_info` -  (MSNdis_WmiOutputInfo)
    pub fn wmi_set_interrupt_moderation(&self, method_header: MSNdis_WmiMethodHeader, interrupt_moderation: MSNdis_InterruptModerationParameters, output_info: &mut MSNdis_WmiOutputInfo) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MethodHeader".to_string(), value: method_header.into() });
        args.push(MethodParameter { name: "InterruptModeration".to_string(), value: interrupt_moderation.into() });

        let result = self.invoke_method("WmiSetInterruptModeration", &args)?;
        let output_info = result.get_value("OutputInfo")?;
        Ok(result.return_value)

    }

}

